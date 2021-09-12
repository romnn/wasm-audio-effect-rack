use anyhow::Result;
use async_trait::async_trait;
use ndarray::prelude::*;
use num::traits::{NumCast, ToPrimitive, Zero};
use rodio;
use std::error;
use std::fmt;
use std::marker::Send;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};

pub mod cpal;
pub mod portaudio;
pub trait Sample:
    rodio::Sample + NumCast + std::fmt::Debug + Clone + Zero + ToPrimitive + Send + Sync + 'static
{
}

impl Sample for f32 {}
impl Sample for i16 {}
impl Sample for u16 {}

#[derive(Clone, Debug)]
pub struct AudioBackendConfig {
    #[cfg(use_jack)]
    pub use_jack: bool,
    #[cfg(use_portaudio)]
    pub use_portaudio: bool,
}

impl Default for AudioBackendConfig {
    fn default() -> Self {
        Self {
            #[cfg(use_jack)]
            use_jack: false,
            #[cfg(use_portaudio)]
            use_portaudio: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct AudioInputConfig {
    pub backend_config: AudioBackendConfig,
    pub input_device: Option<String>,
    pub output_device: Option<String>,
}

impl Default for AudioInputConfig {
    fn default() -> Self {
        Self {
            backend_config: AudioBackendConfig::default(),
            input_device: None,
            output_device: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct AudioOutputConfig {
    pub backend_config: AudioBackendConfig,
    pub input_device: Option<String>,
    pub output_device: Option<String>,
    /// latency in milliseconds
    pub latency: f32,
}

impl Default for AudioOutputConfig {
    fn default() -> Self {
        Self {
            backend_config: AudioBackendConfig::default(),
            input_device: None,
            output_device: None,
            latency: 400.0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AudioError {
    Unknown(String),
    DeviceNotFound(String),
    DeviceNotAvailable(String),
}

impl fmt::Display for AudioError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Unknown(msg) => {
                write!(f, "audio error: {}", msg)
            }
            Self::DeviceNotFound(device_name) => {
                write!(f, "device \"{}\" was not found", device_name)
            }
            Self::DeviceNotAvailable(device_name) => {
                write!(f, "device \"{}\" is not available", device_name)
            }
        }
    }
}

impl error::Error for AudioError {}

#[derive(Debug, Clone)]
pub enum AudioAnalysisError {
    Unknown(String),
}

impl fmt::Display for AudioAnalysisError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Unknown(msg) => {
                write!(f, "analysis error: {}", msg)
            }
        }
    }
}

impl error::Error for AudioAnalysisError {}

pub type AudioBuffer<T> = (Result<Array2<T>, AudioError>, u32, u16);
pub type AudioBufferReceiver<T> = Arc<Mutex<broadcast::Receiver<AudioBuffer<T>>>>;
pub type AudioBufferSender<T> = broadcast::Sender<AudioBuffer<T>>;

type AudioAnalysisResult = Result<proto::audio::analysis::AudioAnalysisResult, AudioAnalysisError>;
pub type AudioAnalysisResultReceiver = broadcast::Receiver<AudioAnalysisResult>;
pub type AudioAnalysisResultSender = broadcast::Sender<AudioAnalysisResult>;

pub type AudioInputCallback<T> =
    Box<dyn FnMut(Result<Array2<T>>, u32, u16) -> () + Send + Sync + 'static>;

pub type AudioOutputCallback<T> = Box<dyn FnMut() -> Option<T> + Send + Sync + 'static>;

#[derive(Clone, Copy, Debug, Default)]
pub struct AudioStreamInfo {
    pub sample_rate: u32,
    // pub buffer_size: usize,
    pub nchannels: u16,
}

pub trait AudioBackend<T> {
    // todo: use a proto to list all the available inputs
    fn query(&self) -> Result<()>;
    fn get_file_info(&self, path: PathBuf) -> Result<(u32, u16)>;
}

pub trait AudioOutput<T>
where
    T: Sample,
{
    fn new(config: AudioOutputConfig) -> Result<Self>
    where
        Self: Sized;

    fn descriptor(&self) -> Result<proto::grpc::AudioOutputDescriptor>;
    fn input_stream_params(&self) -> AudioStreamInfo;
    fn output_stream_params(&self) -> AudioStreamInfo;

    fn stream_to_output(&mut self, callback: AudioOutputCallback<T>) -> Result<()>;
}

#[async_trait]
pub trait AudioNode<T>
where
    T: Sample,
{
    async fn start(&mut self) -> Result<()>;
}

pub trait AudioOutputNode<T>: AudioNode<T>
where
    T: Sample,
{
    fn new(input_node: &dyn AudioInputNode<T>, config: AudioOutputConfig) -> Result<Self>
    where
        Self: Sized;

    fn descriptor(&self) -> proto::grpc::AudioOutputDescriptor;
    fn input_stream_params(&self) -> AudioStreamInfo;
    fn output_stream_params(&self) -> AudioStreamInfo;
}

pub trait AudioInput<T>
where
    T: Sample,
{
    fn new(config: AudioInputConfig) -> Result<Self>
    where
        Self: Sized;

    fn stream_from_input(&self, callback: AudioInputCallback<T>) -> Result<()>;

    fn descriptor(&self) -> Result<proto::grpc::AudioInputDescriptor>;
    fn input_stream_params(&self) -> AudioStreamInfo;
}

pub trait AudioInputNode<T>: AudioNode<T>
where
    T: Sample,
{
    fn new(config: AudioInputConfig) -> Result<Self>
    where
        Self: Sized;

    fn descriptor(&self) -> proto::grpc::AudioInputDescriptor;
    fn input_stream_params(&self) -> AudioStreamInfo;
    fn connect(&self) -> AudioBufferReceiver<T>;
}
