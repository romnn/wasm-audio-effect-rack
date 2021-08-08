#[cfg(all(not(feature = "record"), any(feature = "portaudio", feature = "jack")))]
compile_error!("feature \"jack\" and feature \"portaudio\" cannot be enabled when feature \"record\" is disabled");

mod cli;
mod controller;
mod viewer;
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
use cli::{Commands, Opts, StartOpts};
use common::errors::FeatureDisabledError;
use futures::{Future, Stream};
use ndarray::prelude::*;
use ndarray::{
    concatenate, indices, Array, IntoDimension, Ix, NdIndex, RemoveAxis, ScalarOperand, Slice, Zip,
};
use num::traits::{Float, FloatConst, NumCast, Zero};
use proto::grpc::remote_controller_server::{RemoteController, RemoteControllerServer};
use proto::grpc::remote_viewer_server::{RemoteViewer, RemoteViewerServer};
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
use std::rc::Rc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::signal;
use tokio::sync::{mpsc, oneshot, watch, RwLock};
use tokio::time;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server as TonicServer, Code, Request, Response, Status};

pub type EffectRackStateType<U, B> = Arc<RwLock<EffectRackState<U, B>>>;
// pub type Msg = Result<Update, Status>;
// pub type Msg = Result<Update>;
pub type Msg = Update;
pub trait DynRecorder: Recorder + Sync + Send {}

struct EffectRack<U, B>
where
    B: AudioBackend + Sync + Send,
    U: Clone,
{
    state: EffectRackStateType<U, B>,
}

#[derive(Debug)]
pub struct ConnectedUserState<U, R> {
    connection: Arc<RwLock<mpsc::Sender<U>>>,
    recorder: Option<Arc<R>>,
    config: StartOpts,
    is_analyzing: bool,
}

#[derive(Debug)]
pub struct AudioAnalysis {
    recorder_thread: thread::JoinHandle<()>,
    is_running: bool,
}

impl AudioAnalysis {
    pub fn start() {}

    pub fn stop() {}

    pub fn remove_analyzer() {}
    pub fn add_analyzer() {}
}

impl<R> ConnectedUserState<Msg, R>
where
    R: Send + Sync,
{
    pub fn new(config: StartOpts, connection: mpsc::Sender<Msg>) -> Self {
        Self {
            config,
            connection: Arc::new(RwLock::new(connection)),
            recorder: None,
            is_analyzing: false,
        }
    }

    async fn start_analysis<T, B>(&mut self, audio_backend: &Arc<B>) -> Result<()>
    where
        B: AudioBackend + Sync + Send + 'static,
        T: Float
            + FloatConst
            + Mel
            + Hz
            + Sync
            + Send
            + std::fmt::Debug
            + Default
            + ScalarOperand
            + 'static,
    {
        self.is_analyzing = true;
        let audio_backend = audio_backend.clone();

        // create a tokio runtime to perform async operations in threads
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        let (result_tx, result_rx) = std::sync::mpsc::channel();
        let (rec_tx, rec_rx) = std::sync::mpsc::channel();
        // let rec_file_name = file_name.clone();
        let play_file = self.config.play_file.clone();
        let builder = thread::Builder::new();
        let recorder_thread = builder.name("recorder thread".to_string()).spawn(move || {
            println!("starting the recording...");
            let rec = audio_backend.new_recorder().expect("create recorder");

            // if let Some(file) = self.state.read().await.config.play_file {
            if let Some(file) = play_file {
                if let Err(err) = rec.stream_file(
                    PathBuf::from(file),
                    move |samples: Result<Array2<T>>, sample_rate, nchannels| {
                        if let Err(err) = rec_tx.send((samples, sample_rate, nchannels)) {
                            panic!("{}", err);
                        }
                    },
                ) {
                    eprintln!("failed to stream input: {}", err);
                };
            } else {
                if let Err(err) = rec.stream_input(
                    true,
                    move |samples: Result<Array2<T>>, sample_rate, nchannels| {
                        if let Err(err) = rec_tx.send((samples, sample_rate, nchannels)) {
                            panic!("{}", err);
                        }
                    },
                ) {
                    eprintln!("failed to stream input: {}", err);
                }
            }
            println!("playback is over");
        })?;

        // thread that collects and analyzes samples
        let builder = thread::Builder::new();
        let analysis_thread = builder.name("analysis thread".to_string()).spawn(move || {
            // let (sample_rate, nchannels) = match self.config.play_file {
            //      Some(file) =>
            //         <B as AudioBackend>::Rec::get_file_info(PathBuf::from(file).clone()).unwrap();
            //      None => {
            //         let input_config = self.input_device.default_input_config()?;
            //      }
            // };
            let buffer_window_size = 2048;
            let analyzer_opts = SpectralAnalyzerOptions {
                window_size: buffer_window_size,
                mel_bands: 24,
                // nchannels: nchannels,
                // sample_rate: sample_rate,
                fps: 60,
                ..SpectralAnalyzerOptions::default()
            };
            let mut analyzer = SpectralAnalyzer::<T>::new(analyzer_opts).unwrap();
            let mut buffer = Array2::<T>::zeros((0, 2));
            loop {
                match rec_rx.recv() {
                    Ok((Ok(samples), sample_rate, nchannels)) => {
                        // println!("new samples: {:?}", samples.shape());
                        analyzer.options.nchannels = nchannels;
                        analyzer.options.sample_rate = sample_rate;
                        if let Err(err) = buffer.append(Axis(0), samples.view()) {
                            eprintln!("failed to extend buffer: {}", err);
                        }
                        // println!("size of buffer: {:?}", buffer.shape());
                        // todo: maybe measure the processing time here and try to keep the real time
                        let buffer_size = buffer.len_of(Axis(0));
                        if buffer_size > NumCast::from(sample_rate * 1).unwrap() {
                            panic!("more than one second in the buffer");
                        }

                        let ready_buffers = buffer_size / buffer_window_size;
                        let mut processed = 0;

                        // process the chunks
                        for i in (0..ready_buffers) {
                            let start = i * buffer_window_size;
                            let end = (i + 1) * buffer_window_size;
                            // println!("analyzing from {} to {}", start, end);
                            let chunk = buffer
                                .slice_axis(Axis(0), Slice::from(start..end))
                                .to_owned();
                            if let Err(err) = result_tx.send(analyzer.analyze_samples(chunk)) {
                                eprintln!("failed to send result: {}", err);
                            }
                            processed += 1;
                        }
                        buffer.slice_axis_inplace(
                            Axis(0),
                            Slice::from((processed * buffer_window_size)..),
                        );
                    }
                    Ok((Err(err), _, _)) => {
                        println!("error while recording samples: {}", err);
                    }
                    Err(err) => {
                        // println!("failed to receive new samples: {}", err);
                    }
                }
            }
        })?;

        // wait for analysis results and send them to the user
        let stream_tx = self.connection.clone();
        let builder = thread::Builder::new();
        let update_thread = builder.name("upate thread".to_string()).spawn(move || {
            let mut seq_num = 0;
            loop {
                match result_rx.recv() {
                    Ok(Err(analysis_err)) => {
                        println!("{}", analysis_err);
                    }
                    Ok(Ok(mut analysis_result)) => {
                        analysis_result.seq_num = seq_num;
                        let analysis_result_update = Update {
                            update: Some(update::Update::AudioAnalysisResult(analysis_result)),
                        };
                        // let stream_tx = self.connection;
                        match rt.block_on(async {
                            let rx = stream_tx.read().await;
                            // rx.send(Ok(analysis_result_update)).await
                            rx.send(analysis_result_update).await
                        }) {
                            Err(err) => println!("{}", err),
                            Ok(_) => seq_num = seq_num + 1,
                        };
                    }
                    Err(recv_err) => {
                        println!("{}", recv_err);
                        // break;
                    }
                }
            }
        });

        let analysis = AudioAnalysis {
            recorder_thread,
            is_running: true,
        };
        Ok(())
    }
}

#[derive(Debug)]
pub struct EffectRackState<U, B>
where
    B: AudioBackend + Sync + Send,
    U: Clone,
{
    pub audio_backend: Arc<B>,
    pub config: StartOpts,
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

    // async fn broadcast(&self, msg: Msg) {
    //     for (user_id, state) in &self.connected {
    //         let conn = &state.read().await.connection;
    //         let conn = conn.read().await;
    //         if let Err(err) = conn.send(msg.clone()).await {
    //             println!("broadcast send error to {}, {:?}", user_id, err)
    //         }

    //         // this would be a simple msg.clone() but Status has no Clone
    //         // match msg {
    //         //     Ok(ref msg) => {
    //         //         if let Err(err) = conn.send(Ok(msg.clone())).await {
    //         //             println!("broadcast send error to {}, {:?}", user_id, err)
    //         //         }
    //         //     }
    //         //     Err(ref err) => {
    //         //         if let Err(err) = conn
    //         //             .send(Err(Status::new(err.code(), err.to_string())))
    //         //             .await
    //         //         {
    //         //             println!("broadcast send error to {}, {:?}", user_id, err)
    //         //         }
    //         //     }
    //         // }
    //     }
    // }
}

#[derive(Clone)]
pub struct RemoteService<U, B>
where
    B: AudioBackend + Sync + Send + Clone,
    U: Clone,
{
    pub state: EffectRackStateType<U, B>,
    pub shutdown_rx: watch::Receiver<bool>,
}

impl<B> RemoteService<Msg, B>
where
    B: AudioBackend + Clone + Sync + Send,
{
    fn new_with_shutdown(
        state: EffectRackStateType<Msg, B>,
        shutdown_rx: watch::Receiver<bool>,
    ) -> Self {
        Self { state, shutdown_rx }
    }

    fn extract_user_token<T>(request: Request<T>) -> Result<String, Status> {
        request
            .metadata()
            .get("user-token")
            .and_then(|token| token.to_str().ok())
            .map(|token| token.to_string())
            .ok_or(Status::new(Code::InvalidArgument, "missing user token"))
    }
}

impl<B> EffectRack<Msg, B>
where
    B: AudioBackend + Sync + Clone + Send + 'static,
{
    // async fn shutdown(&self) -> Result<()> {
    //     println!("todo: set some internal oneshot channel or so");
    //     Ok(())
    // }

    async fn start(&self, shutdown_rx: watch::Receiver<bool>) -> Result<()> {
        let addr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
        let addr = SocketAddr::new(addr, self.state.read().await.config.port);

        println!("listening on {}", addr);

        let remote_server =
            RemoteService::new_with_shutdown(self.state.clone(), shutdown_rx.clone());
        // let remote_controller_server =
        //     RemoteService::new_with_shutdown(self.state.clone(), shutdown_rx.clone());
        // let remote_viewer_server =
        //     RemoteService::new_with_shutdown(self.state.clone(), shutdown_rx.clone());

        // let remote_controller_grpc_server = RemoteControllerServer::new(remote_controller_server);
        let remote_controller_grpc_server = RemoteControllerServer::new(remote_server.clone());
        let remote_controller_grpc_server = tonic_web::config()
            // .allow_origins(vec!["localhost", "127.0.0.1"])
            .enable(remote_controller_grpc_server);

        // let remote_grpc_server = tonic_web::config()
        //     // .allow_origins(vec!["localhost", "127.0.0.1"])
        //     .enable(remote_server);

        // let remote_viewer_grpc_server = RemoteViewerServer::new(remote_viewer_server);
        let remote_viewer_grpc_server = RemoteViewerServer::new(remote_server.clone());
        let remote_viewer_grpc_server = tonic_web::config()
            // .allow_origins(vec!["localhost", "127.0.0.1"])
            .enable(remote_viewer_grpc_server);

        TonicServer::builder()
            .accept_http1(true)
            // .add_service(remote_grpc_server)
            .add_service(remote_controller_grpc_server)
            .add_service(remote_viewer_grpc_server)
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
        input_device: opts.input_device,
        output_device: opts.output_device,
        latency: NumCast::from(opts.latency).unwrap(),
    };
    let audio_backend = Arc::new(CpalAudioBackend::new(audio_config));

    cfg_if::cfg_if! {
        if #[cfg(feature = "portaudio")] {
            if opts.use_portaudio_backend {
                let audio_backend = Arc::new(PortaudioAudioBackend::new(audio_config));
            };
        }
    };

    if let Some(subcommand) = opts.commands {
        match subcommand {
            #[cfg(feature = "record")]
            Commands::Query(cfg) => {
                // println!("query:  {:?}", cfg);
                // if !cfg!(feature = "record") {
                //     return Err(FeatureDisabledError::new("record is not available").into());
                // }
                // rack.state.read().await.audio_backend.query()?;
                audio_backend.query()?;
            }
            Commands::Start(cfg) => {
                // println!("start:  {:?}", cfg);

                let state = Arc::new(RwLock::new(EffectRackState {
                    connected: HashMap::new(),
                    config: cfg,
                    audio_backend,
                }));
                let rack = Arc::new(EffectRack {
                    // todo: give them instances
                    // analyzer: Arc::new(SpectralAnalyzer::new(SpectralAnalyzerOptions::default())),
                    state,
                });

                // let rack = rack.clone();
                let running = tokio::task::spawn(async move {
                    rack.start(shutdown_rx).await.expect("failed to run rack");
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
