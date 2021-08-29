use crate::analyzer::{AudioAnalyzerNode, AudioAnalyzerNodeTrait, AudioInputNode, AudioOutputNode};
use crate::cli::Config;
use crate::{ControllerUpdateMsg, EffectRack, ViewerUpdateMsg};
use analysis::bpm::{BpmDetectionAnalyzer, BpmDetectionAnalyzerConfig};
#[cfg(feature = "analyze")]
use analysis::spectral::{SpectralAnalyzer, SpectralAnalyzerOptions};
use anyhow::Result;
use futures::{Stream};
use hardware::led;
use num::traits::{NumCast};
#[cfg(feature = "portaudio")]
use recorder::portaudio::PortaudioRecorder;
#[cfg(feature = "record")]
// todo: simplify the required traits by having trait inheritance
use recorder::{
    AudioInputConfig,
    AudioInputNode as AudioInputNodeTrait, AudioNode, AudioOutputConfig,
    AudioOutputNode as AudioOutputNodeTrait, 
};
use ringbuf::RingBuffer;
use std::marker::Send;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;
use std::thread;
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct Controller<CU> {
    config: Config,
    connection: Arc<RwLock<mpsc::Sender<CU>>>,
}

impl<CU> Controller<CU> {
    pub fn new(config: Config, connection: mpsc::Sender<CU>) -> Self {
        Self {
            config,
            connection: Arc::new(RwLock::new(connection)),
        }
    }
}

pub fn map(value: f32, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    (value - x1) * (y2 - x2) / (y1 - x1) + x2
}

#[tonic::async_trait]
impl proto::grpc::remote_controller_server::RemoteController
    for EffectRack<ViewerUpdateMsg, ControllerUpdateMsg>
{
    type ConnectStream = Pin<
        Box<
            dyn Stream<Item = Result<proto::grpc::ControllerUpdate, Status>>
                + Send
                + Sync
                + 'static,
        >,
    >;

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
        _request: Request<proto::grpc::ControllerConnectRequest>,
    ) -> Result<Response<Self::ConnectStream>, Status> {
        let (_stream_tx, stream_rx) = mpsc::channel(1);
        let pinned_stream = Box::pin(ReceiverStream::new(stream_rx));

        // TODO
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
        audio_input
            .start()
            .await
            .map_err(|_| Status::internal("failed to start input stream"))?;

        if input_streams.contains_key(&descriptor) {
            return Err(Status::ok("the specified input stream already exists"));
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
        let mut audio_output = AudioOutputNode::<crate::Sample>::new(input_node, output_config)
            .map_err(|_| Status::internal("failed to create input stream"))?;
        let descriptor = audio_output.descriptor();
        audio_output
            .start()
            .await
            .map_err(|_| Status::internal("failed to start output stream"))?;

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

        let requested_analyzer = request.analyzer.map(|a| a.analyzer);
        let requested_analyzer =
            requested_analyzer.ok_or(Status::invalid_argument("missing audio analyzer"))?;

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
        let audio_analyzer_node = match requested_analyzer {
            Some(proto::audio::analysis::audio_analyzer::Analyzer::Spectral(_spectral_analyzer)) => {
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
                        .map_err(|_| Status::internal("failed to create spectral analyzer"))?,
                );
                AudioAnalyzerNode::<crate::Sample>::new(input_node, analyzer)
                    .map_err(|_| Status::internal("failed to create analyzer"))
            }
            Some(proto::audio::analysis::audio_analyzer::Analyzer::Bpm(_bpm_analyzer)) => {
                let analyzer_opts = BpmDetectionAnalyzerConfig {
                    // window_size: buffer_window_size,
                    // mel_bands: 24,
                    // nchannels: nchannels,
                    // sample_rate: sample_rate,
                    // fps: 60,
                    ..BpmDetectionAnalyzerConfig::default()
                };
                let analyzer = Box::new(
                    BpmDetectionAnalyzer::new(analyzer_opts)
                        .map_err(|_| Status::internal("failed to create bpm analyzer"))?,
                );
                AudioAnalyzerNode::<crate::Sample>::new(input_node, analyzer)
                    .map_err(|_| Status::internal("failed to create analyzer"))
            }
            None => Err(Status::invalid_argument("missing analyzer")),
        }?;

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

    async fn connect_lights_to_audio_analyzer(
        &self,
        request: Request<proto::grpc::ConnectLightsToAudioAnalyzerRequest>,
    ) -> Result<Response<proto::grpc::InstanceSubscriptions>, Status> {
        let (session_token, controller_instance_id) =
            Self::extract_session_instance(&request).await?;
        let request = request.into_inner();
        let lights = request
            .lights
            .ok_or(Status::invalid_argument("missing lights"))?;
        if lights.serial_port.len() < 1 {
            return Err(Status::invalid_argument(
                "missing light serial connection port",
            ));
        }
        if lights.strips.len() < 1 {
            return Err(Status::invalid_argument("no light strips"));
        }
        let _min_led_count = lights
            .strips
            .iter()
            .fold(0, |acc, strip| acc.min(strip.num_lights));

        let analyzer_desc = request.audio_analyzer_descriptor;
        let analyzer_desc = analyzer_desc.ok_or(Status::invalid_argument(
            "missing audio analyzer descriptor",
        ))?;

        println!(
            "[{}] connect leds to analyzer {} in session {}",
            controller_instance_id, analyzer_desc, session_token,
        );
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
        if lights.serial_port.len() < 1 {
            return Err(Status::invalid_argument(
                "missing light serial connection port",
            ));
        }
        if lights.strips.len() < 1 {
            return Err(Status::invalid_argument("no light strips"));
        }

        println!("light serial port: {}", lights.serial_port);
        println!("num light strips: {}", lights.strips.len());

        let latency = 30;
        let ring = RingBuffer::<(u8, u8, u8)>::new(latency);
        let (mut producer, mut consumer) = ring.split();

        for _ in 0..latency {
            let _ = producer.push((0, 0, 0));
        }

        thread::spawn(move || {
            let mut controller = match led::LEDSerialController::new(lights, led::ARDUINO_SETTINGS)
            {
                Ok(controller) => controller,
                Err(err) => {
                    println!("error: {}", err);
                    return;
                }
            };
            if let Err(err) = controller.connect() {
                println!("connect failed: {}", err);
            };

            if let Err(err) = controller.configure() {
                println!("configure failed: {}", err);
            };
            loop {
                if let Some(color) = consumer.pop() {
                    if let Err(err) = controller.update_color(color) {
                        println!("failed to update color: {}", err);
                    }
                }
            }
        });

        let mut rx = analyzer.connect();
        tokio::task::spawn(async move {
            loop {
                match rx.recv().await {
                    Ok(Ok(result)) => {
                        match result.result {
                            Some(
                                proto::audio::analysis::audio_analysis_result::Result::Spectral(
                                    spectral,
                                ),
                            ) => {
                                let volume = spectral.volume;
                                let split_idx: f32 =
                                    NumCast::from(spectral.mel_bands.len()).unwrap();
                                let split_idx = (split_idx / 3.0).ceil();
                                let split_idx: usize = NumCast::from(split_idx).unwrap();
                                let mut r = spectral.mel_bands[0..split_idx]
                                    .iter()
                                    .fold(f32::MIN, |acc, band| acc.max(*band));
                                let mut g = spectral.mel_bands[split_idx..2 * split_idx]
                                    .iter()
                                    .fold(f32::MIN, |acc, band| acc.max(*band));
                                let mut b = spectral.mel_bands[2 * split_idx..3 * split_idx]
                                    .iter()
                                    .fold(f32::MIN, |acc, band| acc.max(*band));

                                let min_volume_threshold = 1e-2;
                                let intensity =
                                    map(volume, min_volume_threshold, 0.8, 0.0, 1.0).powf(2.0);
                                r *= intensity * 255.0;
                                g *= intensity * 255.0;
                                b *= intensity * 255.0;

                                // todo: compute the speed param here
                                let r: u8 = NumCast::from(r).unwrap_or(0);
                                let g: u8 = NumCast::from(g).unwrap_or(0);
                                let b: u8 = NumCast::from(b).unwrap_or(0);
                                let color = (r, g, b);
                                if let Err(err) = producer.push(color) {
                                    eprintln!("failed to produce: {:?}", err);
                                }
                            }
                            _ => {}
                        }
                    }
                    Ok(Err(err)) => {
                        eprintln!("output receive error: {:?}", err);
                    }
                    Err(err) => {
                        eprintln!("output receive error: {:?}", err);
                    }
                }
            }
        });

        Ok(Response::new(proto::grpc::InstanceSubscriptions {}))
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

        println!(
            "[{}] subscribe to analyzer: {} {}",
            controller_instance_id, session_token, instance_id
        );
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
                            Err(err) => {
                                eprintln!(
                                    "[{}] failed to send update to viewer {} in session {}: {}",
                                    controller_instance_id, instance_id, session_token, err
                                );
                            }
                        }
                    }
                    Ok(Err(_)) => {
                        // ignore errors of the audio analyzer for now
                    }
                    Err(err) => {
                        eprintln!(
                            "[{}] failed to receive update in session {}: {}",
                            controller_instance_id, session_token, err
                        );
                    }
                }
            }
        });
        Ok(Response::new(proto::grpc::InstanceSubscriptions {}))
    }
}
