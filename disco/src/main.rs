use anyhow::Result;
use clap::Clap;
use disco::cli::{Commands, Config, Opts};
use disco::{DiscoServer, Sample};
#[cfg(feature = "portaudio")]
use recorder::portaudio::PortaudioAudioInput;

#[cfg(feature = "record")]
use recorder::{cpal::CpalAudioBackend, cpal::CpalAudioInput, AudioInput, AudioInputConfig};
use std::sync::Arc;
use tokio::signal;
use tokio::sync::{watch};

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

                let _running = tokio::task::spawn(async move {
                    disco.serve().await.expect("failed to run disco");
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
