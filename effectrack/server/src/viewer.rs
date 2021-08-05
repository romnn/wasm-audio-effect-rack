use analyzer::spectral::{SpectralAnalyzer, SpectralAnalyzerOptions};
#[cfg(feature = "analyze")]
use analyzer::{mel::Hz, mel::Mel, Analyzer};
use anyhow::Result;
use clap::Clap;
use crate::{ConnectedUserState, Msg, RemoteService};
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

#[tonic::async_trait]
impl<B> RemoteViewer for RemoteService<Msg, B>
where
    B: AudioBackend + Clone + Sync + Send + 'static,
{
    type SubscribeStream =
        Pin<Box<dyn Stream<Item = Result<Update, Status>> + Send + Sync + 'static>>;

    async fn update_subscription(
        &self,
        request: Request<UpdateSubscriptionRequest>,
    ) -> Result<Response<Empty>, Status> {
        let user_token = Self::extract_user_token(request)?;
        Ok(Response::new(Empty {}))
    }

    async fn unsubscribe(
        &self,
        request: Request<UnsubscriptionRequest>,
    ) -> Result<Response<Empty>, Status> {
        let user_token = Self::extract_user_token(request)?;
        Ok(Response::new(Empty {}))
    }

    async fn subscribe(
        &self,
        request: Request<SubscriptionRequest>,
    ) -> Result<Response<Self::SubscribeStream>, Status> {
        let user_token = Self::extract_user_token(request)?;
        let (stream_tx, stream_rx) = mpsc::channel(1);
        let pinned_stream = Box::pin(ReceiverStream::new(stream_rx));
        let (tx, mut rx) = mpsc::channel(1);

        let connected = &mut self.state.write().await.connected;
        match connected.get_mut(&user_token) {
            Some(existing) => {
                println!("({} reconnected)", user_token);
                let user_state = existing.read().await;
                let mut old_connection = user_state.connection.write().await;
                *old_connection = tx;
            }
            None => {
                let user_state = ConnectedUserState::new(tx);
                connected.insert(user_token.clone(), RwLock::new(user_state));
                println!("{} connected", user_token);
            }
        }

        let state = self.state.clone();
        let update_tx = stream_tx.clone();
        let update_user_token = user_token.clone();
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
                            if let Err(err) = update_tx.send(Ok(update)).await {
                                // If sending failed, then remove the user from shared data
                                // state.write().await.remove_user(&update_user_token);
                                // break;
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
                            // state.write().await.remove_user(&update_user_token);
                            // break;
                        };
                        seq = seq + 1;
                    }
                }
            }
        });

        Ok(Response::new(pinned_stream))
    }
}
