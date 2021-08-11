use ::cpal::{Stream, StreamConfig};
use anyhow::Result;
use async_trait::async_trait;
use ndarray::parallel::prelude::*;
use ndarray::prelude::*;
use ndarray::{
    concatenate, indices, Array, IntoDimension, Ix, NdIndex, RemoveAxis, ScalarOperand, Slice, Zip,
};
use num::traits::{Float, FloatConst, NumCast, ToPrimitive, Zero};
use rodio;
use std::convert::From;
use std::error;
use std::fmt;
use std::marker::Send;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc, oneshot, watch, Mutex, RwLock};

pub mod cpal;
pub mod portaudio;
// pub mod backend {
//     pub mod cpal;
//     pub mod portaudio;
// }

// pub trait Sample: rodio::Sample + Zero + Float + FloatConst + Send {}

// NumCast + Clone + Zero + Send + Sync + 'static
// pub trait Sample: NumCast + Clone + Zero + Send + Sync + 'static {}
// pub trait Sample: rodio::Sample + Zero + ToPrimitive + Send {}
pub trait Sample:
    rodio::Sample + NumCast + std::fmt::Debug + Clone + Zero + ToPrimitive + Send + Sync + 'static
{
}

// rodio does not support any other float other than f32 but maybe others
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
            // monitor_input: None,
            // latency: 400.0,
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
            // monitor_input: None,
            latency: 400.0,
        }
    }
}

// #[derive(Eq, PartialEq, Debug, Clone, Hash)]
// pub enum AudioStreamKind
// // where
// // Self: std::hash::Hash + std::cmp::Eq,
// // Self: std::hash::Hash,
// {
//     INPUT,
//     OUTPUT,
// }

// impl std::cmp::PartialEq for AudioStreamKind {
//     fn eq(&self, other: &AudioStreamDescriptor) -> bool {
//         self.x == other.x
//     }
// }

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub struct AudioStreamDescriptor
// // where
// //     // Self: std::hash::Hash + std::cmp::Eq,
// //     Self: std::hash::Hash,
// {
//     pub kind: AudioStreamKind,
//     pub device: String,
//     pub host: String,
// }

// pub type CallbackMut<T> = ;
// pub type Callback<T> = Box<dyn FnMut(Result<Array2<T>>, u32, u16) -> () + Send + Sync + 'static>;
#[derive(Debug, Clone)]
pub enum AudioError {
    Unknown(String),
    DeviceNotFound(String),
    DeviceNotAvailable(String),
    // DeviceNotAvailable(String),
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
    // DeviceNotFound(String),
    // DeviceNotAvailable(String),
    // DeviceNotAvailable(String),
}

impl fmt::Display for AudioAnalysisError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Unknown(msg) => {
                write!(f, "analysis error: {}", msg)
            } // Self::DeviceNotFound(device_name) => {
              //     write!(f, "device \"{}\" was not found", device_name)
              // }
              // Self::DeviceNotAvailable(device_name) => {
              //     write!(f, "device \"{}\" is not available", device_name)
              // }
        }
    }
}

impl error::Error for AudioAnalysisError {}

// pub type AudioBuffer<T> = (Result<Array2<T>>, u32, u16);
pub type AudioBuffer<T> = (Result<Array2<T>, AudioError>, u32, u16);
// pub type AudioBufferReceiver<T> = broadcast::Receiver<AudioBuffer<T>>;
pub type AudioBufferReceiver<T> = Arc<Mutex<broadcast::Receiver<AudioBuffer<T>>>>;
// pub type AudioBufferSender<T> = Arc<broadcast::Sender<AudioBuffer<T>>>;
pub type AudioBufferSender<T> = broadcast::Sender<AudioBuffer<T>>;

type AudioAnalysisResult = Result<proto::audio::analysis::AudioAnalysisResult, AudioAnalysisError>;
pub type AudioAnalysisResultReceiver = broadcast::Receiver<AudioAnalysisResult>;
pub type AudioAnalysisResultSender = broadcast::Sender<AudioAnalysisResult>;

pub type AudioInputCallback<T> =
    Box<dyn FnMut(Result<Array2<T>>, u32, u16) -> () + Send + Sync + 'static>;

// pub type AudioOutputCallback<T> = Box<dyn FnMut() -> Result<Array2<T>> + Send + Sync + 'static>;
pub type AudioOutputCallback<T> = Box<dyn FnMut() -> Option<T> + Send + Sync + 'static>;

#[derive(Clone, Copy, Debug, Default)]
pub struct AudioStreamInfo {
    pub sample_rate: u32,
    // pub buffer_size: usize,
    pub nchannels: u16,
}

// pub trait Recorder<T, F>
// pub trait AudioBackend<T>
pub trait AudioBackend<T>
// where
//     // F: Fn(Result<Array2<T>>, u32, u16) -> () + Send + 'static,
//     T: NumCast + Zero + Send + 'static,
{
    // todo: use a proto to list all the available inputs
    fn query(&self) -> Result<()>;
    fn get_file_info(&self, path: PathBuf) -> Result<(u32, u16)>;

    // fn new(config: RecorderConfig) -> Result<Self>
    // fn new(config: AudioBackendConfig) -> Result<Self>
    // where
    //     Self: Sized;

    // fn output_name(&self) -> Result<String>;

    // fn descriptor(&self) -> Result<AudioStreamDescriptor>;
}

pub trait AudioOutput<T>
where
    // F: Fn(Result<Array2<T>>, u32, u16) -> () + Send + 'static,
    T: Sample,
{
    // fn new(input_stream: AudioBufferReceiver<T>, config: AudioOutputConfig) -> Result<Self>

    fn new(
        // input_stream: &dyn std::ops::Deref<Target = AudioInputNode<T>>,
        // input_stream: &dyn AudioInputNode<T>,
        // input_stream: AudioInputNode<T>,
        config: AudioOutputConfig,
    ) -> Result<Self>
    where
        Self: Sized;

    fn descriptor(&self) -> Result<proto::grpc::AudioOutputDescriptor>;
    fn input_stream_params(&self) -> AudioStreamInfo;
    fn output_stream_params(&self) -> AudioStreamInfo;

    fn stream_to_output(
        &mut self,
        // input_config: Option<StreamConfig>,
        // playback: bool,
        // callback: &'static (dyn Fn(Result<Array2<T>>, u32, u16) -> () + Send + Sync),
        callback: AudioOutputCallback<T>,
    ) -> Result<()>;
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
    fn new(input_node: &AudioInputNode<T>, config: AudioOutputConfig) -> Result<Self>
    where
        Self: Sized;

    fn descriptor(&self) -> proto::grpc::AudioOutputDescriptor;
    fn input_stream_params(&self) -> AudioStreamInfo;
    fn output_stream_params(&self) -> AudioStreamInfo;
    // fn connect(&self) -> Result<AudioBufferReceiver<T>>;
}

pub trait AudioInput<T>
where
    // F: Fn(Result<Array2<T>>, u32, u16) -> () + Send + 'static,
    // T: NumCast + Clone + Zero + Send + Sync + 'static,
    T: Sample,
{
    // fn new(config: AudioInputConfig) -> Result<Self>
    fn new(config: AudioInputConfig) -> Result<Self>
    where
        Self: Sized;

    fn stream_from_input(
        &self,
        // playback: bool,
        // callback: &'static (dyn Fn(Result<Array2<T>>, u32, u16) -> () + Send + Sync),
        callback: AudioInputCallback<T>,
    ) -> Result<()>;

    fn descriptor(&self) -> Result<proto::grpc::AudioInputDescriptor>;
    fn input_stream_params(&self) -> AudioStreamInfo;
    // where
    // Self: Sized;
    // where
    //     F: Fn(Result<Array2<T>>, u32, u16) -> () + Send + 'static,
    //     T: NumCast + Zero + Send + Clone + 'static;

    // fn stream_from_file(
    //     &self,
    //     path: PathBuf,
    //     // callback: &'static (dyn Fn(Result<Array2<T>>, u32, u16) -> () + Send + Sync),
    //     callback: Callback<T>,
    // ) -> Result<()>;
    // where
    //     Self: Sized;
}

pub trait AudioInputNode<T>: AudioNode<T>
where
    T: Sample,
{
    // fn new(
    //     config: AudioOutputConfig,
    // ) -> Result<Self>
    // where
    //     Self: Sized;
    fn new(config: AudioInputConfig) -> Result<Self>
    where
        Self: Sized;

    // fn start(&self) -> Result<()>;
    fn descriptor(&self) -> proto::grpc::AudioInputDescriptor;
    fn input_stream_params(&self) -> AudioStreamInfo;
    fn connect(&self) -> AudioBufferReceiver<T>;
}

// // pub trait Recorder<T, F>
// pub trait Recorder<T>
// where
//     // F: Fn(Result<Array2<T>>, u32, u16) -> () + Send + 'static,
//     T: NumCast + Zero + Send + 'static,
// {
//     fn new(config: RecorderConfig) -> Result<Self>
//     where
//         Self: Sized;

//     // todo: use a proto to list all the available inputs
//     fn query(&self) -> Result<()>;

//     fn input_name(&self) -> Result<String>;

//     fn output_name(&self) -> Result<String>;

//     fn descriptor(&self) -> Result<AudioStreamDescriptor>;

//     fn get_file_info(&self, path: PathBuf) -> Result<(u32, u16)>;

//     fn stream_output(
//         &self,
//         // playback: bool,
//         // callback: &'static (dyn Fn(Result<Array2<T>>, u32, u16) -> () + Send + Sync),
//         // callback: Callback<T>,
//     ) -> Result<()>;

//     fn stream_input(
//         &self,
//         // playback: bool,
//         // callback: &'static (dyn Fn(Result<Array2<T>>, u32, u16) -> () + Send + Sync),
//         callback: Callback<T>,
//     ) -> Result<()>;
//     // where
//     // Self: Sized;
//     // where
//     //     F: Fn(Result<Array2<T>>, u32, u16) -> () + Send + 'static,
//     //     T: NumCast + Zero + Send + Clone + 'static;

//     fn stream_file(
//         &self,
//         path: PathBuf,
//         // callback: &'static (dyn Fn(Result<Array2<T>>, u32, u16) -> () + Send + Sync),
//         callback: Callback<T>,
//     ) -> Result<()>;
//     // where
//     //     Self: Sized;
// }

// #[derive(Clone, Debug, Default)]
// pub struct AudioBackendConfig {
//     #[cfg(use_jack)]
//     pub jack: bool,
//     pub input_device: Option<String>,
//     pub output_device: Option<String>,
//     pub latency: f32,
// }

// pub trait AudioBackend {
//     type Rec: Recorder + Sync + Send;

//     fn new(config: AudioBackendConfig) -> Self
//     where
//         Self: Sized;
//     fn new_recorder(&self) -> Result<Self::Rec>;

//     // todo: use a proto to list all the available inputs
//     fn query(&self) -> Result<()>;
// }
