use crate::cli::{Config, Opts};
use crate::{ControllerUpdateMsg, EffectRack};
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
    cpal::CpalAudioInput, cpal::CpalAudioOutput, AudioBackendConfig, AudioInput,
    AudioInputCallback, AudioInputConfig, AudioOutput, AudioOutputCallback, AudioOutputConfig,
    AudioStreamDescriptor, Sample,
};

// use recorder::{cpal::CpalRecorder, AudioOutputConfig, Recorder, RecorderConfig, Sample};
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
// use cpal::{StreamConfig};

// #[derive()]
// pub struct AudioOutputStreamConfig<T>
// where
//     T: NumCast + Zero + Send + Clone + 'static,
// {
//     recorder: RecorderConfig,

//     thread: Vec<thread::JoinHandle<()>>,
//     is_running: bool,
// }

// pub type AudioInputConfig = RecorderConfig;
// pub type AudioOutputConfig = RecorderConfig;

// impl From<AudioBackendConfig> for Opts {
impl From<Opts> for AudioBackendConfig {
    fn from(options: Opts) -> AudioBackendConfig {
        AudioBackendConfig {
            #[cfg(use_jack)]
            use_jack: options.use_jack,
            #[cfg(use_portaudio)]
            use_portaudio: options.use_portaudio,
        }
    }
}

// impl From<AudioInputConfig> for Opts {
impl From<Opts> for AudioInputConfig {
    fn from(options: Opts) -> AudioInputConfig {
        AudioInputConfig {
            backend_config: AudioBackendConfig::from(options.clone()),
            input_device: options.input_device,
            output_device: options.output_device,
        }
    }
}

#[derive()]
pub struct AudioOutputNode<T>
where
    T: NumCast + Zero + Send + Clone + 'static,
    // Self: Sized,
{
    pub config: AudioOutputConfig,
    // pub recorder: Box<Recorder<T> + Send + Sync + std::marker::Send: Clone + std::marker::Send: Sized + 'static>,
    // pub recorder: Arc<Box<dyn Recorder<T> + Send + Sync + 'static>>,
    pub output: Arc<Box<dyn recorder::AudioOutput<T> + Send + Sync + 'static>>,
    // pub recorder: Box<NewTrait>,
    threads: Vec<thread::JoinHandle<()>>,
    pub is_running: bool,
}

// impl<T> AudioOutputNode<T>
impl<T> AudioOutput<T> for AudioOutputNode<T>
where
    T: NumCast + Zero + Send + Sync + Clone + 'static,
    // Self: Sized,
{
    fn new(config: AudioOutputConfig) -> Result<Self> {
        let audio = CpalAudioOutput::<T>::new(config.clone())?;
        cfg_if::cfg_if! {
            if #[cfg(feature = "portaudio")] {
                if config.use_portaudio{
                    let audio = PortaudioAudioOutput::<T>::new(config.clone())?;
                };
            }
        };
        Ok(Self {
            config,
            // recorder: Arc::new(Box::new(recorder)),
            output: Arc::new(Box::new(audio)),
            threads: Vec::new(),
            is_running: false,
        })
    }

    fn descriptor(&self) -> Result<AudioStreamDescriptor> {
        self.output.descriptor()
    }

    fn stream_to_output(
        &mut self,
        // input_config: Option<StreamConfig>,
        callback: AudioOutputCallback<T>,
    ) -> Result<()> {
        let output = self.output.clone();
        Ok(())
        // output.stream_to_output(
        // input_config: Option<StreamConfig>,
        // input_config: AudioInputConfig,
        // playback: bool,
        // mut callback: AudioOutputCallback<T>,
        // callback: &'static (dyn Fn(Result<Array2<T>>, u32, u16) -> () + Send + Sync),
        // callback: Callback<T>,
        // )
        // let builder = thread::Builder::new();
        // let config = self.config.clone();
        // let input = self.input.clone();
        // let audio_stream_thread = builder
        //     .name("audio input stream thread".to_string())
        //     .spawn(move || {
        //         println!(
        //             "streaming audio from input \"{}\"",
        //             "default" // recorder.input_name().unwrap_or("unknown".to_string())
        //         );
        //         if let Err(err) = input.stream_from_input(
        //             // config.monitor_input.unwrap_or(false),
        //             callback,
        //             // Box::new(move |samples: Result<Array2<T>>, sample_rate, nchannels| {
        //             // todo: send to all subscribed analyzers
        //             // if let Err(err) = rec_tx.send((samples, sample_rate, nchannels)) {
        //             //     panic!("{}", err);
        //             // }
        //             // }),
        //         ) {
        //             eprintln!("failed to stream input: {}", err);
        //         }
        //         println!("playback is over");
        //     })?;
        // self.threads.push(audio_stream_thread);
        // Ok(())
    }

    // todo: move the callback in here
    // pub fn stream_from_input(&mut self, callback: AudioInputCallback<T>) -> Result<()> {
}

// impl<T> AudioOutput<T>
// where
//     T: NumCast + Zero + Send + Sync + Clone + 'static,
//     // Self: Sized,
// {
//     pub fn new(config: AudioOutputConfig) -> Result<Self> {
//         let audio = CpalAudioOutput::<T>::new(config.clone())?;
//         cfg_if::cfg_if! {
//             if #[cfg(feature = "portaudio")] {
//                 if config.use_portaudio{
//                     let audio= PortaudioAudioBackend::<T>::new(config.clone())?;
//                 };
//             }
//         };
//         Ok(Self {
//             config,
//             // recorder: Arc::new(Box::new(recorder)),
//             recorder: Arc::new(Box::new(audio)),
//             threads: Vec::new(),
//             is_running: false,
//         })
//     }

//     pub fn stream_output(&mut self) -> Result<()> {
//         let builder = thread::Builder::new();
//         let config = self.config.clone();
//         let recorder = self.recorder.clone();
//         let audio_stream_thread = builder
//             .name("audio output stream thread".to_string())
//             .spawn(move || {
//                 println!(
//                     "streaming audio to output \"{}\"",
//                     recorder.output_name().unwrap_or("unknown".to_string())
//                 );
//                 // if let Err(err) = recorder.stream_output(
//                 //     // config.monitor_input.unwrap_or(false),
//                 //     Box::new(move |samples: Result<Array2<T>>, sample_rate, nchannels| {
//                 //         // todo: send to all subscribed analyzers
//                 //         // if let Err(err) = rec_tx.send((samples, sample_rate, nchannels)) {
//                 //         //     panic!("{}", err);
//                 //         // }
//                 //     }),
//                 // ) {
//                 //     eprintln!("failed to stream output: {}", err);
//                 // }
//                 println!("output stream ended");
//             })?;
//         self.threads.push(audio_stream_thread);
//         Ok(())
//     }

//     pub fn start() {}
//     pub fn stop() {}
// }

// #[derive()]
// pub struct AudioInputConfig
// // where
// //     T: NumCast + Zero + Send + Clone + 'static,
// {
//     /// configuration for the audio input recorder
//     recorder: RecorderConfig,
//     /// latency in milliseconds
//     // pub latency: f32,
// }

// pub monitor_input: Option<bool>,
// trait NewTrait: Recorder<T> + Clone + Send + Sync + 'static {}

#[derive()]
pub struct AudioInputNode<T>
where
    T: NumCast + Zero + Send + Clone + 'static,
    // Self: Sized + Clone,
{
    pub config: AudioInputConfig,
    // pub recorder: Box<Recorder<T> + Send + Sync + std::marker::Send: Clone + std::marker::Send: Sized + 'static>,
    // pub recorder: Arc<Box<dyn Recorder<T> + Send + Sync + 'static>>,
    pub input: Arc<RwLock<Box<dyn recorder::AudioInput<T> + Send + Sync + 'static>>>,
    // pub recorder: Box<NewTrait>,
    threads: Vec<thread::JoinHandle<()>>,
    pub is_running: bool,
}

impl<T> AudioInput<T> for AudioInputNode<T>
where
    T: NumCast + Zero + Send + Sync + Clone + 'static,
    // Self: Sized,
{
    fn new(config: AudioInputConfig) -> Result<Self> {
        let audio = CpalAudioInput::<T>::new(config.clone())?;
        cfg_if::cfg_if! {
            if #[cfg(feature = "portaudio")] {
                if config.use_portaudio{
                    let recorder = PortaudioAudioInput::<T>::new(config.clone())?;
                };
            }
        };
        Ok(Self {
            config,
            // recorder: Arc::new(Box::new(recorder)),
            input: Arc::new(RwLock::new(Box::new(audio))),
            threads: Vec::new(),
            is_running: false,
        })
    }

    fn descriptor(&self) -> Result<AudioStreamDescriptor> {
        self.input.descriptor()
    }

    // pub fn stream_from_input(&mut self, callback: AudioInputCallback<T>) -> Result<()> {
    fn stream_from_input(&mut self, callback: AudioInputCallback<T>) -> Result<()> {
        let builder = thread::Builder::new();
        let config = self.config.clone();
        let input = self.input.clone();
        let audio_stream_thread = builder
            .name("audio input stream thread".to_string())
            .spawn(move || {
                println!(
                    "streaming audio from input \"{}\"",
                    "default" // recorder.input_name().unwrap_or("unknown".to_string())
                );
                if let Err(err) = input.write().await.stream_from_input(
                    // config.monitor_input.unwrap_or(false),
                    callback,
                    // Box::new(move |samples: Result<Array2<T>>, sample_rate, nchannels| {
                    // todo: send to all subscribed analyzers
                    // if let Err(err) = rec_tx.send((samples, sample_rate, nchannels)) {
                    //     panic!("{}", err);
                    // }
                    // }),
                ) {
                    eprintln!("failed to stream input: {}", err);
                }
                println!("playback is over");
            })?;
        self.threads.push(audio_stream_thread);
        Ok(())
    }
}

#[derive()]
// pub struct AudioAnalyzer<'a>
pub struct AudioAnalyzer<T>
where
    // F: Fn(Result<Array2<T>>, u32, u16) -> () + Send + 'static,
    T: NumCast + Zero + Send + Sync + Clone + 'static,
    // T: NumCast + Zero + Send + Clone + 'static,
{
    // i really want to box the recorder or not even care about it at all
    // device: String,
    // config: StartOpts,
    // config: RecorderConfig,
    // audio_backend: Box<dyn AudioBackend>,
    // analyzer: Box<dyn Analyer>,
    analyzer: Box<dyn Analyzer<T> + Send + Sync + 'static>,
    /// the analyzer audio thread
    threads: Vec<thread::JoinHandle<()>>,
    // threads: thread::JoinHandle<()>,
    // recorder_thread: thread::JoinHandle<()>,
    is_running: bool,
}

impl<T> AudioAnalyzer<T>
where
    // F: Fn(Result<Array2<T>>, u32, u16) -> () + Send + 'static,
    // T: NumCast + Zero + Send + Clone + 'static,
    T: NumCast + Zero + Send + Sync + Clone + 'static,
{
    // pub fn new(config: StartOpts) -> Self {
    pub fn new(
        // config: RecorderConfig,
        analyzer: Box<dyn Analyzer<T> + Send + Sync + 'static>,
    ) -> Result<Self> {
        // let recorder = CpalRecorder::new(config)?;
        // cfg_if::cfg_if! {
        //     if #[cfg(feature = "portaudio")] {
        //         if config.use_portaudio{
        //             let recorder = PortaudioRecorder::new(config)?;
        //         };
        //     }
        // };
        Ok(Self {
            // config,
            analyzer,
            threads: Vec::new(),
            is_running: false,
        })
    }

    pub fn start() {}
    pub fn stop() {}
}
