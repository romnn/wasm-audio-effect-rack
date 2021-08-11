use crate::analyzer::{AudioAnalyzerNode, AudioAnalyzerNodeTrait, AudioInputNode, AudioOutputNode};
use crate::cli::Config;
use crate::{ControllerUpdateMsg, EffectRack, MyCustomError, ViewerUpdateMsg};
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
// todo: simplify the required traits by having trait inheritance
use recorder::{
    cpal::CpalAudioBackend, cpal::CpalAudioInput, AudioInput, AudioInputConfig,
    AudioInputNode as AudioInputNodeTrait, AudioNode, AudioOutput, AudioOutputConfig,
    AudioOutputNode as AudioOutputNodeTrait, Sample,
};

// use recorder::{backend::portaudio::PortaudioAudioBackend, portaudio::PortaudioRecorder};

use std::collections::HashMap;
use std::error::Error;
use std::marker::Send;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::ops::Deref;
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
impl proto::grpc::remote_controller_server::RemoteController
    for EffectRack<ViewerUpdateMsg, ControllerUpdateMsg>
// where
// B: AudioBackend + Clone + Sync + Send + 'static,
// VU: Clone + Send + Sync + 'static,
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
        let descriptor = audio_input.descriptor();
        // let descriptor = audio_input
        //     .input
        //     .descriptor()
        //     .map_err(|_| Status::internal("failed to create audio stream descriptor"))?;
        audio_input
            .start()
            .await
            .map_err(|_| Status::internal("failed to start input stream"))?;
        // audio_input.stream_from_input(Box::new(
        //     move |samples: Result<Array2<f32>>, sample_rate, nchannels| {
        //         // todo: send to all subscribed analyzers
        //         println!("got samples: {:?}", samples);
        //         // if let Err(err) = rec_tx.send((samples, sample_rate, nchannels)) {
        //         //     panic!("{}", err);
        //         // }
        //     },
        // ));

        if input_streams.contains_key(&descriptor) {
            return Err(Status::internal(
                "failed to create audio input stream descriptor",
            ));
        } else {
            input_streams.insert(descriptor.clone(), RwLock::new(audio_input))
        };
        Ok(Response::new(proto::grpc::AudioInputStream {
            descriptor: Some(descriptor),
        }))
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
        let session = sessions
            .get(&session_token)
            .ok_or(Status::not_found(format!(
                "session {} does not exist",
                session_token
            )))?;
        let mut output_streams = session.output_streams.write().await;

        let input_stream_desc = request.into_inner().input_descriptor;
        let input_stream_desc =
            input_stream_desc.ok_or(Status::invalid_argument("missing input stream descriptor"))?;

        // todo: check if the stream is already active
        // todo: this requires differet descriptors based on in/out

        // todo: find the right input stream to connect it with
        let input_streams = session.input_streams.read().await;
        // let input_stream: recorder::AudioInputNode<_> = input_streams
        // let input_stream: recorder::AudioInputNode<_> = input_streams
        let input_stream = input_streams
            .get(&input_stream_desc)
            .ok_or(Status::not_found(format!(
                "no input {} found",
                input_stream_desc
            )))?
            .read()
            .await;
        // let test: &dyn recorder::AudioInputNode<_> = input_stream.deref();
        let input_node: &(dyn recorder::AudioInputNode<_> + Sync) = input_stream.deref();
        // .await as &dyn Deref<Target = recorder::AudioInputNode<_>>;
        // .deref();
        // .tx
        // .subscribe();

        // let mut audio_output: AudioOutputNode<crate::Sample> =
        let mut audio_output = AudioOutputNode::<crate::Sample>::new(input_node, output_config)
            .map_err(|_| Status::internal("failed to create input stream"))?;
        let descriptor = audio_output.descriptor();
        // let descriptor = audio_output
        //     .output
        //     .descriptor()
        //     .map_err(|_| Status::internal("failed to create audio output stream descriptor"))?;
        audio_output
            .start()
            .await
            .map_err(|_| Status::internal("failed to start output stream"))?;
        // audio_output.stream_to_output(
        //     // None,
        //     Box::new(
        //         // move |samples: Result<Array2<Sample>>, sample_rate, nchannels| {
        //         move || {
        //             // todo: send to all subscribed analyzers
        //             let data = Array2::<crate::Sample>::zeros((10, 10));
        //             // println!("got samples: {:?}", samples);
        //             Ok(data)
        //             // if let Err(err) = rec_tx.send((samples, sample_rate, nchannels)) {
        //             //     panic!("{}", err);
        //             // }
        //         },
        //     ),
        // );

        if output_streams.contains_key(&descriptor) {
            return Err(Status::internal("failed to create audio stream descriptor"));
        } else {
            output_streams.insert(descriptor.clone(), RwLock::new(audio_output))
        };
        Ok(Response::new(proto::grpc::AudioOutputStream {
            descriptor: Some(descriptor),
        }))
    }

    async fn add_audio_analyzer(
        &self,
        request: Request<proto::grpc::AddAudioAnalyzerRequest>,
    ) -> Result<Response<proto::grpc::AudioAnalyzer>, Status> {
        let (session_token, instance_id) = Self::extract_session_instance(&request).await?;
        let request = request.into_inner();
        println!("new analyzer: {} {}", session_token, instance_id);

        let input_stream_desc = request.input_descriptor;
        let input_stream_desc =
            input_stream_desc.ok_or(Status::invalid_argument("missing input stream descriptor"))?;

        let sessions = self.sessions.read().await;
        let session = sessions
            .get(&session_token)
            .ok_or(Status::not_found(format!(
                "session {} does not exist",
                session_token
            )))?;

        // get the input node
        let input_streams = session.input_streams.read().await;
        let input_stream = input_streams
            .get(&input_stream_desc)
            .ok_or(Status::not_found(format!(
                "no input {} found",
                input_stream_desc
            )))?
            .read()
            .await;
        let input_node: &(dyn recorder::AudioInputNode<_> + Sync) = input_stream.deref();

        // create the analyzer
        let analyzer_opts = SpectralAnalyzerOptions {
            // window_size: buffer_window_size,
            mel_bands: 24,
            // nchannels: nchannels,
            // sample_rate: sample_rate,
            fps: 60,
            ..SpectralAnalyzerOptions::default()
        };
        // let analyzer: &(dyn analysis::Analyzer<_> + Send + Sync) =
        let analyzer = Box::new(
            SpectralAnalyzer::<crate::Sample>::new(analyzer_opts)
                .map_err(|_| Status::internal("failed to create analyzer"))?,
        );
        let audio_analyzer_node = AudioAnalyzerNode::<crate::Sample>::new(input_node, analyzer)
            .map_err(|_| Status::internal("failed to create analyzer"))?;
        let descriptor = audio_analyzer_node.descriptor();

        // check if the analyzer already exists
        let mut analyzers = session.analyzers.write().await;
        if analyzers.contains_key(&descriptor) {
            return Err(Status::already_exists("audio analyzer already exists"));
        } else {
            // start the analyzer
            audio_analyzer_node
                .start()
                .await
                .map_err(|_| Status::internal("failed to start analyzer"))?;

            analyzers.insert(descriptor.clone(), RwLock::new(audio_analyzer_node))
        };
        Ok(Response::new(proto::grpc::AudioAnalyzer {
            descriptor: Some(descriptor),
        }))
    }

    async fn subscribe_to_audio_analyzer(
        &self,
        request: Request<proto::grpc::SubscribeToAudioAnalyzerRequest>,
    ) -> Result<Response<proto::grpc::InstanceSubscriptions>, Status> {
        let (session_token, controller_instance_id) =
            Self::extract_session_instance(&request).await?;
        let request = request.into_inner();
        let instance_id = request
            .instance_id
            .ok_or(Status::invalid_argument("missing instance id"))?;
        let analyzer_desc = request.audio_analyzer_descriptor;
        let analyzer_desc = analyzer_desc.ok_or(Status::invalid_argument(
            "missing audio analyzer descriptor",
        ))?;

        println!("subscribe to analyzer: {} {}", session_token, instance_id);
        let sessions = self.sessions.read().await;
        let session = sessions
            .get(&session_token)
            .ok_or(Status::not_found(format!(
                "session {} does not exist",
                session_token
            )))?;

        let analyzers = session.analyzers.read().await;
        let analyzer = analyzers
            .get(&analyzer_desc)
            .ok_or(Status::not_found(format!(
                "no audio analyzer {} found",
                analyzer_desc
            )))?
            .read()
            .await;

        let viewers = session.viewers.read().await;
        let viewer = viewers
            .get(&instance_id)
            .ok_or(Status::not_found(format!(
                "no viewer instance {} found",
                instance_id
            )))?
            .read()
            .await;

        // spawn a tokio task that waits for updates and sends them to the viewer
        let mut rx = analyzer.connect();
        let viewer_tx = viewer.tx.clone();
        // let viewers = viewers.clone();
        // let viewer = viewer.clone();
        tokio::task::spawn(async move {
            let mut seq_num = 0;
            loop {
                match rx.recv().await {
                    Ok(Ok(mut result)) => {
                        result.seq_num = seq_num;
                        let update = proto::grpc::ViewerUpdate {
                            update: Some(proto::grpc::viewer_update::Update::AudioAnalysisResult(
                                result,
                            )),
                        };
                        match viewer_tx.send(Ok(update)).await {
                            Ok(()) => {
                                seq_num = seq_num + 1;
                            }
                            Err(err) => {}
                        }

                        // match viewers.get(&instance_id) {
                        //     Some(viewer) => match viewer.read().await.tx.send(Ok(update)).await {
                        //         Ok(()) => {
                        //             seq_num = seq_num + 1;
                        //         }
                        //         Err(err) => {}
                        //     },
                        //     None => {
                        //         return;
                        //     }
                        // }
                    }
                    Ok(Err(err)) => {}
                    Err(err) => {}
                }
            }
        });

        // check if the analyzer already exists
        // get a receiver handle of the analyzer
        // let mut analyzers = session.analyzers.write().await;
        // if analyzers.contains_key(&descriptor) {
        //     return Err(Status::already_exists("audio analyzer already exists"));
        // } else {
        //     analyzers.insert(descriptor.clone(), RwLock::new(audio_analyzer_node))
        // };

        // let analyzer_node: &(dyn analysis::Analyzer<_> + Sync) = analyzer.deref();
        // let analyzer_node = analyzer.deref();

        Ok(Response::new(proto::grpc::InstanceSubscriptions {}))
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
