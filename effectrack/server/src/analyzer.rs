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
use ringbuf::RingBuffer;
use std::sync::mpsc::channel;
// use tokio::time::{sleep, Duration as TokioDuration};

#[cfg(feature = "portaudio")]
use recorder::portaudio::PortaudioRecorder;

#[cfg(feature = "record")]
use recorder::{
    cpal::CpalAudioInput, cpal::CpalAudioOutput, AudioAnalysisError, AudioAnalysisResultReceiver,
    AudioAnalysisResultSender, AudioBackendConfig, AudioBuffer, AudioBufferReceiver,
    AudioBufferSender, AudioError, AudioInput, AudioInputCallback, AudioInputConfig,
    AudioInputNode as AudioInputNodeTrait, AudioNode, AudioOutput, AudioOutputCallback,
    AudioOutputConfig, AudioOutputNode as AudioOutputNodeTrait, AudioStreamInfo, Sample,
};

// use recorder::{cpal::CpalRecorder, AudioOutputConfig, Recorder, RecorderConfig, Sample};
// use recorder::{backend::portaudio::PortaudioAudioBackend, portaudio::PortaudioRecorder};
use async_trait::async_trait;
use std::collections::HashMap;
use std::error::Error;
use std::marker::Send;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::ops::Deref;
use std::path::PathBuf;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::{Arc, Mutex as SyncMutex};
use std::thread;
use std::time::Duration;
use tokio::signal;
use tokio::sync::{broadcast, mpsc, oneshot, watch, Mutex, RwLock};
use tokio::time;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server as TonicServer, Code, Request, Response, Status};

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
pub struct AudioAnalyzerNode<T>
where
    // F: Fn(Result<Array2<T>>, u32, u16) -> () + Send + 'static,
    // T: NumCast + Zero + Send + Sync + Clone + 'static,
    T: Sample, // T: NumCast + Zero + Send + Clone + 'static,
{
    // pub analyzer: Arc<SyncMutex<Box<dyn Analyzer<Array2<T>> + Send + Sync>>>,
    pub analyzer: Arc<SyncMutex<Box<dyn Analyzer<Array2<T>> + Sync + Send>>>,
    // pub analyzer: Arc<SyncMutex<Box<dyn Analyzer<T> + Send + Sync + 'static>>>,
    pub rx: AudioBufferReceiver<T>,
    pub tx: AudioAnalysisResultSender,
    pub is_running: bool,
    pub input_descriptor: proto::grpc::AudioInputDescriptor,
    pub analyzer_descriptor: proto::grpc::AudioAnalyzerDescriptor,
    pub input_params: AudioStreamInfo,
    pub window_size: usize,

    /// the analyzer audio thread
    threads: Vec<thread::JoinHandle<()>>,
    // threads: thread::JoinHandle<()>,
    // recorder_thread: thread::JoinHandle<()>,
}

#[async_trait]
pub trait AudioAnalyzerNodeTrait<T>
where
    T: Sample,
{
    fn new(
        input_stream: &dyn AudioInputNodeTrait<T>,
        // analyzer: &dyn Analyzer<Array2<T>>,
        analyzer: Box<dyn Analyzer<Array2<T>> + Sync + Send>,
        // analyzer: &(dyn Analyzer<Array2<T>> + Send + Sync),
        // analyzer: &(dyn Analyzer<Array2<T>> + Send + Sync + 'static),
        // analyzer: Box<dyn Analyzer<T> + Send + Sync + 'static>,
    ) -> Result<Self>
    where
        Self: Sized;
    fn descriptor(&self) -> proto::grpc::AudioAnalyzerDescriptor;
    fn connect(&self) -> AudioAnalysisResultReceiver;
    async fn start(&self) -> Result<()>;
    async fn stop(&self) -> Result<()>;
}

#[async_trait]
impl<T> AudioAnalyzerNodeTrait<T> for AudioAnalyzerNode<T>
where
    T: Sample,
{
    fn new(
        input_stream: &dyn AudioInputNodeTrait<T>,
        // analyzer: &(dyn Analyzer<Array2<T>> + Send + Sync),
        // analyzer: &dyn Analyzer<Array2<T>>,
        analyzer: Box<dyn Analyzer<Array2<T>> + Sync + Send>,
        // analyzer: &(dyn Analyzer<Array2<T>> + Send + Sync + 'static),
        // analyzer: Box<dyn Analyzer<T> + Send + Sync + 'static>,
    ) -> Result<Self> {
        let rx = input_stream.connect();
        let input_descriptor = input_stream.descriptor();
        let analyzer_descriptor = analyzer.descriptor();
        let input_params = input_stream.input_stream_params();
        let window_size = analyzer.window_size();
        // let (tx, _) = broadcast::channel::<proto::audio::analysis::AudioAnalysisResult>(
        let (tx, _) = broadcast::channel(NumCast::from(3).unwrap());
        // NumCast::from(input_stream.input_config.sample_rate().0).unwrap_or(100),

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
            // analyzer: Arc::new(SyncMutex::new(Box::new(analyzer))),
            analyzer: Arc::new(SyncMutex::new(analyzer)),
            window_size,
            input_descriptor,
            analyzer_descriptor,
            input_params,
            rx,
            tx,
            threads: Vec::new(),
            is_running: false,
        })
    }

    fn descriptor(&self) -> proto::grpc::AudioAnalyzerDescriptor {
        proto::grpc::AudioAnalyzerDescriptor {
            input: Some(self.input_descriptor.clone()),
            ..self.analyzer_descriptor.clone()
        }
    }

    fn connect(&self) -> AudioAnalysisResultReceiver {
        // Arc::new(Mutex::new(self.tx.subscribe()))
        self.tx.subscribe()
    }

    async fn start(&self) -> Result<()> {
        let rx = self.rx.clone();
        let window_size = self.window_size;
        let mut buffer = Array2::<T>::zeros((0, self.input_params.nchannels as usize));
        let (ready_tx, ready_rx) = channel();
        // let config = self.config.clone();
        // let input = self.input.clone();
        // let test = {
        //     let analyzer = analyzer.lock()?;
        //     let buffer_window_size = analyzer.window_size();
        // };

        tokio::task::spawn(async move {
            let mut rx = rx.lock().await;
            loop {
                match rx.recv().await {
                    Ok((Ok(samples), sample_rate, nchannels)) => {
                        // println!("new samples: {:?}", samples.shape());
                        // analyzer.options.nchannels = nchannels;
                        // analyzer.options.sample_rate = sample_rate;
                        if let Err(err) = buffer.append(Axis(0), samples.view()) {
                            eprintln!("failed to extend buffer: {}", err);
                        }
                        // println!("size of buffer: {:?}", buffer.shape());
                        // todo: maybe measure the processing time here and try to keep the real time
                        let buffer_size = buffer.len_of(Axis(0));
                        if buffer_size > NumCast::from(sample_rate * 1).unwrap() {
                            panic!("more than one second in the buffer");
                        }

                        let ready_buffers = buffer_size / window_size;
                        let mut processed = 0;

                        // process the chunks
                        for i in (0..ready_buffers) {
                            let start = i * window_size;
                            let end = (i + 1) * window_size;
                            // println!("analyzing from {} to {}", start, end);
                            let chunk = buffer
                                .slice_axis(Axis(0), Slice::from(start..end))
                                .to_owned();
                            if let Err(err) = ready_tx.send(chunk) {
                                eprintln!("failed to send ready buffer for analysis: {}", err);
                            }
                            processed += 1;
                        }
                        buffer
                            .slice_axis_inplace(Axis(0), Slice::from((processed * window_size)..));
                    }
                    Ok((Err(err), _, _)) => {
                        println!("output receive error: {:?}", err);
                    }
                    Err(err) => {
                        println!("output receive error: {:?}", err);
                    }
                }
            }
        });
        let analyzer = self.analyzer.clone();
        let tx = self.tx.clone();
        let builder = thread::Builder::new();
        let audio_stream_thread =
            builder
                .name("audio input stream thread".to_string())
                .spawn(move || {
                    let mut analyzer = analyzer.lock().unwrap();
                    loop {
                        match ready_rx.recv() {
                            Ok(buffer) => {
                                let analyzed = analyzer
                                    .analyze_samples(buffer)
                                    .map_err(|err| AudioAnalysisError::Unknown(err.to_string()));
                                tx.send(analyzed);
                                // if let Err(err) = result_tx.send() {
                                //     println
                                // }
                            }
                            Err(err) => {}
                        }
                    }
                });

        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        Ok(())
    }
}

// #[derive()]
// pub trait AudioNode<T>
// where
//     T: NumCast + Zero + Send + Clone + 'static,
//     // Self: Sized,
// {
//     fn connect_input(&mut self,

#[derive()]
pub struct AudioOutputNode<T>
where
    // T: NumCast + Zero + Send + Clone + 'static,
    T: Sample,
    // Self: Sized,
{
    pub config: AudioOutputConfig,
    // pub recorder: Box<Recorder<T> + Send + Sync + std::marker::Send: Clone + std::marker::Send: Sized + 'static>,
    // pub recorder: Arc<Box<dyn Recorder<T> + Send + Sync + 'static>>,
    // pub output: Arc<Box<dyn recorder::AudioOutput<T> + Send + Sync + 'static>>,
    pub output: Arc<SyncMutex<Box<dyn recorder::AudioOutput<T> + Send + Sync + 'static>>>,
    pub input_descriptor: proto::grpc::AudioInputDescriptor,
    pub input_params: AudioStreamInfo,
    pub output_descriptor: proto::grpc::AudioOutputDescriptor,
    pub output_params: AudioStreamInfo,
    // pub recorder: Box<NewTrait>,
    threads: Vec<thread::JoinHandle<()>>,
    // input and output
    // pub rx: RwLock<AudioBufferReceiver<T>>,
    pub rx: AudioBufferReceiver<T>,
    pub is_running: bool,
    //
    // pub stream_tx: AudioBufferSender<T>>,
    // pub stream_rx: Option<AudioBufferReceiver<T>>,
    // pub stream_tx: Option<AudioBufferSender<T>>,
}

impl<T> AudioOutputNodeTrait<T> for AudioOutputNode<T>
where
    // T: NumCast + Zero + Send + Sync + Clone + 'static,
    T: Sample,
    // Self: Sized,
{
    // fn new(input_stream: AudioStreamDescriptor, config: AudioOutputConfig) -> Result<Self> {
    // fn new(input_stream: AudioBufferReceiver<T>, config: AudioOutputConfig) -> Result<Self> {
    fn new(
        // input_stream: &AudioInputNodeTrait<T>,
        input_stream: &dyn AudioInputNodeTrait<T>,
        // input_stream: &dyn Deref<Target=&dyn AudioInputNodeTrait<T>>,
        // input_stream: &dyn std::ops::Deref<Target = AudioInputNodeTrait<T>>,
        config: AudioOutputConfig,
    ) -> Result<Self> {
        let rx = input_stream.connect();
        // rx.recv();
        // let audio = CpalAudioOutput::<T>::new(input_stream, config.clone())?;
        let output_stream = CpalAudioOutput::<T>::new(config.clone())?;
        let input_descriptor = input_stream.descriptor();
        let input_params = input_stream.input_stream_params();
        let output_descriptor = output_stream.descriptor()?;
        let output_params = output_stream.output_stream_params();
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
            // output: Arc::new(Box::new(audio)),
            input_descriptor,
            input_params,
            output_descriptor,
            output_params,
            // input: Box::new(input_stream),
            output: Arc::new(SyncMutex::new(Box::new(output_stream))),
            threads: Vec::new(),
            // rx: RwLock::new(rx),
            // rx: Arc::new(rx),
            rx,
            // stream_rx: None,
            // stream_tx: None,
            is_running: false,
        })
    }

    fn descriptor(&self) -> proto::grpc::AudioOutputDescriptor {
        proto::grpc::AudioOutputDescriptor {
            input: Some(self.input_descriptor.clone()),
            ..self.output_descriptor.clone()
        }
    }

    fn input_stream_params(&self) -> AudioStreamInfo {
        self.input_params
    }

    fn output_stream_params(&self) -> AudioStreamInfo {
        self.output_params
    }

    // fn stream_to_output(
    //     &mut self,
    //     // input_config: Option<StreamConfig>,
    //     callback: AudioOutputCallback<T>,
    // ) -> Result<()> {
    //     // let output = self.output.clone();
    //     Ok(())
    //     // output.stream_to_output(
    //     // input_config: Option<StreamConfig>,
    //     // input_config: AudioInputConfig,
    //     // playback: bool,
    //     // mut callback: AudioOutputCallback<T>,
    //     // callback: &'static (dyn Fn(Result<Array2<T>>, u32, u16) -> () + Send + Sync),
    //     // callback: Callback<T>,
    //     // )
    //     // let builder = thread::Builder::new();
    //     // let config = self.config.clone();
    //     // let input = self.input.clone();
    //     // let audio_stream_thread = builder
    //     //     .name("audio input stream thread".to_string())
    //     //     .spawn(move || {
    //     //         println!(
    //     //             "streaming audio from input \"{}\"",
    //     //             "default" // recorder.input_name().unwrap_or("unknown".to_string())
    //     //         );
    //     //         if let Err(err) = input.stream_from_input(
    //     //             // config.monitor_input.unwrap_or(false),
    //     //             callback,
    //     //             // Box::new(move |samples: Result<Array2<T>>, sample_rate, nchannels| {
    //     //             // todo: send to all subscribed analyzers
    //     //             // if let Err(err) = rec_tx.send((samples, sample_rate, nchannels)) {
    //     //             //     panic!("{}", err);
    //     //             // }
    //     //             // }),
    //     //         ) {
    //     //             eprintln!("failed to stream input: {}", err);
    //     //         }
    //     //         println!("playback is over");
    //     //     })?;
    //     // self.threads.push(audio_stream_thread);
    //     // Ok(())
    // }

    // todo: move the callback in here
    // pub fn stream_from_input(&mut self, callback: AudioInputCallback<T>) -> Result<()> {
}

// impl<T> AudioOutputNode<T>
#[async_trait]
impl<T> AudioNode<T> for AudioOutputNode<T>
where
    T: Sample,
{
    async fn start(&mut self) -> Result<()> {
        // if self.rx.try_lock().is_err() {
        //     // return Err()
        // }
        // let mut s = self.rx.read().await;
        // let mut s = self.rx.write().await;
        // delay in case the input and output devices aren't synced
        let latency_frames = (self.config.latency / 1_000.0) * self.input_params.sample_rate as f32;
        let latency_samples = latency_frames as usize * self.input_params.nchannels as usize;

        // ring buffer to keep samples
        println!("the ring buffer size is: {}", latency_samples * 2);
        let ring = RingBuffer::<T>::new(latency_samples * 2);
        let (mut producer, mut consumer) = ring.split();

        for _ in 0..latency_samples {
            let _ = producer.push(T::zero());
        }

        let rx = self.rx.clone();
        tokio::task::spawn(async move {
            // let mut rx = rx.lock().unwrap();
            let mut rx = rx.lock().await;
            let mut output_fell_behind = false;
            loop {
                // let test = rx.lock().unwrap().recv();
                // let rx = rx.lock().unwrap();
                match rx.recv().await {
                    Ok((Ok(samples), sample_rate, nchannels)) => {
                        // println!("got {:?} samples", samples.shape());
                        samples.for_each(|sample| {
                            // println!("producing {:?}", sample);
                            match producer.push(sample.clone()) {
                                Ok(()) => {
                                    // println!("success!: {:?}", sample.clone());
                                }
                                Err(err) => {
                                    // println!("failed to produce: {:?}", err);
                                    output_fell_behind = true;
                                }
                            }
                        });
                        // for sample in &samples {
                        //                             }
                        // println!("output received: {:?}", samples);
                    }
                    Ok((Err(err), _, _)) => {
                        println!("output receive error: {:?}", err);
                    }
                    Err(err) => {
                        println!("output receive error: {:?}", err);
                    }
                }
            }
        });
        let output_callback: AudioOutputCallback<T> = Box::new(move || {
            // Some(T::zero())
            let test = consumer.pop();
            // println!("popped {:?}", test);
            test
            // Some(match consumer.pop() {
            //     Some(s) => s,
            //     None => 0.0,
            // })
        });
        let builder = thread::Builder::new();
        let output = self.output.clone();
        let audio_receiver_thread = builder
            .name("audio output stream thread".to_string())
            .spawn(move || {
                let mut output = output.lock().unwrap();
                output.stream_to_output(output_callback);
            });
        //
        // let output_callback = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        //     let mut input_fell_behind = false;
        //     for sample in data {
        //         *sample = match consumer.pop() {
        //             Some(s) => NumCast::from(s).unwrap(),
        //             None => {
        //                 input_fell_behind = true;
        //                 0.0
        //             }
        //         };
        //     }
        //     if input_fell_behind {
        //         eprintln!("input stream fell behind: try increasing latency");
        //     }
        // };
        // let output_stream = self.output_device.build_output_stream(
        //     &self.input_config.clone().into(),
        //     output_callback,
        //     |err| eprintln!("an error occurred on output stream: {}", err),
        // )?;
        // output_stream

        Ok(())
    }
}

// impl<T> AudioOutputNodeTrait<T> for AudioOutputNode<T> where T: Sample {}

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
    // pub input: Arc<RwLock<Box<dyn recorder::AudioInput<T> + Send + Sync + 'static>>>,
    pub input: Arc<Box<dyn recorder::AudioInput<T> + Send + Sync + 'static>>,
    pub input_descriptor: proto::grpc::AudioInputDescriptor,
    pub input_params: AudioStreamInfo,
    // the stream sender is private and only used internally
    // stream_rx: AudioBufferReceiver<T>,

    // the stream_tx is public and can be used to subscribe to the feed
    // pub stream_tx: AudioBufferSender<T>,
    pub tx: AudioBufferSender<T>,

    // pub recorder: Box<NewTrait>,
    threads: Vec<thread::JoinHandle<()>>,
    pub is_running: bool,
}

// impl<T> AudioInputNodeTrait<T> for AudioInputNode<T>
// impl<T> AudioNode<T> for AudioInputNode<T> where T: Sample {}

impl<T> AudioInputNodeTrait<T> for AudioInputNode<T>
where
    // T: NumCast + Zero + Send + Sync + Clone + 'static,
    T: Sample,
    // Self: Sized,
{
    fn new(config: AudioInputConfig) -> Result<Self> {
        let input_stream = CpalAudioInput::<T>::new(config.clone())?;
        cfg_if::cfg_if! {
            if #[cfg(feature = "portaudio")] {
                if config.use_portaudio{
                    let input_stream = PortaudioAudioInput::<T>::new(config.clone())?;
                };
            }
        };
        let input_descriptor = input_stream.descriptor()?;
        let input_params = input_stream.input_stream_params();
        let (tx, _) = broadcast::channel(
            NumCast::from(input_stream.input_config.sample_rate().0).unwrap_or(100),
        );
        Ok(Self {
            config,
            // recorder: Arc::new(Box::new(recorder)),
            // input: Arc::new(RwLock::new(Box::new(audio))),
            // input: Arc::new(Box::new(audio)),
            input: Arc::new(Box::new(input_stream)),
            input_descriptor,
            input_params,
            threads: Vec::new(),
            // stream_rx: tx,
            // tx: Arc::new(tx),
            tx,
            is_running: false,
        })
    }

    fn connect(&self) -> AudioBufferReceiver<T> {
        Arc::new(Mutex::new(self.tx.subscribe()))
    }

    fn descriptor(&self) -> proto::grpc::AudioInputDescriptor {
        self.input_descriptor.clone()
    }

    fn input_stream_params(&self) -> AudioStreamInfo {
        self.input_params
    }

    // fn descriptor(&self) -> Result<proto::grpc::AudioInputDescriptor> {
    //     Ok(proto::grpc::AudioInputDescriptor {
    //         // kind: proto::grpc::audio_stream_descriptor::AudioStreamKind::Input.into(),
    //         backend: self.input.backend(),
    //         device: self.input.input_device.name()?,
    //         host: self.input.host.id().name().to_string(),
    //     })
    // }

    // fn descriptor(&self) -> Result<proto::grpc::AudioInputDescriptor> {
    //     self.input.descriptor()
    // }

    //     recorder::AudioInputNode<_>
    //
    // pub fn stream_from_input(&mut self, callback: AudioInputCallback<T>) -> Result<()> {

    // fn stream_from_input(&mut self, callback: AudioInputCallback<T>) -> Result<()> {
    //     let builder = thread::Builder::new();
    //     let config = self.config.clone();
    //     // let input = self.input.clone();
    //     // let audio_stream_thread = builder
    //     //     .name("audio input stream thread".to_string())
    //     //     .spawn(move || {
    //     //         println!(
    //     //             "streaming audio from input \"{}\"",
    //     //             "default" // recorder.input_name().unwrap_or("unknown".to_string())
    //     //         );
    //     //         if let Err(err) = input.write().await.stream_from_input(
    //     //             // config.monitor_input.unwrap_or(false),
    //     //             callback,
    //     //             // Box::new(move |samples: Result<Array2<T>>, sample_rate, nchannels| {
    //     //             // todo: send to all subscribed analyzers
    //     //             // if let Err(err) = rec_tx.send((samples, sample_rate, nchannels)) {
    //     //             //     panic!("{}", err);
    //     //             // }
    //     //             // }),
    //     //         ) {
    //     //             eprintln!("failed to stream input: {}", err);
    //     //         }
    //     //         println!("playback is over");
    //     //     })?;
    //     // self.threads.push(audio_stream_thread);
    //     Ok(())
    // }
}

#[async_trait]
impl<T> AudioNode<T> for AudioInputNode<T>
where
    T: Sample,
{
    async fn start(&mut self) -> Result<()> {
        let config = self.config.clone();
        // self.input.write().await.stream_from_input(
        println!("starting");
        let builder = thread::Builder::new();
        let tx = self.tx.clone();
        // let config = self.config.clone();
        let input = self.input.clone();
        // tokio::spawn(async {
        let audio_stream_thread =
            builder
                .name("audio input stream thread".to_string())
                .spawn(move || {
                    input.stream_from_input(
                        // config.monitor_input.unwrap_or(false),
                        // callback,
                        Box::new(move |samples: Result<Array2<T>>, sample_rate, nchannels| {
                            tx.send((
                                samples.map_err(|err| AudioError::Unknown(err.to_string())),
                                sample_rate,
                                nchannels,
                            ));
                            // todo: send to all subscribed analyzers
                            // println!("hot samples: {:?}", samples);
                            // if let Err(err) = rec_tx.send((samples, sample_rate, nchannels)) {
                            //     panic!("{}", err);
                            // }
                        }),
                    );
                    // sleep(Duration::from_secs(10)).await;
                    // std::thread::sleep(std::time::Duration::from_secs(60 * 60));
                    // drop(handle);
                });
        println!("started");
        // {
        //             eprintln!("failed to stream input: {}", err);
        //         };
        Ok(())
    }
}
