#[cfg(all(not(feature = "record"), any(feature = "portaudio", feature = "jack")))]
compile_error!("feature \"jack\" and feature \"portaudio\" cannot be enabled when feature \"record\" is disabled");

mod cli;
pub extern crate common;
pub extern crate proto;

#[cfg(feature = "p2p")]
mod p2p;
#[cfg(feature = "analyze")]
pub extern crate analyzer;
#[cfg(feature = "record")]
pub extern crate recorder;

use analyzer::spectral::{SpectralAnalyzer, SpectralAnalyzerOptions};
#[cfg(feature = "analyze")]
use analyzer::{mel::Hz, mel::Mel, Analyzer};
use anyhow::Result;
use clap::Clap;
use cli::{Commands, Opts};
use common::errors::FeatureDisabledError;
use futures::{Future, Stream};
use ndarray::prelude::*;
use ndarray::{
    concatenate, indices, Array, IntoDimension, Ix, NdIndex, RemoveAxis, ScalarOperand, Slice, Zip,
};
use num::traits::{Float, FloatConst, Zero};
use proto::grpc::remote_controller_server::{RemoteController, RemoteControllerServer};
use proto::grpc::update;
use proto::grpc::{
    Empty, Heartbeat, StartAnalysisRequest, SubscriptionRequest, UnsubscriptionRequest, Update,
    UpdateSubscriptionRequest,
};
#[cfg(feature = "record")]
use recorder::{
    backend::cpal::CpalAudioBackend, cpal::CpalRecorder, AudioBackend, AudioBackendConfig,
    Recorder, Sample,
};
#[cfg(feature = "portaudio")]
use recorder::{backend::portaudio::PortaudioAudioBackend, portaudio::PortaudioRecorder};
use std::collections::HashMap;
use std::error::Error;
use std::marker::Send;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::signal;
use tokio::sync::{mpsc, oneshot, watch, RwLock};
use tokio::time;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server as TonicServer, Code, Request, Response, Status};

pub type EffectRackStateType<B> = Arc<RwLock<EffectRackState<B>>>;

struct EffectRack<B>
where
    B: AudioBackend + Sync + Send, //+ 'static, // R: Recorder,
{
    // audio_backend: Arc<dyn Recorder + Sync + Send>,
    state: EffectRackStateType<B>,
}

pub trait DynRecorder: Recorder + Sync + Send {}

#[derive(Debug)]
pub struct ConnectedUserState<R> {
    // todo: also allow sending errors?
    connection: mpsc::Sender<Update>,
    // recorder: Option<Arc<dyn DynRecorder>>,
    // recorder: Option<Arc<dyn DynRecorder>>,
    recorder: Option<Arc<R>>,
    // analyzer: Arc<dyn Analyzer<T> + Sync + Send>,
}

impl<R> ConnectedUserState<R>
// where
// R: Sync,
{
    pub fn new(connection: mpsc::Sender<Update>) -> Self {
        Self {
            connection,
            recorder: None,
        }
    }

    // pub fn start_analysis<T>(&mut self, recorder: R) -> Result<()>
    // pub fn start_analysis<T, B>(&mut self, audio_backend: &Arc<B>) -> Result<()>
    // todo: make this async, add another channel and write out the updates for this analysis in a
    // spawned tokio thread
    async fn start_analysis<T, B>(&mut self, audio_backend: &Arc<B>) -> Result<()>
    where
        // R: DynRecorder + 'static,
        // R: DynRecorder + 'static,
        // B: AudioBackend + Sync + Send + 'static,
        B: AudioBackend + Sync + Send + 'static,
        T: Sample + Mel + Hz + Sync + std::fmt::Debug + Default + ScalarOperand + 'static,
    {
        // self.recorder = Some(Arc::new(recorder));
        // let recorder = self.recorder.clone();
        let audio_backend = audio_backend.clone();

        // create a tokio runtime to perform async operations in threads
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        // let (sender, receiver) = std::sync::mpsc::sync_channel(100);
        // sender.send((10,10));
        // let sender2 = sender.clone();
        let (result_tx, result_rx) = std::sync::mpsc::channel();
        let (rec_tx, rec_rx) = std::sync::mpsc::channel();
        let stream_tx = self.connection.clone();
        // let analysis_thread = thread::spawn(move || {
        // let (sender, receiver) = std::sync::mpsc::sync_channel(100);
        // let sender2 = sender.clone();

        // let sender2 = sender.clone();
        let file_name = PathBuf::from(
            "/home/roman/dev/wasm-audio-effect-rack/experimental/audio-samples/roddy.wav",
        );
        let rec_file_name = file_name.clone();
        let recorder_thread = thread::spawn(move || {
            println!("starting the recording...");
            let rec = audio_backend.new_recorder().expect("create recorder");
            // .map_err(|_| Status::new(Code::Internal, "failed to create a recorder"))?;

            // if let Some(recorder) = recorder {
            // for i in 0..1000 {
            //     if let Err(err) = sender.send((10, 10)) {
            //         panic!("{}", err)
            //     }
            // }
            // todo: choose either file or stream based on the server config
            // let sender3 = sender2.clone();
            rec.stream_file(
                rec_file_name,
                move |samples: Result<Array2<T>>, sample_rate, nchannels| {
                    // let _ = sender.send((samples, sample_rate, nchannels));
                    // let _ = sender3.send((sample_rate, nchannels));
                    // sender.send((10,10));
                    // println!("sending to analyser");
                    if let Err(err) = rec_tx.send((samples, sample_rate, nchannels)) {
                        // println!("can not send channel");
                        // println!("{}", err);
                        panic!("{}", err);
                    }
                    // sender2.send((10,10)).expect("can send upates");
                },
            );
            println!("playback is over. exiting thread");
            // }
        });

        let analysis_thread = thread::spawn(move || {
            println!("starting the analysis...");
            let (sample_rate, nchannels) =
                <B as AudioBackend>::Rec::get_file_info(file_name.clone()).unwrap();
            let analyzer_opts = SpectralAnalyzerOptions {
                nchannels: nchannels,
                sample_rate: sample_rate,
                fps: 60,
            };
            let mut analyzer = SpectralAnalyzer::<T>::new(analyzer_opts).unwrap();
            loop {
                // match receiver.try_recv() {
                match rec_rx.recv() {
                    Ok((samples, sample_rate, nchannels)) => {
                        match samples {
                            Ok(samples) => {
                                // analyze them
                                println!("size of samples: {:?}", samples.shape());
                                result_tx.send(10);
                                // let heartbeat = Update {
                                //     update: Some(update::Update::Heartbeat(Heartbeat { seq: 0 })),
                                // };
                                // if let Err(err) = rt.block_on(stream_tx.send(heartbeat)) {
                                //     println!("{}", err);
                                // };
                                // match analyzer.analyze_samples(samples) {
                                //     Ok(result) => println!("{:?}", result),
                                //     Err(err) => println!("{}", err),
                                // }
                            }
                            Err(err) => {}
                        }
                    }
                    Err(err) => {
                        println!("{}", err);
                        break;
                    }
                }
            }
            // while let Ok((sample_rate, nchannels)) = receiver.try_recv() { }
        });

        // let analysis_thread = thread::spawn(move || {
        //     // println!("{:?}", samples);
        //     // while let Ok((samples, sample_rate, nchannels)) = receiver.try_recv() {
        //             });

        // wait for analysis results and send them to the user
        // tokio::spawn(async move {
        let update_thread = thread::spawn(move || {
            loop {
                match result_rx.recv() {
                    Ok(i) => {
                        let heartbeat = Update {
                            update: Some(update::Update::Heartbeat(Heartbeat { seq: 0 })),
                        };
                        if let Err(err) = rt.block_on(stream_tx.send(heartbeat)) {
                            println!("{}", err);
                        }

                        // match samples {
                        //     Ok(samples) => {
                        //                                     }
                        //     Err(err) => {}
                        // }
                    }
                    Err(err) => {
                        println!("{}", err);
                        break;
                    }
                }
            }
        });

        Ok(())
    }
}

#[derive(Debug)]
pub struct EffectRackState<B>
where
    B: AudioBackend + Sync + Send, // + 'static,
{
    pub audio_backend: Arc<B>,
    // mspc send channel for each user that is connected
    pub connected: HashMap<String, RwLock<ConnectedUserState<B::Rec>>>,
}

impl<B> EffectRackState<B>
where
    B: AudioBackend + Sync + Send, // + 'static,
{
    // fn new() -> Self {
    //     Self {
    //     }
    // }

    fn remove_user(&mut self, user_id: &String) {
        println!("removing {}", user_id);
        self.connected.remove(user_id);
    }

    async fn broadcast(&self, msg: Update) {
        for (user_id, state) in &self.connected {
            if let Err(err) = state.read().await.connection.send(msg.clone()).await {
                println!("broadcast send error to {}, {:?}", user_id, err)
            }
        }
    }
}

#[derive()]
pub struct RemoteControllerService<B>
where
    B: AudioBackend + Sync + Send, // + 'static,
{
    pub state: EffectRackStateType<B>,
    pub shutdown_rx: watch::Receiver<bool>,
}

impl<B> RemoteControllerService<B>
where
    B: AudioBackend + Sync + Send, // + 'static,
{
    fn new_with_shutdown(
        state: EffectRackStateType<B>,
        shutdown_rx: watch::Receiver<bool>,
    ) -> Self {
        Self { state, shutdown_rx }
    }
}

#[tonic::async_trait]
impl<B> RemoteController for RemoteControllerService<B>
where
    B: AudioBackend + Sync + Send + 'static,
{
    type SubscribeStream =
        Pin<Box<dyn Stream<Item = Result<Update, Status>> + Send + Sync + 'static>>;

    async fn update_subscription(
        &self,
        request: Request<UpdateSubscriptionRequest>,
    ) -> Result<Response<Empty>, Status> {
        Ok(Response::new(Empty {}))
    }

    async fn unsubscribe(
        &self,
        request: Request<UnsubscriptionRequest>,
    ) -> Result<Response<Empty>, Status> {
        Ok(Response::new(Empty {}))
    }

    async fn start_analysis(
        &self,
        request: Request<StartAnalysisRequest>,
    ) -> Result<Response<Empty>, Status> {
        let audio_backend = &self.state.read().await.audio_backend;
        let user_id = request.into_inner().user_id;
        println!("wants to start: {}", user_id);
        if let Some(user) = self.state.read().await.connected.get(&user_id) {
            // this will not block
            user.write()
                .await
                .start_analysis::<f32, _>(&audio_backend)
                .await;
        };
        Ok(Response::new(Empty {}))
    }

    async fn subscribe(
        &self,
        request: Request<SubscriptionRequest>,
    ) -> Result<Response<Self::SubscribeStream>, Status> {
        let user_id = request.into_inner().user_id;
        if user_id.len() < 1 {
            return Err(Status::new(
                Code::InvalidArgument,
                "will not connect with user without user_id",
            ));
        }

        let (stream_tx, stream_rx) = mpsc::channel(1);
        let pinned_stream = Box::pin(ReceiverStream::new(stream_rx));
        let (tx, mut rx) = mpsc::channel(1);

        let connected = &mut self.state.write().await.connected;
        match connected.get_mut(&user_id) {
            Some(existing) => {
                // reconnect
                println!("({} reconnected)", user_id);
                existing.write().await.connection = tx;
            }
            None => {
                let user_state = ConnectedUserState::new(tx);
                connected.insert(user_id.clone(), RwLock::new(user_state));
                println!("{} connected", user_id);
            }
        }

        let state = self.state.clone();
        let update_tx = stream_tx.clone();
        let update_user_id = user_id.clone();
        let mut shutdown_rx = self.shutdown_rx.clone();
        tokio::spawn(async move {
            // send ack
            let _ = stream_tx.send(Ok(Update::default())).await;
            // stream_tx
            //     .send(Err(Status::new(Code::Ok, "connected")))
            //     .await;

            // wait for either shutdown, heartbeat or an update to send out
            let mut seq = 0u64;
            // todo: counter for failed send events
            let heartbeat_interval = time::sleep(Duration::from_millis(5 * 1000));
            let mut heartbeat_timer = Box::pin(heartbeat_interval);
            let mut shutdown_signal = Box::pin(shutdown_rx.changed());
            loop {
                tokio::select! {
                    _ = &mut shutdown_signal => {
                        println!("shutdown from open connection");
                        break;
                    }
                    received = rx.recv() => {
                        if let Some(update) = received {
                            if let Err(err) = update_tx.send(Ok(update)).await {
                                // If sending failed, then remove the user from shared data
                                state.write().await.remove_user(&update_user_id);
                                break;
                            }
                        }
                    }
                    _ = &mut heartbeat_timer => {
                        // reset the heartbeat
                        let heartbeat_interval = time::sleep(Duration::from_millis(5 * 1000));
                        heartbeat_timer = Box::pin(heartbeat_interval);

                        let heartbeat = Update {
                            update: Some(update::Update::Heartbeat(Heartbeat { seq })),
                        };
                        if let Err(_) = update_tx.send(Ok(heartbeat)).await {
                            state.write().await.remove_user(&update_user_id);
                            break;
                        };
                        seq = seq + 1;
                    }
                }
            }
        });

        Ok(Response::new(pinned_stream))
    }
}

impl<B> EffectRack<B>
where
    B: AudioBackend + Sync + Send + 'static,
{
    async fn shutdown(&self) -> Result<()> {
        println!("todo: set some internal oneshot channel or so");
        Ok(())
    }

    async fn start(&self, addr: SocketAddr, shutdown_rx: watch::Receiver<bool>) -> Result<()> {
        println!("listening on {}", addr);

        let server =
            RemoteControllerService::new_with_shutdown(self.state.clone(), shutdown_rx.clone());

        // let analyzer = self.analyzer.clone();
        // let analyzer_thread = thread::spawn(move || {
        //     // println!("{:?}", self.recorder);
        //     // println!("{:?}", analyzer);
        //     // for i in 1..10 {
        //     // server.broadcast(i).await;
        //     // println!("hi number {} from the spawned thread!", i);
        //     // thread::sleep(Duration::from_millis(1));
        //     // }
        // });

        // let recorder = self.recorder.clone();
        // for t in thrds {
        // t.join();
        // }
        // analyzer_thread.join();

        let grpc_server = RemoteControllerServer::new(server);
        let grpc_server = tonic_web::config()
            // .allow_origins(vec!["localhost", "127.0.0.1"])
            .enable(grpc_server);

        // let state = self.state.clone();
        // tokio::spawn(async move {
        //     for x in 0..2 {
        //         for x in 0..5 {
        //             time::sleep(Duration::from_millis(1 * 1000)).await;
        //             state.read().await.broadcast(Update::default()).await;
        //         }
        //         time::sleep(Duration::from_millis(5 * 1000)).await;
        //     }
        //     println!("done pushing updates to");
        // });

        let tserver = TonicServer::builder()
            .accept_http1(true)
            .add_service(grpc_server)
            .serve_with_shutdown(addr, async {
                shutdown_rx
                    .clone()
                    .changed()
                    .await
                    .expect("failed to shutdown");
            })
            .await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    // let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    let audio_config = AudioBackendConfig {
        #[cfg(use_jack)]
        jack: opts.use_jack_backend,
        device: opts.device,
    };
    let audio_backend = Arc::new(CpalAudioBackend::new(audio_config));

    cfg_if::cfg_if! {
        if #[cfg(feature = "portaudio")] {
            if opts.use_portaudio_backend {
                let audio_backend = Arc::new(PortaudioAudioBackend::new(audio_config));
            };
        }
    };

    let state = Arc::new(RwLock::new(EffectRackState {
        connected: HashMap::new(),
        audio_backend,
    }));
    let rack = Arc::new(EffectRack {
        // todo: give them instances
        // analyzer: Arc::new(SpectralAnalyzer::new(SpectralAnalyzerOptions::default())),
        state,
    });

    if let Some(subcommand) = opts.commands {
        match subcommand {
            #[cfg(feature = "record")]
            Commands::Query(cfg) => {
                // println!("query:  {:?}", cfg);
                // if !cfg!(feature = "record") {
                //     return Err(FeatureDisabledError::new("record is not available").into());
                // }
                rack.state.read().await.audio_backend.query()?;
            }
            Commands::Start(cfg) => {
                // println!("start:  {:?}", cfg);
                let addr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
                let addr = SocketAddr::new(addr, cfg.port);

                let rack = rack.clone();
                let running = tokio::task::spawn(async move {
                    rack.start(addr, shutdown_rx)
                        .await
                        .expect("failed to run rack");
                });

                signal::ctrl_c().await.ok().map(|_| {
                    println!("received shutdown");
                    shutdown_tx.send(true).expect("send shutdown signal");
                });

                // this is not graceful at all
                running.await.ok();
                println!("exiting");
            }
        }
    }
    Ok(())
}
