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
use analyzer::Analyzer;
use anyhow::Result;
use clap::Clap;
use cli::{Commands, Opts};
use common::errors::FeatureDisabledError;
use futures::{Future, Stream};
use proto::grpc::remote_controller_server::{RemoteController, RemoteControllerServer};
use proto::grpc::{SubscriptionRequest, Update};
#[cfg(feature = "portaudio")]
use recorder::PortaudioRecorder;
#[cfg(feature = "record")]
use recorder::{CpalRecorder, Recorder, RecorderOptions};
use std::collections::HashMap;
use std::error::Error;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::signal;
use tokio::sync::{mpsc, oneshot, RwLock};
use tokio::time;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server as TonicServer, Code, Request, Response, Status};

pub type EffectRackStateType = Arc<RwLock<EffectRackState>>;
// type EffectRackStateType = RwLock<EffectRackState>;

struct EffectRack {
    recorder: Arc<dyn Recorder + Sync + Send>,
    analyzer: Arc<dyn Analyzer + Sync + Send>,
    // analyzer: Arc<Analyzer>,
    // recorder: Box<dyn Recorder + Sync + Send>,
    // analyzer: Analyzer,
    state: EffectRackStateType,
    // server: RemoteControllerGRPCServer,
    // analyzer: Arc<Analyzer<'a>>,
    // recorder: Arc<dyn Recorder + Sync + Send>,
    // analyzer: Arc<&'a Analyzer>,
}

#[derive(Debug)]
struct ConnectedUserState {
    connection: mpsc::Sender<Update>,
}

impl ConnectedUserState {
    pub fn new(connection: mpsc::Sender<Update>) -> Self {
        Self { connection }
    }
}

#[derive(Debug)]
pub struct EffectRackState {
    // mspc send channel for each user that is connected
    connected: HashMap<String, RwLock<ConnectedUserState>>,
}

impl EffectRackState {
    fn new() -> Self {
        Self {
            connected: HashMap::new(),
        }
    }

    async fn broadcast(&self, msg: Update) {
        // let uwe = self.connected.read().await;
        for (user_id, state) in &self.connected {
            match state.read().await.connection.send(msg.clone()).await {
                Ok(_) => {}
                Err(_) => {
                    println!("broadcast send error to {}, {:?}", user_id, msg)
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct RemoteControllerService {
    pub state: EffectRackStateType,
    // pub shutdown_rx:
}

impl RemoteControllerService {
    fn new(state: EffectRackStateType) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl RemoteController for RemoteControllerService {
    // type SubscribeStream = ReceiverStream<Result<Update, Status>>;
    type SubscribeStream =
        Pin<Box<dyn Stream<Item = Result<Update, Status>> + Send + Sync + 'static>>;

    async fn subscribe(
        &self,
        request: Request<SubscriptionRequest>,
    ) -> Result<Response<Self::SubscribeStream>, Status> {
        let user_id = request.into_inner().user_id;
        if user_id.len() < 1 {
            // println!("will not connect with user without user_id");
            return Err(Status::new(
                Code::InvalidArgument,
                "will not connect with user without user_id",
            ));
        }
        println!("{} connected", user_id);

        let (stream_tx, stream_rx) = mpsc::channel(1);
        let (tx, mut rx) = mpsc::channel(1);
        let user_state = ConnectedUserState::new(tx);
        let connected = &mut self.state.write().await.connected;
        if connected.contains_key(&user_id) {
            return Err(Status::new(
                Code::InvalidArgument,
                "will not connect with user without user_id",
            ));
        };
        connected.insert(user_id.clone(), RwLock::new(user_state));

        let recorder_thread = thread::spawn(|| {
            let recorder = CpalRecorder::new(RecorderOptions::default()).unwrap();
            recorder.stream_file(PathBuf::from(
                "/home/roman/dev/wasm-audio-effect-rack/experimental/audio-samples/roddy.wav",
            ));
            println!("playback is over. exiting thread");
        });

        let state = self.state.clone();
        tokio::spawn(async move {
            // send ack
            // stream_tx.send(Ok(Update::default())).await;
            // stream_tx.send(Err(Status::new(
            //     Code::Ok,
            //     "connected",
            // ))).await;

            // wait for either shutdown or an update to send out
            loop {
                tokio::select! {
                    received = rx.recv() => {
                        if let Some(update) = received {
                            match stream_tx.send(Ok(update)).await {
                                Ok(_) => {}
                                Err(_) => {
                                    // If sending failed, then remove the user from shared data
                                    println!("[Remote] stream tx sending error. Remote {}", &user_id);
                                    state.write().await.connected.remove(&user_id);
                                        }
                        }
                        }
                    }
                }
                // wait for updates and send them to the user
                // while let Some(update) = rx.recv().await {
                //     match stream_tx.send(Ok(update)).await {
                //         Ok(_) => {}
                //         Err(_) => {
                //             // If sending failed, then remove the user from shared data
                //             println!("[Remote] stream tx sending error. Remote {}", &user_id);
                //             state.write().await.connected.remove(&user_id);
                //         }
                //     }
                // }
            }
        });

        Ok(Response::new(Box::pin(ReceiverStream::new(stream_rx))))
        // Ok(Response::new(ReceiverStream::new(stream_rx)))
    }
}

impl EffectRack {
    async fn shutdown(&self) -> Result<()> {
        println!("todo: set some internal oneshot channel or so");
        Ok(())
    }

    async fn start<F>(&self, addr: SocketAddr, shutdown_signal: F) -> Result<()>
    where
        F: Future<Output = ()>,
    {
        println!("listening on {}", addr);

        // let state = self.state.clone();
        let server = RemoteControllerService::new(self.state.clone());

        let analyzer = self.analyzer.clone();
        let analyzer_thread = thread::spawn(move || {
            // println!("{:?}", self.recorder);
            // println!("{:?}", analyzer);
            // for i in 1..10 {
            // server.broadcast(i).await;
            // println!("hi number {} from the spawned thread!", i);
            // thread::sleep(Duration::from_millis(1));
            // }
        });

        // let recorder = self.recorder.clone();
        // for t in thrds {
        // t.join();
        // }
        // analyzer_thread.join();

        let grpc_server = RemoteControllerServer::new(server);
        let grpc_server = tonic_web::config()
            // .allow_origins(vec!["localhost", "127.0.0.1"])
            .enable(grpc_server);

        let state = self.state.clone();
        tokio::spawn(async move {
            for x in 0..2 {
                for x in 0..5 {
                    time::sleep(Duration::from_millis(1 * 1000)).await;
                    state.read().await.broadcast(Update::default()).await;
                }
                time::sleep(Duration::from_millis(5 * 1000)).await;
            }
            println!("done pushing updates to");
        });

        let tserver = TonicServer::builder()
            .accept_http1(true)
            .add_service(grpc_server)
            .serve_with_shutdown(addr, shutdown_signal)
            .await?;
        Ok(())
    }
}

#[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    // let state = Arc::new(RwLock::new(EffectRackState::new()));
    // let rec_options = RecorderOptions::default();
    let rec_opts = RecorderOptions {
        #[cfg(use_jack)]
        jack: opts.use_jack_backend,
        device: opts.device,
    };
    let recorder = Arc::new(CpalRecorder::new(rec_opts)?);

    #[cfg(feature = "portaudio")]
    if opts.use_portaudio_backend {
        let recorder = Arc::new(PortaudioRecorder::new(rec_opts)?);
    };

    // let rack: &'static EffectRack = &EffectRack {
    let rack = Arc::new(EffectRack {
        // recorder: Box::new(CpalRecorder::new(RecorderOptions::default())?),
        // analyzer: Analyzer::new(),
        // state: RwLock::new(EffectRackState::new()),
        recorder,
        analyzer: Arc::new(SpectralAnalyzer::new(SpectralAnalyzerOptions::default())),
        state: Arc::new(RwLock::new(EffectRackState::new())),
    });

    if let Some(subcommand) = opts.commands {
        match subcommand {
            #[cfg(feature = "record")]
            Commands::Query(cfg) => {
                // println!("query:  {:?}", cfg);
                // if !cfg!(feature = "record") {
                //     return Err(FeatureDisabledError::new("record is not available").into());
                // }
                rack.recorder.query();
            }
            Commands::Start(cfg) => {
                // println!("start:  {:?}", cfg);
                let addr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
                let addr = SocketAddr::new(addr, cfg.port);

                let rack = rack.clone();
                let running = tokio::task::spawn(async move {
                    rack.start(addr, async {
                        shutdown_rx.await.ok();
                        println!("telling the rack to shutdown");
                        rack.shutdown().await.ok();
                    })
                    .await;
                });

                signal::ctrl_c().await.ok().map(|_| {
                    println!("received shutdown");
                    shutdown_tx.send(()).expect("send shutdown signal");
                });

                // this is not graceful at all
                // running.await.ok();
                println!("exiting");
            }
        }
    }
    Ok(())
}
