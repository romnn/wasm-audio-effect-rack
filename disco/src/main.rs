use anyhow::Result;
use clap::Parser;
use disco::cli::{Commands, Config, Opts};
use disco::{DiscoServer, Sample, INSTANCE_ID_KEY, SESSION_TOKEN_KEY};
#[cfg(feature = "portaudio")]
use recorder::portaudio::PortaudioAudioInput;
use thirtyfour::prelude::*;
use thirtyfour::OptionRect;

use proto::grpc::remote_controller_client::RemoteControllerClient;
#[cfg(feature = "record")]
use recorder::{cpal::CpalAudioBackend, cpal::CpalAudioInput, AudioInput, AudioInputConfig};
use std::sync::Arc;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use tokio::signal;
use tokio::sync::{watch, Mutex};
use tonic::metadata::MetadataValue;
use tonic::Request;
mod retry;

const SPLASH_LOGO: &str = "
   __  ___  __   _   _  
   ) )  )  (_ ` / ` / ) 
  /_/ _(_ .__) (_. (_/  
";

#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    println!("{}", SPLASH_LOGO);

    if let Some(ref subcommand) = opts.commands {
        match subcommand {
            #[cfg(feature = "record")]
            Commands::Query(cfg) => {
                let audio_backend =
                    CpalAudioInput::<Sample>::new(AudioInputConfig::from(opts.clone()))?;
                cfg_if::cfg_if! {
                    if #[cfg(feature = "portaudio")] {
                        if opts.use_portaudio{
                            let audio_backend  = PortaudioAudioInput::new(opts.into())?;
                        };
                    }
                };
                match &cfg.device {
                    Some(_device) => {
                        // audio_backend.query_device(device)?;
                    }
                    None => {
                        audio_backend.query()?;
                    }
                }
            }
            Commands::Start(cfg) => {
                let config = Config {
                    run: cfg.clone(),
                    default: opts.clone(),
                };

                let disco = Arc::new(DiscoServer::new_with_shutdown(config, shutdown_rx));

                let disco_clone = disco.clone();
                let _running = tokio::task::spawn(async move {
                    disco_clone.serve().await.expect("failed to run disco");
                });

                // spawn the webdriver thread
                let _screenshot = tokio::task::spawn(async move {
                    take_screenshot(&disco).await.expect("take screenshot");
                });

                signal::ctrl_c().await.ok().map(|_| {
                    println!("received shutdown");
                    shutdown_tx.send(true).expect("send shutdown signal");
                });

                // todo: also wait for other threads and all
                // _running.await.ok();
                println!("exiting");
            }
        }
    }
    Ok(())
}

async fn take_screenshot<VU: Clone, CU: Clone>(disco: &Arc<DiscoServer<VU, CU>>) -> Result<()> {
    let width = 4096;
    let height = 4096;
    let mut caps = DesiredCapabilities::chrome();
    caps.add_chrome_arg("--headless")?;
    caps.add_chrome_arg("--enable-automation")?;
    caps.add_chrome_arg(&format!("--window-size={},{}", width, height))?;
    caps.add_chrome_arg("--no-sandbox")?;
    caps.add_chrome_arg("--disable-infobars")?;
    caps.add_chrome_arg("--disable-browser-side-navigation")?;
    caps.add_chrome_arg("--disable-gpu")?;
    caps.add_chrome_arg("--disable-features=VizDisplayCompositor")?;
    caps.add_chrome_arg("--disable-dev-shm-usage")?;
    caps.add_chrome_arg("--disable-extensions")?;
    let driver = WebDriver::new("http://localhost:4444", &caps).await?;
    // let window_size = OptionRect::new().with_size(1920, 1080);
    // driver.set_window_rect(window_size).await?;

    retry::retry(retry::Fixed::from_secs(5).take(10), |_| async {
        match driver.get("http://host.docker.internal:8888").await {
            Ok(driver) => retry::OperationResult::Ok(driver),
            Err(err) => {
                println!("failed to get page: {:?}", err);
                retry::OperationResult::Retry(err)
            }
        }
    })
    .await
    .map_err(|err| err.error)?;

    let mut client = retry::retry(retry::Fixed::from_secs(5).take(10), |_| async {
        match RemoteControllerClient::connect("http://0.0.0.0:9000").await {
            Ok(client) => retry::OperationResult::Ok(client),
            Err(err) => {
                println!("failed to connect to grpc: {:?}", err);
                retry::OperationResult::Retry(err)
            }
        }
    })
    .await
    .map_err(|err| err.error)?;

    let viewer: Arc<Mutex<Option<proto::grpc::InstanceId>>> = Arc::new(Mutex::new(None));

    let mut stream = client
        .subscribe(Request::new(proto::grpc::ControllerSubscribeRequest {
            instance: None,
        }))
        .await?
        .into_inner();

    // let ready = Condvar::new();
    // connect to a new viewer with a new session
    // let mut client_clone = client.clone();
    let assignment = tokio::task::spawn(async move {
        while let Some(msg) = stream.message().await.unwrap() {
            println!("message {:?}", msg);
            match msg.update {
                Some(proto::grpc::controller_update::Update::Assignment(
                        assignment
                    // proto::grpc::Assignment {
                    //     session_token,
                    //     instance_id,
                    // },
                )) => {
                    return Some(assignment);
                    // let mut viewer_lock = viewer.lock().await;
                    // println!("assigned instance id: {:?}", instance_id);
                    // *viewer_lock = instance_id;
                    // ready.notify_one();
                }
                _ => {}
            };
        }
        None
    });
    let assignment = assignment.await.unwrap().unwrap();
    println!("ready");
    // ready.wait(started).unwrap();

    // println!("response: {:?}", response);

    // configure session to play a nice tune
    let mut request = Request::new(proto::grpc::AddAudioInputStreamRequest {
        input: Some(proto::grpc::add_audio_input_stream_request::Input::File(
            proto::grpc::FileInputStreamRequest {
                file_path: "/Users/roman/Desktop/adriana_selection/SOPHIE - OOH.mp3".to_string(),
                looped: true,
            },
        )),
    });
    let metadata = request.metadata_mut();
    if let Some(proto::grpc::SessionToken { ref token }) = assignment.session_token {
        metadata.insert(SESSION_TOKEN_KEY, MetadataValue::from_str(&token).unwrap());
    }
    if let Some(proto::grpc::InstanceId { ref id }) = assignment.instance_id {
        metadata.insert(INSTANCE_ID_KEY, MetadataValue::from_str(&id).unwrap());
    }

    let response = client.add_audio_input_stream(request).await?;
    println!("response: {:?}", response);

    // start recording
    let mut request = Request::new(proto::grpc::StartRecordingRequest {});
    let metadata = request.metadata_mut();
    if let Some(proto::grpc::SessionToken { ref token }) = assignment.session_token {
        metadata.insert(SESSION_TOKEN_KEY, MetadataValue::from_str(&token).unwrap());
    }
    if let Some(proto::grpc::InstanceId { ref id }) = assignment.instance_id {
        metadata.insert(INSTANCE_ID_KEY, MetadataValue::from_str(&id).unwrap());
    }
    let response = client.start_recording(request).await?;
    println!("response: {:?}", response);

    // sleep some time
    // stop recording
    // finalize recording

    // take screenshots in a loop
    let mut frame = 0;
    let max_frames = 20;
    loop {
        if frame > max_frames {
            break;
        }
        // render frame
        let ret = driver
            .execute_script(
                r#"
            window.recReceiveUpdate();
            window.animateFrame();
            "#,
            )
            .await?;

        // let elem = document.getElementById("recReceiveUpdate");
        // elem.click();
        // elem = document.getElementById("recAnimateFrame");
        // elem.click();

        let elem_out = ret.get_element()?;
        println!("elem out: {}", elem_out);

        let image = driver.screenshot_as_png().await?;
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open("/Users/roman/Desktop/seleniumtest.png")
            .await?;

        file.write_all(&image).await;
        frame += 1;
    }
    driver.quit().await?;

    Ok(())
}
