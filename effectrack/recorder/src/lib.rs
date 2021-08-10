use anyhow::Result;
use ndarray::parallel::prelude::*;
use ndarray::prelude::*;
use ndarray::{
    concatenate, indices, Array, IntoDimension, Ix, NdIndex, RemoveAxis, ScalarOperand, Slice, Zip,
};
use num::traits::{Float, FloatConst, NumCast, ToPrimitive, Zero};
use rodio;
use std::convert::From;
use std::marker::Send;
use std::path::PathBuf;
use ::cpal::{StreamConfig};

pub mod cpal;
pub mod portaudio;
// pub mod backend {
//     pub mod cpal;
//     pub mod portaudio;
// }

// pub trait Sample: rodio::Sample + Zero + Float + FloatConst + Send {}
pub trait Sample: rodio::Sample + Zero + ToPrimitive + Send {}

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

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub enum AudioStreamKind
// where
// Self: std::hash::Hash + std::cmp::Eq,
// Self: std::hash::Hash,
{
    INPUT,
    OUTPUT,
}

// impl std::cmp::PartialEq for AudioStreamKind {
//     fn eq(&self, other: &AudioStreamDescriptor) -> bool {
//         self.x == other.x
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AudioStreamDescriptor
// where
//     // Self: std::hash::Hash + std::cmp::Eq,
//     Self: std::hash::Hash,
{
    pub kind: AudioStreamKind,
    pub device: String,
    pub host: String,
}

// pub type CallbackMut<T> = ;
// pub type Callback<T> = Box<dyn FnMut(Result<Array2<T>>, u32, u16) -> () + Send + Sync + 'static>;
pub type AudioInputCallback<T> =
    Box<dyn FnMut(Result<Array2<T>>, u32, u16) -> () + Send + Sync + 'static>;

pub type AudioOutputCallback<T> =
    Box<dyn FnMut() -> Result<Array2<T>> + Send + Sync + 'static>;

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

    // fn input_name(&self) -> Result<String>;

    // fn output_name(&self) -> Result<String>;

    // fn descriptor(&self) -> Result<AudioStreamDescriptor>;
}

pub trait AudioOutput<T>
where
    // F: Fn(Result<Array2<T>>, u32, u16) -> () + Send + 'static,
    T: NumCast + Clone + Zero + Send + Sync + 'static,
    // T: NumCast + Clone + Zero + Send + 'static,
    // T: NumCast + Zero + Send + 'static,
{
    fn stream_to_output(
        &mut self,
        // input_config: Option<StreamConfig>,
        // playback: bool,
        // callback: &'static (dyn Fn(Result<Array2<T>>, u32, u16) -> () + Send + Sync),
        callback: AudioOutputCallback<T>,
    ) -> Result<()>;

    fn new(config: AudioOutputConfig) -> Result<Self>
    where
        Self: Sized;

    fn descriptor(&self) -> Result<AudioStreamDescriptor>;
}

pub trait AudioInput<T>
where
    // F: Fn(Result<Array2<T>>, u32, u16) -> () + Send + 'static,
    T: NumCast + Clone + Zero + Send + Sync + 'static,
    // T: NumCast + Clone + Zero + Send + 'static,
{
    // fn new(config: AudioInputConfig) -> Result<Self>
    fn new(config: AudioInputConfig) -> Result<Self>
    where
        Self: Sized;

    fn descriptor(&self) -> Result<AudioStreamDescriptor>;

    fn stream_from_input(
        &mut self,
        // playback: bool,
        // callback: &'static (dyn Fn(Result<Array2<T>>, u32, u16) -> () + Send + Sync),
        callback: AudioInputCallback<T>,
    ) -> Result<()>;
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
