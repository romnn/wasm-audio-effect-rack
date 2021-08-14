#[cfg(all(not(feature = "record"), any(feature = "portaudio", feature = "jack")))]
compile_error!("feature \"jack\" and feature \"portaudio\" cannot be enabled when feature \"record\" is disabled");

mod analyzer;
mod cli;
mod controller;
mod session;
mod viewer;
pub extern crate hardware;
pub extern crate common;
pub extern crate proto;

#[cfg(feature = "p2p")]
mod p2p;
#[cfg(feature = "analyze")]
pub extern crate analysis;
#[cfg(feature = "record")]
pub extern crate recorder;

use analysis::spectral::{SpectralAnalyzer, SpectralAnalyzerOptions};
#[cfg(feature = "analyze")]
use analysis::{mel::Hz, mel::Mel, Analyzer};
use anyhow::Result;
use clap::Clap;
use cli::{Commands, Config, Opts, StartOpts};
use common::errors::FeatureDisabledError;
use futures::{Future, Stream};
use nanoid::nanoid;
use ndarray::prelude::*;
use ndarray::{
    concatenate, indices, Array, IntoDimension, Ix, NdIndex, RemoveAxis, ScalarOperand, Slice, Zip,
};
use num::traits::{Float, FloatConst, NumCast, Zero};
use proto::grpc::remote_controller_server::{RemoteController, RemoteControllerServer};
use proto::grpc::remote_viewer_server::{RemoteViewer, RemoteViewerServer};
use proto::grpc::{ControllerUpdate, Empty, Heartbeat, StartAnalysisRequest, ViewerUpdate};
#[cfg(feature = "portaudio")]
use recorder::portaudio::PortaudioAudioInput;

#[cfg(feature = "record")]
use recorder::{cpal::CpalAudioBackend, cpal::CpalAudioInput, AudioInputConfig, AudioInput};
use session::Session;
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

// pub type DeprEffectRackStateType<VU, CU, B> = Arc<RwLock<DeprEffectRackState<VU, CU, B>>>;
// pub type DeprEffectRackStateType<VU, CU> = Arc<RwLock<DeprEffectRackState<VU, CU>>>;
//
// pub type DeprEffectRackStateType<VU, CU> = Arc<DeprEffectRackState<VU, CU>>;
// pub type Msg = Result<Update, Status>;
// pub type Msg = Result<Update>;
const INSTANCE_ID_KEY: &str = "instance-id";
const SESSION_TOKEN_KEY: &str = "session-token";

pub type MyCustomError = String;
// pub type ViewerUpdateMsg = Result<ViewerUpdate, MyCustomError>;
// pub type ViewerUpdateMsg = Result<ViewerUpdate, Status>;
pub type ViewerUpdateMsg = ViewerUpdate;
// pub type ControllerUpdateMsg = Result<ControllerUpdate, MyCustomError>;
pub type ControllerUpdateMsg = ControllerUpdate;

// pub trait DynRecorder: Recorder + Sync + Send {}

// // struct DeprEffectRack<VU, CU, B>
// struct DeprEffectRack<VU, CU>
// where
//     // B: AudioBackend + Sync + Send,
//     VU: Clone,
//     CU: Clone,
// {
//     // state: DeprEffectRackStateType<VU, CU, B>,
//     state: DeprEffectRackStateType<VU, CU>,
// }

// #[derive(Debug)]
// pub struct ConnectedUserState<U, R> {
//     connection: Arc<RwLock<mpsc::Sender<U>>>,
//     recorder: Option<Arc<R>>,
//     config: StartOpts,
//     is_analyzing: bool,
// }

// impl<R> ConnectedUserState<Msg, R>
// where
//     R: Send + Sync,
// {
//     pub fn new(config: StartOpts, connection: mpsc::Sender<Msg>) -> Self {
//         Self {
//             config,
//             connection: Arc::new(RwLock::new(connection)),
//             recorder: None,
//             is_analyzing: false,
//         }
//     }

//     async fn start_analysis<T, B>(&mut self, audio_backend: &Arc<B>) -> Result<()>
//     where
//         B: AudioBackend + Sync + Send + 'static,
//         T: Float
//             + FloatConst
//             + Mel
//             + Hz
//             + Sync
//             + Send
//             + std::fmt::Debug
//             + Default
//             + ScalarOperand
//             + 'static,
//     {
//         self.is_analyzing = true;
//         let audio_backend = audio_backend.clone();

//         // create a tokio runtime to perform async operations in threads
//         let rt = tokio::runtime::Builder::new_current_thread()
//             .enable_all()
//             .build()?;

//         let (result_tx, result_rx) = std::sync::mpsc::channel();
//         let (rec_tx, rec_rx) = std::sync::mpsc::channel();
//         // let rec_file_name = file_name.clone();
//         let play_file = self.config.play_file.clone();
//         let builder = thread::Builder::new();
//         let recorder_thread = builder.name("recorder thread".to_string()).spawn(move || {
//             println!("starting the recording...");
//             let rec = audio_backend.new_recorder().expect("create recorder");

//             // if let Some(file) = self.state.read().await.config.play_file {
//             if let Some(file) = play_file {
//                 if let Err(err) = rec.stream_file(
//                     PathBuf::from(file),
//                     move |samples: Result<Array2<T>>, sample_rate, nchannels| {
//                         if let Err(err) = rec_tx.send((samples, sample_rate, nchannels)) {
//                             panic!("{}", err);
//                         }
//                     },
//                 ) {
//                     eprintln!("failed to stream input: {}", err);
//                 };
//             } else {
//                 if let Err(err) = rec.stream_input(
//                     true,
//                     move |samples: Result<Array2<T>>, sample_rate, nchannels| {
//                         if let Err(err) = rec_tx.send((samples, sample_rate, nchannels)) {
//                             panic!("{}", err);
//                         }
//                     },
//                 ) {
//                     eprintln!("failed to stream input: {}", err);
//                 }
//             }
//             println!("playback is over");
//         })?;

//         // thread that collects and analyzes samples
//         let builder = thread::Builder::new();
//         let analysis_thread = builder.name("analysis thread".to_string()).spawn(move || {
//             // let (sample_rate, nchannels) = match self.config.play_file {
//             //      Some(file) =>
//             //         <B as AudioBackend>::Rec::get_file_info(PathBuf::from(file).clone()).unwrap();
//             //      None => {
//             //         let input_config = self.input_device.default_input_config()?;
//             //      }
//             // };
//             let buffer_window_size = 2048;
//             let analyzer_opts = SpectralAnalyzerOptions {
//                 window_size: buffer_window_size,
//                 mel_bands: 24,
//                 // nchannels: nchannels,
//                 // sample_rate: sample_rate,
//                 fps: 60,
//                 ..SpectralAnalyzerOptions::default()
//             };
//             let mut analyzer = SpectralAnalyzer::<T>::new(analyzer_opts).unwrap();
//             let mut buffer = Array2::<T>::zeros((0, 2));
//             loop {
//                 match rec_rx.recv() {
//                     Ok((Ok(samples), sample_rate, nchannels)) => {
//                         // println!("new samples: {:?}", samples.shape());
//                         analyzer.options.nchannels = nchannels;
//                         analyzer.options.sample_rate = sample_rate;
//                         if let Err(err) = buffer.append(Axis(0), samples.view()) {
//                             eprintln!("failed to extend buffer: {}", err);
//                         }
//                         // println!("size of buffer: {:?}", buffer.shape());
//                         // todo: maybe measure the processing time here and try to keep the real time
//                         let buffer_size = buffer.len_of(Axis(0));
//                         if buffer_size > NumCast::from(sample_rate * 1).unwrap() {
//                             panic!("more than one second in the buffer");
//                         }

//                         let ready_buffers = buffer_size / buffer_window_size;
//                         let mut processed = 0;

//                         // process the chunks
//                         for i in (0..ready_buffers) {
//                             let start = i * buffer_window_size;
//                             let end = (i + 1) * buffer_window_size;
//                             // println!("analyzing from {} to {}", start, end);
//                             let chunk = buffer
//                                 .slice_axis(Axis(0), Slice::from(start..end))
//                                 .to_owned();
//                             if let Err(err) = result_tx.send(analyzer.analyze_samples(chunk)) {
//                                 eprintln!("failed to send result: {}", err);
//                             }
//                             processed += 1;
//                         }
//                         buffer.slice_axis_inplace(
//                             Axis(0),
//                             Slice::from((processed * buffer_window_size)..),
//                         );
//                     }
//                     Ok((Err(err), _, _)) => {
//                         println!("error while recording samples: {}", err);
//                     }
//                     Err(err) => {
//                         // println!("failed to receive new samples: {}", err);
//                     }
//                 }
//             }
//         })?;

//         // wait for analysis results and send them to the user
//         let stream_tx = self.connection.clone();
//         let builder = thread::Builder::new();
//         let update_thread = builder.name("upate thread".to_string()).spawn(move || {
//             let mut seq_num = 0;
//             loop {
//                 match result_rx.recv() {
//                     Ok(Err(analysis_err)) => {
//                         println!("{}", analysis_err);
//                     }
//                     Ok(Ok(mut analysis_result)) => {
//                         analysis_result.seq_num = seq_num;
//                         let analysis_result_update = Update {
//                             update: Some(update::Update::AudioAnalysisResult(analysis_result)),
//                         };
//                         // let stream_tx = self.connection;
//                         match rt.block_on(async {
//                             let rx = stream_tx.read().await;
//                             // rx.send(Ok(analysis_result_update)).await
//                             rx.send(analysis_result_update).await
//                         }) {
//                             Err(err) => println!("{}", err),
//                             Ok(_) => seq_num = seq_num + 1,
//                         };
//                     }
//                     Err(recv_err) => {
//                         println!("{}", recv_err);
//                         // break;
//                     }
//                 }
//             }
//         });

//         let analysis = AudioAnalysis {
//             recorder_thread,
//             is_running: true,
//         };
//         Ok(())
//     }
// }

// #[derive(Debug)]
// // pub struct DeprEffectRackState<VU, CU, B>
// pub struct DeprEffectRackState<VU, CU>
// where
//     // B: AudioBackend + Sync + Send,
//     VU: Clone,
//     CU: Clone,
// {
//     // pub audio_backend: Arc<B>,
//     pub config: StartOpts,
//     // mspc send channel for each user that is connected
//     // pub connected: HashMap<String, RwLock<ConnectedUserState<U, B::Rec>>>,
//     // pub sessions: HashMap<String, RwLock<Session<U, B::Rec>>>,
//     // pub sessions: HashMap<String, RwLock<Session<VU, CU>>>,
//     pub sessions: RwLock<HashMap<String, RwLock<Session<VU, CU>>>>,
// }

// // impl<B> DeprEffectRackState<ViewerUpdateMsg, ControllerUpdateMsg, B>
// impl DeprEffectRackState<ViewerUpdateMsg, ControllerUpdateMsg>
// // where
// //     B: AudioBackend + Sync + Send,
// {
//     async fn remove_session(&mut self, session_token: &String) {
//         println!("removing session {}", session_token);
//         self.sessions.write().await.remove(session_token);
//     }
//     // async fn get_or_create_session(&mut self, session_token: &String) {
//         // self.sessions.write().await.remove(session_token);

//     // async fn broadcast(&self, msg: Msg) {
//     //     for (session_token, state) in &self.connected {
//     //         let conn = &state.read().await.connection;
//     //         let conn = conn.read().await;
//     //         if let Err(err) = conn.send(msg.clone()).await {
//     //             println!("broadcast send error to {}, {:?}", session_token, err)
//     //         }

//     //         // this would be a simple msg.clone() but Status has no Clone
//     //         // match msg {
//     //         //     Ok(ref msg) => {
//     //         //         if let Err(err) = conn.send(Ok(msg.clone())).await {
//     //         //             println!("broadcast send error to {}, {:?}", session_token, err)
//     //         //         }
//     //         //     }
//     //         //     Err(ref err) => {
//     //         //         if let Err(err) = conn
//     //         //             .send(Err(Status::new(err.code(), err.to_string())))
//     //         //             .await
//     //         //         {
//     //         //             println!("broadcast send error to {}, {:?}", session_token, err)
//     //         //         }
//     //         //     }
//     //         // }
//     //     }
//     // }
// }

#[derive(Clone)]
// pub struct EffectRack<VU, CU, B>
// pub struct EffectRack<VU, CU, S>
pub struct EffectRack<VU, CU>
where
    // B: AudioBackend + Sync + Send + Clone,
    VU: Clone,
    CU: Clone,
    // S: Clone,
{
    // pub state: DeprEffectRackStateType<VU, CU, B>,
    pub config: Config,
    // pub sessions: RwLock<HashMap<String, RwLock<Session<VU, CU>>>>,
    // pub sessions: Arc<RwLock<HashMap<String, RwLock<Session<VU, CU>>>>>,
    // pub sessions: Arc<RwLock<HashMap<String, Session<VU, CU, S>>>>,
    pub sessions: Arc<RwLock<HashMap<proto::grpc::SessionToken, Session<VU, CU>>>>,
    // pub state: DeprEffectRackStateType<VU, CU>,
    pub shutdown_rx: watch::Receiver<bool>,
    // pub lights_running: bool,
}

// impl<VU, CU, S> EffectRack<VU, CU, S>
impl<VU, CU> EffectRack<VU, CU>
where
    // B: AudioBackend + Sync + Send + Clone,
    VU: Clone,
    CU: Clone,
    // S: Clone,
{
    // async fn remove_session(&mut self, session_token: &String) {
    //     println!("removing session {}", session_token);
    //     // self.sessions.write().await.remove(session_token);
    // }

    // async fn get_session(self, session_token: &String) -> Result<&Session<VU, CU>, Status> {
    //     let session = self.sessions
    //         .read()
    //         .await
    //         .get(session_token)
    //         .ok_or(Status::new(
    //             Code::NotFound,
    //             format!("session \"{}\" does not exist", session_token),
    //         ))?;
    //     Ok(&session)
    // }

    // async fn get_viewer_instance(
    //     self,
    //     session_token: &String,
    //     viewer_instance_id: &String,
    // ) -> Result<Viewer, Status> {
    //     self.get_session(session_token)
    //         .await?
    //         .viewers
    //         .read()
    //         .await
    //         .get(&instance_id)
    //         .ok_or(Status::new(
    //             Code::NotFound,
    //             format!("viewer instance \"{}\" does not exist", instance_id),
    //         ))
    // }

    async fn new_session(&self) -> Result<proto::grpc::SessionToken, Status> {
        let alphabet: [char; 26] = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];
        let mut sessions = self.sessions.write().await;
        let session_count = sessions.len();
        println!("there are {} sessions", session_count);
        if let Some(max_sessions) = self.config.run.max_sessions {
            if session_count >= max_sessions {
                return Err(Status::unavailable(format!(
                    "maximum number of sessions ({}) exceeded",
                    max_sessions
                )));
            }
        }
        let session_token = (|| {
            for _ in 0..100 {
                let token = proto::grpc::SessionToken {
                    token: nanoid!(4, &alphabet),
                };
                if sessions.get(&token).is_none() {
                    // insert
                    sessions.insert(
                        token.clone(),
                        // RwLock::new(Session::new(self.config.clone())),
                        Session::new(self.config.clone()),
                    );
                    return Ok(token);
                }
            }
            Err(Status::internal("failed to generate a new session token"))
        })();
        session_token

        // let viewers = sessions
        //     .get(session_token)
        //     .ok_or(Status::not_found(format!(
        //         "session \"{}\" does not exist",
        //         session_token
        //     )))?
        //     .viewers
        //     .read()
        //     .await;
        // println!("new session: {:?}", session_token);
        // let viewer_count = viewers.len();
        // if let Some(max_viewers) = self.config.run.max_viewers {
        //     if viewer_count >= max_viewers {
        //         return Err(Status::unavailable(format!(
        //             "maximum number of viewers ({}) exceeded",
        //             max_viewers
        //         )));
        //     }
        // }
        // println!("session {} has {} viewers", session_token, viewer_count);
        // let instance_id = (move || {
        //     for candidate in 1..viewer_count + 2 {
        //         if !viewers.contains_key(&proto::grpc::InstanceId {
        //             id: candidate.to_string(),
        //         }) {
        //             return Ok(proto::grpc::InstanceId {
        //                 id: candidate.to_string(),
        //             });
        //         }
        //     }
        //     Err(Status::internal("failed to find available instance id"))
        // })();
        // instance_id
        // Ok(proto::grpc::InstanceId { id: instance_id })
    }

    async fn extract_metadata<T>(request: &Request<T>, key: &str) -> Result<String, Status> {
        request
            .metadata()
            .get(key)
            .ok_or(Status::new(
                Code::InvalidArgument,
                format!("missing \"{}\" metadata", key),
            ))
            .and_then(|token| {
                token.to_str().map_err(|_| {
                    Status::new(
                        Code::InvalidArgument,
                        format!("failed to decode \"{}\" metadata", key),
                    )
                })
            })
            .map(|token| token.to_string())
    }

    async fn extract_session_token<T>(
        request: &Request<T>,
    ) -> Result<proto::grpc::SessionToken, Status> {
        Self::extract_metadata(request, SESSION_TOKEN_KEY)
            .await
            .map(|token| proto::grpc::SessionToken { token })
    }

    async fn extract_instance_id<T>(
        request: &Request<T>,
    ) -> Result<proto::grpc::InstanceId, Status> {
        Self::extract_metadata(request, INSTANCE_ID_KEY)
            .await
            .map(|id| proto::grpc::InstanceId { id })
    }
    async fn extract_session_instance<T>(
        request: &Request<T>,
    ) -> Result<(proto::grpc::SessionToken, proto::grpc::InstanceId), Status> {
        let session_token = Self::extract_session_token(request).await?;
        let instance_id = Self::extract_instance_id(request).await?;
        Ok((session_token, instance_id))
    }
}

// impl EffectRack<ViewerUpdateMsg, ControllerUpdateMsg>
impl<VU, CU> EffectRack<VU, CU>
// impl<B> EffectRack<ViewerUpdateMsg, ControllerUpdateMsg, B>
// where
//     B: AudioBackend + Clone + Sync + Send,
where
    VU: Clone + Send,
    CU: Clone + Send,
{
    fn new_with_shutdown(config: Config, shutdown_rx: watch::Receiver<bool>) -> Self {
        Self {
            config,
            shutdown_rx,
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    // fn new_with_shutdown(
    //     // state: DeprEffectRackStateType<ViewerUpdateMsg, ControllerUpdateMsg, B>,
    //     // state: DeprEffectRackStateType<VU, CU>,
    //     shutdown_rx: watch::Receiver<bool>,
    // ) -> Self {
    //     // Self { state, shutdown_rx }
    // }
}

// impl<VU, CU, B> DeprEffectRack<VU, CU, B>
// impl<VU, CU> DeprEffectRack<VU, CU>
// impl DeprEffectRack<ViewerUpdateMsg, ControllerUpdateMsg>
// impl EffectRack<ViewerUpdateMsg, ControllerUpdateMsg, MyCustomError>
impl EffectRack<ViewerUpdateMsg, ControllerUpdateMsg>
// impl<VU, CU> EffectRack<VU, CU>
// where
//     // B: AudioBackend + Sync + Clone + Send + 'static,
//     VU: Clone,
//     CU: Clone,
{
    // async fn shutdown(&self) -> Result<()> {
    //     println!("todo: set some internal oneshot channel or so");
    //     Ok(())
    // }

    async fn serve(&self) -> Result<()> {
        let addr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
        // let addr = SocketAddr::new(addr, self.state.read().await.config.port);
        let addr = SocketAddr::new(addr, self.config.run.port);

        println!("listening on {}", addr);

        // let remote_server =
        // EffectRack::new_with_shutdown(self.state.clone(), shutdown_rx.clone());
        // let remote_controller_server =
        //     EffectRack::new_with_shutdown(self.state.clone(), shutdown_rx.clone());
        // let remote_viewer_server =
        //     EffectRack::new_with_shutdown(self.state.clone(), shutdown_rx.clone());

        // let remote_controller_grpc_server = RemoteControllerServer::new(remote_controller_server);
        let remote_controller_grpc_server = RemoteControllerServer::new(self.clone());
        let remote_controller_grpc_server = tonic_web::config()
            // .allow_origins(vec!["localhost", "127.0.0.1"])
            .enable(remote_controller_grpc_server);

        // let remote_grpc_server = tonic_web::config()
        //     // .allow_origins(vec!["localhost", "127.0.0.1"])
        //     .enable(remote_server);

        // let remote_viewer_grpc_server = RemoteViewerServer::new(remote_viewer_server);
        let remote_viewer_grpc_server = RemoteViewerServer::new(self.clone());
        let remote_viewer_grpc_server = tonic_web::config()
            // .allow_origins(vec!["localhost", "127.0.0.1"])
            .enable(remote_viewer_grpc_server);

        TonicServer::builder()
            .accept_http1(true)
            // .add_service(remote_grpc_server)
            .add_service(remote_controller_grpc_server)
            .add_service(remote_viewer_grpc_server)
            .serve_with_shutdown(addr, async {
                self.shutdown_rx
                    .clone()
                    .changed()
                    .await
                    .expect("failed to shutdown");
            })
            .await?;
        Ok(())
    }
}

pub type Sample = f32;

#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    // let audio_backend = Arc::new(CpalAudioBackend::new(audio_config));

    if let Some(ref subcommand) = opts.commands {
        match subcommand {
            #[cfg(feature = "record")]
            Commands::Query(cfg) => {
                // println!("query:  {:?}", cfg);
                // if !cfg!(feature = "record") {
                //     return Err(FeatureDisabledError::new("record is not available").into());
                // }
                // rack.state.read().await.audio_backend.query()?;
                // audio_backend.query()?;
                let audio_backend = CpalAudioInput::<Sample>::new(AudioInputConfig::from(opts))?;
                cfg_if::cfg_if! {
                    if #[cfg(feature = "portaudio")] {
                        if config.use_portaudio{
                            let audio_backend  = PortaudioAudioInput::new(opts.into())?;
                        };
                    }
                };
                audio_backend.query()?;
            }
            Commands::Start(cfg) => {
                // println!("start:  {:?}", cfg);

                // let state = Arc::new(RwLock::new(DeprEffectRackState {
                // let state = Arc::new(DeprEffectRackState {
                //     sessions: RwLock::new(HashMap::new()),
                //     config: cfg,
                //     // audio_backend,
                // });
                let config = Config {
                    run: cfg.clone(),
                    default: opts.clone(),
                };

                let rack = Arc::new(EffectRack::new_with_shutdown(config, shutdown_rx));
                // {
                //     // todo: give them instances
                //     // analyzer: Arc::new(SpectralAnalyzer::new(SpectralAnalyzerOptions::default())),
                //     state,
                // });

                // let rack = rack.clone();
                let running = tokio::task::spawn(async move {
                    rack.serve().await.expect("failed to run rack");
                });

                signal::ctrl_c().await.ok().map(|_| {
                    println!("received shutdown");
                    shutdown_tx.send(true).expect("send shutdown signal");
                });

                // todo: also wait for other threads and all
                // running.await.ok();
                println!("exiting");
            }
        }
    }
    Ok(())
}
