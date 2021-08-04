use anyhow::Result;
use ndarray::parallel::prelude::*;
use ndarray::prelude::*;
use ndarray::{
    concatenate, indices, Array, IntoDimension, Ix, NdIndex, RemoveAxis, ScalarOperand, Slice, Zip,
};
use num::traits::{Float, FloatConst, Zero};
use rodio;
use std::marker::Send;
use std::path::PathBuf;

pub mod cpal;
pub mod portaudio;
pub mod backend {
    pub mod cpal;
    pub mod portaudio;
}

// pub trait Sample: rodio::Sample + Zero + Float + FloatConst + Send + Sync {}
pub trait Sample: rodio::Sample + Zero + Float + FloatConst + Send {}

// rodio does not support any other float other than f32 but maybe others
impl Sample for f32 {}

//
// Sample: rodio::Sample + Zero + Float + Send + Sync {}
// + 'static {}

// pub trait Recorder
pub trait Recorder // where
// F: Fn(Array2<T>, u32, u16) -> (), // + Send + 'static,
// T: Float,
{
    // fn new(options: RecorderOptions) -> Result<Self>
    // where
    //     Self: Sized;
    // fn stream_file(&self, path: PathBuf) -> Result<()>;
    fn get_file_info(path: PathBuf) -> Result<(u32, u16)>;

    fn stream_file<T, F>(&self, path: PathBuf, callback: F) -> Result<(u32, u16)>
    where
        F: Fn(Result<Array2<T>>, u32, u16) -> () + Send + 'static,
        // S: Zero + rodio::Sample,
        T: Sample + 'static;

    fn stream_input(&self) -> Result<()>;
}

#[derive(Debug, Default)]
pub struct AudioBackendConfig {
    #[cfg(use_jack)]
    pub jack: bool,
    pub device: Option<String>,
}

pub trait AudioBackend
// where
//     F: Fn(Array2<T>, u32, u16) -> (), // + Send + 'static,
//     T: Float,
{
    // type Rec: Recorder<T,F> + Sync + Send;
    type Rec: Recorder + Sync + Send;

    fn new(config: AudioBackendConfig) -> Self
    where
        Self: Sized;
    fn new_recorder(&self) -> Result<Self::Rec>;
    // todo: use a proto to list all the available inputs
    fn query(&self) -> Result<()>;
}
