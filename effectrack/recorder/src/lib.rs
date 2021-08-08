use anyhow::Result;
use ndarray::parallel::prelude::*;
use ndarray::prelude::*;
use ndarray::{
    concatenate, indices, Array, IntoDimension, Ix, NdIndex, RemoveAxis, ScalarOperand, Slice, Zip,
};
use num::traits::{Float, FloatConst, NumCast, ToPrimitive, Zero};
use rodio;
use std::marker::Send;
use std::path::PathBuf;

pub mod cpal;
pub mod portaudio;
pub mod backend {
    pub mod cpal;
    pub mod portaudio;
}

// pub trait Sample: rodio::Sample + Zero + Float + FloatConst + Send {}
pub trait Sample: rodio::Sample + Zero + ToPrimitive + Send {}

// rodio does not support any other float other than f32 but maybe others
impl Sample for f32 {}
impl Sample for i16 {}
impl Sample for u16 {}

pub trait Recorder {
    fn get_file_info(path: PathBuf) -> Result<(u32, u16)>;

    fn stream_input<T, F>(&self, playback: bool, callback: F) -> Result<()>
    where
        F: Fn(Result<Array2<T>>, u32, u16) -> () + Send + 'static,
        T: NumCast + Zero + Send + Clone + 'static;

    fn stream_file<T, F>(&self, path: PathBuf, callback: F) -> Result<()>
    where
        F: Fn(Result<Array2<T>>, u32, u16) -> () + Send + 'static,
        T: NumCast + Zero + Send + 'static;
}

#[derive(Clone, Debug, Default)]
pub struct AudioBackendConfig {
    #[cfg(use_jack)]
    pub jack: bool,
    pub input_device: Option<String>,
    pub output_device: Option<String>,
    pub latency: f32,
}

pub trait AudioBackend {
    type Rec: Recorder + Sync + Send;

    fn new(config: AudioBackendConfig) -> Self
    where
        Self: Sized;
    fn new_recorder(&self) -> Result<Self::Rec>;

    // todo: use a proto to list all the available inputs
    fn query(&self) -> Result<()>;
}
