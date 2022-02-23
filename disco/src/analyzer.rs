use crate::cli::Opts;
#[cfg(feature = "analyze")]
use analysis::Analyzer;
use anyhow::Result;
use ndarray::prelude::*;
use ndarray::Slice;
use num::traits::{NumCast, Zero};
use ringbuf::RingBuffer;
use std::path::PathBuf;
use std::sync::RwLock as SyncRwLock;

#[cfg(feature = "portaudio")]
use recorder::portaudio::PortaudioRecorder;

#[cfg(feature = "record")]
use recorder::{
    cpal::CpalAudioFile, cpal::CpalAudioInput, cpal::CpalAudioOutput, AudioAnalysisError,
    AudioAnalysisResultReceiver, AudioAnalysisResultSender, AudioBackendConfig,
    AudioBufferReceiver, AudioBufferSender, AudioError, AudioInput, AudioInputConfig,
    AudioInputNode as AudioInputNodeTrait, AudioNode, AudioOutput, AudioOutputCallback,
    AudioOutputConfig, AudioOutputNode as AudioOutputNodeTrait, AudioStreamInfo, Sample,
};

use async_trait::async_trait;
use std::marker::Send;
use std::sync::{Arc, Mutex as SyncMutex};
use std::{thread, time};
use tokio::sync::{broadcast, Mutex};

impl From<Opts> for AudioBackendConfig {
    fn from(_options: Opts) -> AudioBackendConfig {
        AudioBackendConfig {
            #[cfg(use_jack)]
            use_jack: _options.use_jack,
            #[cfg(use_portaudio)]
            use_portaudio: _options.use_portaudio,
        }
    }
}

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
    T: Sample,
{
    pub analyzer: Arc<SyncMutex<Box<dyn Analyzer<Array2<T>> + Sync + Send>>>,
    pub rx: AudioBufferReceiver<T>,
    pub tx: AudioAnalysisResultSender,
    pub is_running: bool,
    pub input_descriptor: proto::grpc::AudioInputDescriptor,
    pub analyzer_descriptor: proto::grpc::AudioAnalyzerDescriptor,
    pub input_params: AudioStreamInfo,
    pub window_size: usize,
}

#[async_trait]
pub trait AudioAnalyzerNodeTrait<T>
where
    T: Sample,
{
    fn new(
        input_stream: &dyn AudioInputNodeTrait<T>,
        analyzer: Box<dyn Analyzer<Array2<T>> + Sync + Send>,
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
        analyzer: Box<dyn Analyzer<Array2<T>> + Sync + Send>,
    ) -> Result<Self> {
        let rx = input_stream.connect();
        let input_descriptor = input_stream.descriptor();
        let analyzer_descriptor = analyzer.descriptor();
        let input_params = input_stream.input_stream_params();
        let window_size = analyzer.window_size();
        let (tx, _) = broadcast::channel(NumCast::from(3).unwrap());
        Ok(Self {
            analyzer: Arc::new(SyncMutex::new(analyzer)),
            window_size,
            input_descriptor,
            analyzer_descriptor,
            input_params,
            rx,
            tx,
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
        self.tx.subscribe()
    }

    async fn start(&self) -> Result<()> {
        let rx = self.rx.clone();
        let window_size = self.window_size;
        let buffer = Arc::new(SyncRwLock::new(Array2::<T>::zeros((
            window_size,
            self.input_params.nchannels as usize,
        ))));

        let buffer_clone = buffer.clone();
        tokio::task::spawn(async move {
            let mut rx = rx.lock().await;
            loop {
                tokio::select! {
                                    // _ = &mut self.shutdown_signal => {
                                    //     println!("shutdown from open connection");
                                    //     return;
                                    // }
                                    received = rx.recv() => {
                                        match received {
                Ok((Ok(samples), _sample_rate, _nchannels)) => {
                                        if let Ok(mut wbuffer) = buffer_clone.write() {
                                            if let Err(err) = wbuffer.append(Axis(0), samples.view()) {
                                                eprintln!("failed to extend buffer: {}", err);
                                            }
                                                                                        wbuffer
                                                .slice_axis_inplace(
                                                    Axis(0),
                                                    Slice::from(-(window_size as isize)..),
                                                );
                                            drop(wbuffer);
                                        }
                                    }
                                    Ok((Err(err), _, _)) => {
                                        println!("output receive error: {:?}", err);
                                    }
                                    Err(err) => {
                                        println!("output receive error: {:?}", err);
                                    }
                                        }
                                    }
                                }
            }
        });
        let analyzer = self.analyzer.clone();
        let tx = self.tx.clone();
        let builder = thread::Builder::new();
        let analyzer_descriptor = self.analyzer_descriptor.clone();
        if let Err(err) = builder
            .name(format!("disco audio analyzer {}", analyzer_descriptor).to_string())
            .spawn(move || {
                let mut analyzer = analyzer.lock().unwrap();
                loop {
                    // wait 1/60
                    thread::sleep(time::Duration::from_millis(1000 / 60));
                    if let Ok(rbuffer) = buffer.read() {
                        let rbuffer_copy = rbuffer.to_owned();
                        // drop(rbuffer);
                        // println!("analyzer got samples");
                        let analyzed = analyzer
                            .analyze_samples(rbuffer_copy)
                            .map_err(|err| AudioAnalysisError::Unknown(err.to_string()));
                        // println!("{:?}", analyzed);
                        if let Err(err) = tx.send(analyzed) {
                            eprintln!(
                                "failed to send analysis result of {}: {}",
                                analyzer_descriptor, err
                            );
                        }
                    }
                }
            })
        {
            eprintln!(
                "failed to launch audio analyzer thread for {}: {}",
                self.analyzer_descriptor, err
            );
        }

        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        Ok(())
    }
}

#[derive()]
pub struct AudioOutputNode<T>
where
    T: Sample,
{
    pub config: AudioOutputConfig,
    pub output: Arc<SyncMutex<Box<dyn recorder::AudioOutput<T> + Send + Sync + 'static>>>,
    pub input_descriptor: proto::grpc::AudioInputDescriptor,
    pub input_params: AudioStreamInfo,
    pub output_descriptor: proto::grpc::AudioOutputDescriptor,
    pub output_params: AudioStreamInfo,
    pub rx: AudioBufferReceiver<T>,
    pub is_running: bool,
}

impl<T> AudioOutputNodeTrait<T> for AudioOutputNode<T>
where
    T: Sample,
{
    fn new(input_stream: &dyn AudioInputNodeTrait<T>, config: AudioOutputConfig) -> Result<Self> {
        let rx = input_stream.connect();
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
            input_descriptor,
            input_params,
            output_descriptor,
            output_params,
            output: Arc::new(SyncMutex::new(Box::new(output_stream))),
            rx,
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
}

#[async_trait]
impl<T> AudioNode<T> for AudioOutputNode<T>
where
    T: Sample,
{
    async fn start(&mut self) -> Result<()> {
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
            let mut rx = rx.lock().await;
            let mut output_fell_behind = false;
            loop {
                match rx.recv().await {
                    Ok((Ok(samples), _sample_rate, _nchannels)) => {
                        samples.for_each(|sample| {
                            if let Err(err) = producer.push(sample.clone()) {
                                eprintln!("failed to produce: {:?}", err);
                                output_fell_behind = true;
                            }
                        });
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
            let sample = consumer.pop();
            sample
        });
        let builder = thread::Builder::new();
        let output = self.output.clone();
        let output_descriptor = self.output_descriptor.clone();
        if let Err(err) = builder
            .name(format!("disco audio output {}", output_descriptor).to_string())
            .spawn(move || {
                let mut output = output.lock().unwrap();
                if let Err(err) = output.stream_to_output(output_callback) {
                    eprintln!("failed to stream to output {}: {}", output_descriptor, err);
                }
            })
        {
            eprintln!(
                "failed to launch audio output stream for {}: {}",
                self.output_descriptor, err
            );
        }
        Ok(())
    }
}

#[derive()]
pub struct AudioInputNode<T>
where
    T: NumCast + Zero + Send + Clone + 'static,
{
    // this should move to the input?
    // pub config: AudioInputConfig,
    // this should work for file and for inputs
    pub input: Arc<Box<dyn recorder::AudioInput<T> + Send + Sync + 'static>>,
    pub input_descriptor: proto::grpc::AudioInputDescriptor,
    pub input_params: AudioStreamInfo,
    pub tx: AudioBufferSender<T>,
    pub is_running: bool,
}

impl<T> AudioInputNodeTrait<T> for AudioInputNode<T>
where
    T: Sample,
{
    // fn from_input(config: AudioInputConfig) -> Result<Box<dyn AudioInput<T>>> { // Self> {
    fn from_input(config: AudioInputConfig) -> Result<Self> {
        // Self> {
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
            NumCast::from(input_params.sample_rate).unwrap(), // NumCast::from(input_stream.input_config.sample_rate().0).unwrap_or(100),
        );
        Ok(Self {
            // config,
            input: Arc::new(Box::new(input_stream)),
            input_descriptor,
            input_params,
            tx,
            is_running: false,
        })
    }

    fn from_file(path: PathBuf, looped: bool) -> Result<Self> {
        let input_stream = CpalAudioFile::<T>::new(path, looped)?;
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
            NumCast::from(input_params.sample_rate).unwrap(), // NumCast::from(input_stream.input_config.sample_rate().0).unwrap_or(100),
        );
        Ok(Self {
            // config,
            input: Arc::new(Box::new(input_stream)),
            input_descriptor,
            input_params,
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
}

// impl<T> AudioInputNode<T>
// where
//     T: Sample,
// {
//     fn process(&mut self, samples: Result<Array2<T>>, sample_rate: u32, nchannels: u16) -> () {
//         // let samples_clone = samples.clone().map(|s| s.to_owned());
//         // let sample_size = samples.as_ref().map(|s| s.len());
//         if let Err(err) = tx.send((
//             samples.map_err(|err| AudioError::Unknown(err.to_string())),
//             sample_rate,
//             nchannels,
//         )) {
//             eprintln!(
//                 "failed to send input samples of {}: {}",
//                 input_descriptor_clone, err
//             );
//         } // else {
//           // println!("sent samples");
//           // println!("sent {:?} samples", sample_size);
//           // println!("sent {:?} samples", sample_size);
//           // println!("sent {:?} samples", samples);
//           // }
//         ()
//     }
// }

#[async_trait]
impl<T> AudioNode<T> for AudioInputNode<T>
where
    T: Sample,
{
    async fn start(&mut self) -> Result<()> {
        // let config = self.config.clone();
        let builder = thread::Builder::new();
        let tx = self.tx.clone();
        let input = self.input.clone();
        // todo: check the input descriptor
        let input_descriptor = self.input_descriptor.clone();
        if let Err(err) = builder
            .name(format!("disco input {} recorder", self.input_descriptor).to_string())
            .spawn(move || {
                let input_descriptor_clone = input_descriptor.clone();
                // if let Err(err) = input.stream_input(Box::new(Self::process)) {
                let process = move |samples: Result<Array2<T>>, sample_rate, nchannels| -> () {
                    // let samples_clone = samples.clone().map(|s| s.to_owned());
                    // let sample_size = samples.as_ref().map(|s| s.len());
                    if let Err(err) = tx.send((
                        samples.map_err(|err| AudioError::Unknown(err.to_string())),
                        sample_rate,
                        nchannels,
                    )) {
                        eprintln!(
                            "failed to send input samples of {}: {}",
                            input_descriptor_clone, err
                        );
                    } // else {
                      // println!("sent samples");
                      // println!("sent {:?} samples", sample_size);
                      // println!("sent {:?} samples", sample_size);
                      // println!("sent {:?} samples", samples);
                      // }
                    ()
                };

                if let Err(err) = input.stream(Box::new(process)) {
                    // move |samples: Result<Array2<T>>, sample_rate, nchannels| {
                    //     self.process(samples, sample_rate, nchannels);
                    // },
                    // )) {
                    //         // let samples_clone = samples.clone().map(|s| s.to_owned());
                    //         // let sample_size = samples.as_ref().map(|s| s.len());
                    //         if let Err(err) = tx.send((
                    //             samples.map_err(|err| AudioError::Unknown(err.to_string())),
                    //             sample_rate,
                    //             nchannels,
                    //         )) {
                    //             eprintln!(
                    //                 "failed to send input samples of {}: {}",
                    //                 input_descriptor_clone, err
                    //             );
                    //         } // else {
                    //           // println!("sent samples");
                    //           // println!("sent {:?} samples", sample_size);
                    //           // println!("sent {:?} samples", sample_size);
                    //           // println!("sent {:?} samples", samples);
                    //           // }
                    //     },
                    // )) {
                    eprintln!("failed to stream input {}: {}", input_descriptor, err);
                }
            })
        {
            eprintln!(
                "failed to launch audio input recorder thread for {}: {}",
                self.input_descriptor, err
            );
        }
        println!("started");
        Ok(())
    }
}
