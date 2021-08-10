use crate::cli::Config;
use crate::session::Session;
use crate::{EffectRack, MyCustomError, ViewerUpdateMsg, INSTANCE_ID_KEY, SESSION_TOKEN_KEY};
use analysis::spectral::{SpectralAnalyzer, SpectralAnalyzerOptions};
#[cfg(feature = "analyze")]
use analysis::{mel::Hz, mel::Mel, Analyzer};
use anyhow::Result;
use clap::Clap;
use common::errors::FeatureDisabledError;
use futures::{Future, Stream};
use ndarray::prelude::*;
use ndarray::{
    concatenate, indices, Array, IntoDimension, Ix, NdIndex, RemoveAxis, ScalarOperand, Slice, Zip,
};
use num::traits::{Float, FloatConst, NumCast, Zero};
#[cfg(feature = "portaudio")]
// use recorder::{backend::portaudio::PortaudioAudioBackend, portaudio::PortaudioRecorder};
use recorder::PortaudioRecorder;
#[cfg(feature = "record")]
use recorder::{cpal::CpalAudioInput, AudioInput, AudioInputConfig, Sample};
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
use tokio::sync::{mpsc, oneshot, watch, Mutex, RwLock};
use tokio::time;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server as TonicServer, Code, Request, Response, Status};

// trait NewTrait: Into<VU> + Send + Sync + 'static where Self: Sized {}

// pub type SendOut<VU, S> = mpsc::Sender<Result<VU, S>>;

#[derive(Debug)]
// pub struct Viewer<VU, S>
pub struct Viewer<VU> {
    // connection: Arc<RwLock<mpsc::Sender<Result<VU, Status>>>>,
    // connection: Arc<RwLock<SendOut<VU, tonic::transport::Status>>>,
    connection: Arc<RwLock<mpsc::Sender<Result<VU, Status>>>>,
    tx: mpsc::Sender<Result<VU, String>>,
    alive: Arc<RwLock<bool>>,

    pub config: Config,
    pub session_token: proto::grpc::SessionToken,
    pub instance_id: proto::grpc::InstanceId,
    // pub tx: mpsc::Sender<dyn Test: Into<VU>>,
    // pub tx: mpsc::Sender<Result<VU, S>>,
    // pub tx: mpsc::Sender<impl Into<VU>>,
    // is_analyzing: bool,
    // recorder: Option<Arc<R>>,
}

// impl<VU, CU> Controller<VU, CU> {
// impl<VU, S> Viewer<VU, S> {
impl<VU> Viewer<VU> {
    // pub fn new(
    //     config: Config,
    //     connection: mpsc::Sender<Result<VU, Status>>,
    //     tx: mpsc::Sender<Result<VU, String>>,
    // ) -> Self {
    //     Self {
    //         config,
    //         tx,
    //         instance_id,
    //         session,
    //         connection: Arc::new(RwLock::new(connection)),
    //     }
    // }
    pub async fn send(&self, message: Result<VU, String>) {
        if *self.alive.read().await {
            // send the message
            self.tx.send(message).await;
        }
    }

    pub fn start(&self) {}
}

// impl<CU> EffectRack<ViewerUpdateMsg, CU, MyCustomError>
impl<CU> EffectRack<ViewerUpdateMsg, CU>
where
    // B: AudioBackend + Clone + Sync + Send + 'static,
    CU: Clone + Send + 'static,
{
    async fn new_viewer_instance(
        &self,
        session_token: proto::grpc::SessionToken,
        // instance: Option<proto::grpc::InstanceId>,
    ) -> Result<proto::grpc::InstanceId, Status> {
        println!("[viewer] new instance id for session: {}", session_token);
        let mut sessions = self.sessions.write().await;
        let viewers = sessions
            .entry(session_token.clone())
            .or_insert(Session::new(self.config.clone()))
            // .ok_or(Status::not_found(format!(
            //     "session {} does not exist",
            //     session_token
            // )))?
            .viewers
            .write()
            .await;
        let viewer_count = viewers.len();
        if let Some(max_viewers) = self.config.run.max_viewers {
            if viewer_count >= max_viewers {
                return Err(Status::unavailable(format!(
                    "maximum number of viewers ({}) exceeded",
                    max_viewers
                )));
            }
        }
        println!("session {} has {} viewers", session_token, viewer_count);
        let instance_id = (move || {
            for candidate in 1..viewer_count + 2 {
                let id = proto::grpc::InstanceId {
                    id: candidate.to_string(),
                };
                if !viewers.contains_key(&id) {
                    // insert here
                    return Ok(id);
                }
            }
            Err(Status::internal("failed to generate instance id"))
        })();
        instance_id
        // Ok(proto::grpc::InstanceId { id: instance_id })
    }
}

#[tonic::async_trait]
// impl<CU, B> RemoteViewer for EffectRack<ViewerUpdateMsg, CU, B>
impl<CU> proto::grpc::remote_viewer_server::RemoteViewer for EffectRack<ViewerUpdateMsg, CU>
where
    // B: AudioBackend + Clone + Sync + Send + 'static,
    CU: Send + Clone + 'static,
{
    type ConnectStream = Pin<
        Box<dyn Stream<Item = Result<proto::grpc::ViewerUpdate, Status>> + Send + Sync + 'static>,
    >;

    async fn update_subscription(
        &self,
        request: Request<proto::grpc::UpdateSubscriptionRequest>,
    ) -> Result<Response<proto::grpc::Empty>, Status> {
        let (session_token, instance_id) = Self::extract_session_instance(&request).await?;
        // todo:
        println!("update subscription: {} {}", session_token, instance_id);
        Ok(Response::new(proto::grpc::Empty {}))
    }

    // async fn new_instance_id(
    //     &self,
    //     request: Request<proto::grpc::NewInstanceIdRequest>,
    // ) -> Result<Response<proto::grpc::InstanceId>, Status> {
    //     let session_token = Self::extract_session_token(&request)?;
    //     println!("[viewer] new instance token for session: {}", session_token);
    //     let sessions = self.sessions.read().await;
    //     let viewers = sessions
    //         .get(&session_token)
    //         .ok_or(Status::not_found(format!(
    //             "session \"{}\" does not exist",
    //             session_token
    //         )))?
    //         .viewers
    //         .read()
    //         .await;
    //     let viewer_count = viewers.len();
    //     if let Some(max_viewers) = self.config.run.max_viewers {
    //         if viewer_count >= max_viewers {
    //             return Err(Status::unavailable(format!(
    //                 "maximum number of viewers ({}) exceeded",
    //                 max_viewers
    //             )));
    //         }
    //     }
    //     let instance_id = (move || {
    //         for candidate in 1..viewer_count + 2 {
    //             if !viewers.contains_key(&candidate.to_string()) {
    //                 return Ok(candidate.to_string());
    //             }
    //         }
    //         Err(Status::internal("failed to find available instance id"))
    //     })()?;
    //     println!("session {} has {} viewers", session_token, viewer_count);
    //     Ok(Response::new(proto::grpc::InstanceId { id: instance_id }))
    // }

    async fn disconnect(
        &self,
        request: Request<proto::grpc::ViewerDisconnectRequest>,
    ) -> Result<Response<proto::grpc::Empty>, Status> {
        let (session_token, instance_id) = Self::extract_session_instance(&request).await?;
        self.sessions
            .read()
            .await
            .get(&session_token)
            .ok_or(Status::new(
                Code::NotFound,
                format!("session {} does not exist", session_token),
            ))?
            .viewers
            .write()
            .await
            .remove(&instance_id)
            .ok_or(Status::new(
                Code::NotFound,
                format!("viewer instance {} does not exist", instance_id),
            ))?;
        println!("[viewer] disconnect: {} {}", session_token, instance_id);
        Ok(Response::new(proto::grpc::Empty {}))
    }

    async fn connect(
        &self,
        request: Request<proto::grpc::ViewerConnectRequest>,
    ) -> Result<Response<Self::ConnectStream>, Status> {
        let session_token = match Self::extract_session_token(&request).await {
            Ok(token) => Ok(token),
            Err(_) => self.new_session().await,
        }?;
        // .or_else(|_| -> Result<proto::grpc::SessionToken, Status> {
        //     async { self.new_session().await }
        // });
        // .or_else(async |_| -> Result<proto::grpc::SessionToken, Status> {
        // .or_else(|_| async {
        //     self.new_session().await
        // })?;

        let instance_id = Self::extract_instance_id(&request)
            .await
            .or(self.new_viewer_instance(session_token.clone()).await)?;

        println!("[viewer] connect: {} {}", session_token, instance_id);
        let (stream_tx, stream_rx) = mpsc::channel(1);
        let pinned_stream = Box::pin(ReceiverStream::new(stream_rx));
        let mut response: Response<Self::ConnectStream> = Response::new(pinned_stream);
        let mut metadata = response.metadata_mut();
        metadata.insert(
            SESSION_TOKEN_KEY,
            session_token.clone().token.parse().unwrap(),
        );
        metadata.insert(INSTANCE_ID_KEY, instance_id.clone().id.parse().unwrap());
        let mut sessions = self.sessions.write().await;
        let session = sessions
            .entry(session_token.clone())
            .or_insert(Session::new(self.config.clone()));
        // let session = self.sessions.read().await.get();
        // .or_insert(RwLock::new(Session::new(self.config.clone())));
        // println!("have session");
        let (tx, mut rx) = mpsc::channel(1);
        // let viewers = session.viewers.read();
        // let viewer = viewers.await.get(&instance_id);
        if let Some(existing) = session.viewers.read().await.get(&instance_id) {
            println!(
                "instance {} in session {} reconnected",
                instance_id, session_token
            );
            let existing = existing.read().await;
            let mut old_connection = existing.connection.write().await;
            *old_connection = stream_tx.clone();
            println!("exiting");
            return Ok(response);
        }

        println!(
            "instance {} in session {} connected",
            instance_id, session_token
        );
        let viewer = Viewer {
            connection: Arc::new(RwLock::new(stream_tx.clone())),
            tx,
            alive: Arc::new(RwLock::new(true)),
            config: self.config.clone(),
            session_token: session_token.clone(),
            instance_id: instance_id.clone(),
        };
        let stream_tx = viewer.connection.clone();
        let alive_state = viewer.alive.clone();
        session
            .viewers
            .write()
            .await
            .insert(instance_id.clone(), RwLock::new(viewer));
        // connected.insert(user_token.clone(), RwLock::new(user_state));

        // println!("preparing to go into the busy loop");
        // let state = self.state.clone();
        // let update_tx = stream_tx.clone();
        // let conn = viewer.clone();
        // let stream_tx = stream_tx.clone();
        let viewers = session.viewers.clone();
        let mut shutdown_rx = self.shutdown_rx.clone();
        let config = self.config.clone();
        tokio::spawn(async move {
            // assign the instance
            let assignment = proto::grpc::ViewerUpdate {
                update: Some(proto::grpc::viewer_update::Update::Assignment(
                    proto::grpc::Assignment {
                        session_token: Some(session_token.clone()),
                        instance_id: Some(instance_id.clone()),
                    },
                )),
            };

            let _ = stream_tx.read().await.send(Ok(assignment)).await;

            // wait for either shutdown, heartbeat or an update to send out
            let mut seq = 0u64;
            // todo: counter for failed send events
            let heartbeat_interval = time::sleep(Duration::from_millis(5 * 1000));
            let mut alive = true;
            let mut last_heartbeat = tokio::time::Instant::now();
            let mut heartbeat_timer = Box::pin(heartbeat_interval);
            let mut shutdown_signal = Box::pin(shutdown_rx.changed());
            loop {
                let was_alive = alive;
                tokio::select! {
                    _ = &mut shutdown_signal => {
                        println!("shutdown from open connection");
                        return;
                    }
                    received = rx.recv() => {
                        if let Some(received) = received {
                        match received {
                            Ok(update) => {
                                // if let Err(err) = stream_tx.read().await.send(Ok(update)).await {
                                alive = stream_tx.read().await.send(Ok(update)).await.is_ok();

                            }
                            Err(err) => {
                                alive = stream_tx.read().await.send(Err(Status::internal(err))).await.is_ok();
                            }
                        }
                        }
                    }
                    _ = &mut heartbeat_timer => {
                        // reset the heartbeat
                        let heartbeat_interval = time::sleep(Duration::from_millis(5 * 1000));
                        heartbeat_timer = Box::pin(heartbeat_interval);

                        let heartbeat = proto::grpc::ViewerUpdate {
                            update: Some(proto::grpc::viewer_update::Update::Heartbeat(proto::grpc::Heartbeat { seq })),
                        };
                        match stream_tx.read().await.send(Ok(heartbeat)).await {
                            Err(_) => {
                                alive = false;
                                if last_heartbeat.elapsed().as_secs() > config.run.max_keepalive_sec {
                                    // remove the intstance
                                    eprintln!("can no longer send to {} {}", session_token, instance_id);
                                    if viewers.write().await.remove(&instance_id).is_none() {
                                        eprintln!("failed to remove {} {}", session_token, instance_id);
                                    }
                                    eprintln!("removed {} {}", session_token, instance_id);
                                    return;
                                }
                            }
                            Ok(_) => {
                                alive = true;
                                last_heartbeat =  tokio::time::Instant::now();
                            }
                        };
                        seq = seq + 1;
                    }
                }
                if was_alive != alive {
                    let mut new_liveness = alive_state.write().await;
                    *new_liveness = alive;
                }
            }
        });

        println!("exiting");
        Ok(response)

        // let mut old_connection = existing.connection.write().await;
        // let config = self.state.read().await.config.clone();
        // let sessions = ;
        // match self.sessions.read().await.get(&session_token) {
        //     Some(session) => {
        //         match session.get_mut(&instance_id) {
        //             Some(instance) => {
        //             }
        //             None => {
        //             }
        //         }
        //         println!(
        //             "instance {} in session {} reconnected",
        //             instance_id, session_token
        //         );
        //         // existing.write
        //         // let user_state = existing.read().await;
        //         // let mut old_connection = user_state.connection.write().await;
        //         // let mut old_connection = existing.connection.write().await;
        //         // *old_connection = tx;
        //     }
        //     None => {
        //         return Err(Status::new(
        //             Code::Internal,
        //             format!("session \"{}\" not found", session_token),
        //         ));
        //         // println!("{} connected", user_token);
        //         // let user_state = ConnectedUserState::new(config, tx);
        //         // connected.insert(user_token.clone(), RwLock::new(user_state));
        //     }
        // }

        // let session = match self.sessions.read().await.get(&session_token) {
        // match connected.get_mut(&user_token) {
        //     Some(existing) => {
        //         println!("({} reconnected)", user_token);
        //         let user_state = existing.read().await;
        //         let mut old_connection = user_state.connection.write().await;
        //         *old_connection = tx;
        //     }
        //     None => {
        //         println!("{} connected", user_token);
        //         let user_state = ConnectedUserState::new(config, tx);
        //         connected.insert(user_token.clone(), RwLock::new(user_state));
        //     }
        // }

        // let state = self.state.clone();
        // let update_tx = stream_tx.clone();
        // let update_user_token = user_token.clone();
        // let mut shutdown_rx = self.shutdown_rx.clone();
        // tokio::spawn(async move {
        //     // send ack
        //     let _ = stream_tx.send(Ok(Update::default())).await;

        //     // wait for either shutdown, heartbeat or an update to send out
        //     let mut seq = 0u64;
        //     // todo: counter for failed send events
        //     let heartbeat_interval = time::sleep(Duration::from_millis(5 * 1000));
        //     let mut heartbeat_timer = Box::pin(heartbeat_interval);
        //     let mut shutdown_signal = Box::pin(shutdown_rx.changed());
        //     loop {
        //         tokio::select! {
        //             _ = &mut shutdown_signal => {
        //                 println!("shutdown from open connection");
        //                 break;
        //             }
        //             received = rx.recv() => {
        //                 if let Some(update) = received {
        //                     if let Err(err) = update_tx.send(Ok(update)).await {
        //                         // If sending failed, then remove the user from shared data
        //                         eprintln!("failed to send to user {}: {}", update_user_token, err);
        //                         state.write().await.remove_user(&update_user_token);
        //                         break;
        //                     }
        //                 }
        //             }
        //             _ = &mut heartbeat_timer => {
        //                 // reset the heartbeat
        //                 let heartbeat_interval = time::sleep(Duration::from_millis(5 * 1000));
        //                 heartbeat_timer = Box::pin(heartbeat_interval);

        //                 let heartbeat = Update {
        //                     update: Some(update::Update::Heartbeat(Heartbeat { seq })),
        //                 };
        //                 if let Err(_) = update_tx.send(Ok(heartbeat)).await {
        //                     state.write().await.remove_user(&update_user_token);
        //                     // break;
        //                 };
        //                 seq = seq + 1;
        //             }
        //         }
        //     }
        // });
    }
}
