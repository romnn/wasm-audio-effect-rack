use crate::analyzer::AudioInputNode;
use crate::analyzer::AudioOutputNode;
use crate::cli::Config;
use crate::{ControllerUpdateMsg, EffectRack, MyCustomError};
#[cfg(feature = "analyze")]
use analysis::spectral::{SpectralAnalyzer, SpectralAnalyzerOptions};
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
use recorder::portaudio::PortaudioRecorder;
#[cfg(feature = "record")]
use recorder::{
    cpal::CpalAudioBackend, cpal::CpalAudioInput, AudioInput, AudioInputConfig, AudioOutput,
    AudioOutputConfig, Sample,
};

// use recorder::{backend::portaudio::PortaudioAudioBackend, portaudio::PortaudioRecorder};

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

#[derive(Debug)]
// pub struct Controller<VU, CU> {
pub struct Controller<CU> {
    config: Config,
    connection: Arc<RwLock<mpsc::Sender<CU>>>,
    // analyzers: Vec<Arc<RwLock<mpsc::Sender<U>>>,
}

// impl<VU, CU> Controller<VU, CU> {
impl<CU> Controller<CU> {
    pub fn new(config: Config, connection: mpsc::Sender<CU>) -> Self {
        Self {
            config,
            connection: Arc::new(RwLock::new(connection)),
        }
    }
}

// impl<VU> EffectRack<VU, ControllerUpdateMsg, MyCustomErro>
impl<VU> EffectRack<VU, ControllerUpdateMsg>
where
    // B: AudioBackend + Clone + Sync + Send + 'static,
    VU: Clone + Send + 'static,
{
    // async fn new_controller_instance_id(
    //     &self,
    //     request: Request<proto::grpc::NewInstanceIdRequest>,
    // ) -> Result<Response<proto::grpc::InstanceId>, Status> {
    //     let session_token = Self::extract_session_token(&request).await?;
    //     println!(
    //         "[controller] new instance token for session: {:?}",
    //         session_token
    //     );
    //     Ok(Response::new(proto::grpc::InstanceId {
    //         id: "1".to_string(),
    //     }))
    // }
}

#[tonic::async_trait]
// impl<VU, B> RemoteController for EffectRack<VU, ControllerUpdateMsg, B>
impl<VU> proto::grpc::remote_controller_server::RemoteController
    for EffectRack<VU, ControllerUpdateMsg>
where
    // B: AudioBackend + Clone + Sync + Send + 'static,
    VU: Clone + Send + Sync + 'static,
{
    type ConnectStream = Pin<
        Box<
            dyn Stream<Item = Result<proto::grpc::ControllerUpdate, Status>>
                + Send
                + Sync
                + 'static,
        >,
    >;

    // register_visualization
    // async fn query_current_visualization(
    //     &self,
    //     request: Request<QueryCurrentVisualizationRequest>,
    // ) -> Result<Response<Visualization>, Status> {
    //             Ok(Response::new(Visualization{}))
    // }

    async fn disconnect(
        &self,
        request: Request<proto::grpc::ControllerDisconnectRequest>,
    ) -> Result<Response<proto::grpc::Empty>, Status> {
        let (session_token, instance_id) = Self::extract_session_instance(&request).await?;
        println!("[controller] disconnect: {} {}", session_token, instance_id);
        Ok(Response::new(proto::grpc::Empty {}))
    }

    async fn connect(
        &self,
        request: Request<proto::grpc::ControllerConnectRequest>,
    ) -> Result<Response<Self::ConnectStream>, Status> {
        let (stream_tx, stream_rx) = mpsc::channel(1);
        let pinned_stream = Box::pin(ReceiverStream::new(stream_rx));

        Ok(Response::new(pinned_stream))
    }

    // start analysis?
    // create input stream
    // create analyzer
    // the entire composition should be controlleable by the controller

    async fn add_audio_input_stream(
        &self,
        request: Request<proto::grpc::AddAudioInputStreamRequest>,
    ) -> Result<Response<proto::grpc::AudioInputStream>, Status> {
        let (session_token, instance_id) = Self::extract_session_instance(&request).await?;
        println!("new input stream: {} {}", session_token, instance_id);
        let input_config = AudioInputConfig {
            // TODO: use the request here
            // #[cfg(use_jack)]
            // use_jack: self.config.default.use_jack,
            // #[cfg(use_portaudio)]
            // use_portaudio: self.config.default.use_portaudio.clone(),
            // input_device: self.config.default.input_device.clone(),
            // output_device: self.config.default.output_device.clone(),
            // latency: NumCast::from(self.config.default.latency).unwrap(),
            // ..self.config.default.clone().into()
            ..AudioInputConfig::default()
        };
        let sessions = self.sessions.read().await;
        let mut input_streams = sessions
            .get(&session_token)
            .ok_or(Status::not_found(format!(
                "session {} does not exist",
                session_token
            )))?
            .input_streams
            .write()
            .await;
        // check if the stream is alreay active
        let mut audio_input = AudioInputNode::<f32>::new(input_config)
            .map_err(|_| Status::internal("failed to create input stream"))?;
        let descriptor = audio_input
            .input
            .descriptor()
            .map_err(|_| Status::internal("failed to create audio stream descriptor"))?;
        audio_input.stream_from_input(Box::new(
            move |samples: Result<Array2<f32>>, sample_rate, nchannels| {
                // todo: send to all subscribed analyzers
                println!("got samples: {:?}", samples);
                // if let Err(err) = rec_tx.send((samples, sample_rate, nchannels)) {
                //     panic!("{}", err);
                // }
            },
        ));

        if input_streams.contains_key(&descriptor) {
            return Err(Status::internal(
                "failed to create audio input stream descriptor",
            ));
        } else {
            input_streams.insert(descriptor, RwLock::new(audio_input))
        };
        Ok(Response::new(proto::grpc::AudioInputStream {}))
    }

    async fn add_audio_output_stream(
        &self,
        request: Request<proto::grpc::AddAudioOutputStreamRequest>,
    ) -> Result<Response<proto::grpc::AudioOutputStream>, Status> {
        let (session_token, instance_id) = Self::extract_session_instance(&request).await?;
        println!("new output stream: {} {}", session_token, instance_id);
        let output_config = AudioOutputConfig {
            // TODO: use the request here
            // #[cfg(use_jack)]
            // use_jack: self.config.default.use_jack,
            // #[cfg(use_portaudio)]
            // use_portaudio: self.config.default.use_portaudio.clone(),
            // input_device: self.config.default.input_device.clone(),
            // output_device: self.config.default.output_device.clone(),
            // latency: NumCast::from(self.config.default.latency).unwrap(),
            // ..self.config.default.clone().into()
            ..AudioOutputConfig::default()
        };
        let sessions = self.sessions.read().await;
        let mut output_streams = sessions
            .get(&session_token)
            .ok_or(Status::not_found(format!(
                "session {} does not exist",
                session_token
            )))?
            .output_streams
            .write()
            .await;
        // check if the stream is alreay active
        let mut audio_output = AudioOutputNode::<crate::Sample>::new(output_config)
            .map_err(|_| Status::internal("failed to create input stream"))?;
        let descriptor = audio_output
            .output
            .descriptor()
            .map_err(|_| Status::internal("failed to create audio output stream descriptor"))?;
        audio_output.stream_to_output(
            // None,
            Box::new(
                // move |samples: Result<Array2<Sample>>, sample_rate, nchannels| {
                move || {
                    // todo: send to all subscribed analyzers
                    let data = Array2::<crate::Sample>::zeros((10, 10));
                    // println!("got samples: {:?}", samples);
                    Ok(data)
                    // if let Err(err) = rec_tx.send((samples, sample_rate, nchannels)) {
                    //     panic!("{}", err);
                    // }
                },
            ),
        );

        if output_streams.contains_key(&descriptor) {
            return Err(Status::internal("failed to create audio stream descriptor"));
        } else {
            output_streams.insert(descriptor, RwLock::new(audio_output))
        };
        Ok(Response::new(proto::grpc::AudioOutputStream {}))
    }

    // async fn start_analysis(
    //     &self,
    //     request: Request<StartAnalysisRequest>,
    // ) -> Result<Response<Empty>, Status> {
    //     let (session_token, instance_id) = Self::extract_session_instance(&request)?;
    // let audio_backend = &self.state.read().await.audio_backend;
    // let sessions = self.sessions.read().await;
    // sessions.get(

    // match self.state.read().await.connected.get(&session_token) {
    //     Some(user) => {
    //         // check if the user is already running an analysis first
    //         if user.read().await.is_analyzing {
    //             println!("already running an analysis");
    //             // or use status already exists?
    //             return Err(Status::ok(format!(
    //                 "user {} is alreay running an analysis",
    //                 session_token
    //             )));
    //         }
    //         // this will not block
    //         if let Err(err) = user
    //             .write()
    //             .await
    //             .start_analysis::<f32, _>(&audio_backend)
    //             .await
    //         {
    //             eprintln!("failed to start the analysis: {}", err);
    //             return Err(Status::ok(format!("failed to start the analysis")));
    //         }

    //         Ok(Response::new(Empty {}))
    //     }
    //     None => Err(Status::not_found(format!(
    //         "user {} is not (yet) connected",
    //         user_token
    //     ))),
    // }
    // Ok(Response::new(Empty {}))
    // }
}
