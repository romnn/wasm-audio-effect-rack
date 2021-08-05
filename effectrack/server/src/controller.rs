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
impl<B> RemoteController for RemoteService<Msg, B>
where
    B: AudioBackend + Clone + Sync + Send + 'static,
{
    // register_visualization
    // async fn query_current_visualization(
    //     &self,
    //     request: Request<QueryCurrentVisualizationRequest>,
    // ) -> Result<Response<Visualization>, Status> {
    //             Ok(Response::new(Visualization{}))
    // }

    async fn start_analysis(
        &self,
        request: Request<StartAnalysisRequest>,
    ) -> Result<Response<Empty>, Status> {
        let user_token = Self::extract_user_token(request)?;
        let audio_backend = &self.state.read().await.audio_backend;
        println!("wants to start: {}", user_token);
        match self.state.read().await.connected.get(&user_token) {
            Some(user) => {
                // check if the user is already running an analysis first
                if user.read().await.is_analyzing {
                    println!("already running an analysis");
                    // or use status already exists?
                    return Err(Status::ok(format!(
                        "user {} is alreay running an analysis",
                        user_token
                    )));
                }
                // this will not block
                user.write()
                    .await
                    .start_analysis::<f32, _>(&audio_backend)
                    .await;

                Ok(Response::new(Empty {}))
            }
            None => Err(Status::not_found(format!(
                "user {} is not (yet) connected",
                user_token
            ))),
        }
    }
}
