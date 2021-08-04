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

pub type EffectRackStateType<U, B> = Arc<RwLock<EffectRackState<U, B>>>;

struct EffectRack<U, B>
where
    B: AudioBackend + Sync + Send,
{
    state: EffectRackStateType<U, B>,
}

pub trait DynRecorder: Recorder + Sync + Send {}

#[derive(Debug)]
pub struct ConnectedUserState<U, R> {
    // todo: also allow sending errors?
    connection: mpsc::Sender<U>,
    recorder: Option<Arc<R>>,
}

pub type Msg = Result<Update, Status>;

impl<R> ConnectedUserState<Msg, R> {
    pub fn new(connection: mpsc::Sender<Msg>) -> Self {
        Self {
            connection,
            recorder: None,
        }
    }

    async fn start_analysis<T, B>(&mut self, audio_backend: &Arc<B>) -> Result<()>
    where
        B: AudioBackend + Sync + Send + 'static,
        T: Sample + Mel + Hz + Sync + std::fmt::Debug + Default + ScalarOperand + 'static,
    {
        let audio_backend = audio_backend.clone();

        // create a tokio runtime to perform async operations in threads
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        let (result_tx, result_rx) = std::sync::mpsc::channel();
        let (rec_tx, rec_rx) = std::sync::mpsc::channel();
        let stream_tx = self.connection.clone();
        let file_name = PathBuf::from(
            "/home/roman/dev/wasm-audio-effect-rack/experimental/audio-samples/roddy.wav",
        );
        let rec_file_name = file_name.clone();
        let recorder_thread = thread::spawn(move || {
            println!("starting the recording...");
            let rec = audio_backend.new_recorder().expect("create recorder");

            // todo: choose either file or stream based on the server config
            rec.stream_file(
                rec_file_name,
                move |samples: Result<Array2<T>>, sample_rate, nchannels| {
                    if let Err(err) = rec_tx.send((samples, sample_rate, nchannels)) {
                        panic!("{}", err);
                    }
                },
            );
            println!("playback is over");
        });

        // thread that collects and analyzes samples
        let analysis_thread = thread::spawn(move || {
            let (sample_rate, nchannels) =
                <B as AudioBackend>::Rec::get_file_info(file_name.clone()).unwrap();
            let analyzer_opts = SpectralAnalyzerOptions {
                nchannels: nchannels,
                sample_rate: sample_rate,
                fps: 60,
            };
            let mut analyzer = SpectralAnalyzer::<T>::new(analyzer_opts).unwrap();
            loop {
                match rec_rx.recv() {
                    Ok((Ok(samples), sample_rate, nchannels)) => {
                        println!("size of samples: {:?}", samples.shape());
                        result_tx.send(10);
                        // match analyzer.analyze_samples(samples) {
                        //     Ok(result) => println!("{:?}", result),
                        //     Err(err) => println!("{}", err),
                        // }
                    }
                    Ok((Err(err), _, _)) => {
                        println!("{}", err);
                        break;
                    }
                    Err(err) => {
                        println!("{}", err);
                        break;
                    }
                }
            }
        });

        // wait for analysis results and send them to the user
        let update_thread = thread::spawn(move || {
            loop {
                match result_rx.recv() {
                    Ok(i) => {
                        let heartbeat = Update {
                            update: Some(update::Update::Heartbeat(Heartbeat { seq: 0 })),
                        };
                        if let Err(err) = rt.block_on(stream_tx.send(Ok(heartbeat))) {
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
pub struct EffectRackState<U, B>
where
    B: AudioBackend + Sync + Send,
{
    pub audio_backend: Arc<B>,
    // mspc send channel for each user that is connected
    pub connected: HashMap<String, RwLock<ConnectedUserState<U, B::Rec>>>,
}

impl<B> EffectRackState<Msg, B>
where
    B: AudioBackend + Sync + Send,
{
    fn remove_user(&mut self, user_id: &String) {
        println!("removing {}", user_id);
        self.connected.remove(user_id);
    }

    async fn broadcast(&self, msg: Msg) {
        for (user_id, state) in &self.connected {
            let conn = &state.read().await.connection;
            // this would be a simple msg.clone() but Status has no Clone
            match msg {
                Ok(ref msg) => {
                    if let Err(err) = conn.send(Ok(msg.clone())).await {
                        println!("broadcast send error to {}, {:?}", user_id, err)
                    }
                }
                Err(ref err) => {
                    if let Err(err) = conn
                        .send(Err(Status::new(err.code(), err.to_string())))
                        .await
                    {
                        println!("broadcast send error to {}, {:?}", user_id, err)
                    }
                }
            }
        }
    }
}

#[derive()]
pub struct RemoteControllerService<U, B>
where
    B: AudioBackend + Sync + Send,
{
    pub state: EffectRackStateType<U, B>,
    pub shutdown_rx: watch::Receiver<bool>,
}

impl<B> RemoteControllerService<Msg, B>
where
    B: AudioBackend + Sync + Send,
{
    fn new_with_shutdown(
        state: EffectRackStateType<Msg, B>,
        shutdown_rx: watch::Receiver<bool>,
    ) -> Self {
        Self { state, shutdown_rx }
    }
}

#[tonic::async_trait]
impl<B> RemoteController for RemoteControllerService<Msg, B>
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
                            if let Err(err) = update_tx.send(update).await {
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

impl<B> EffectRack<Msg, B>
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

        let grpc_server = RemoteControllerServer::new(server);
        let grpc_server = tonic_web::config()
            // .allow_origins(vec!["localhost", "127.0.0.1"])
            .enable(grpc_server);

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

                // also wait for other threads and all
                running.await.ok();
                println!("exiting");
            }
        }
    }
    Ok(())
}
