// use crate::analyzer::{AudioAnalyzer, AudioInput};
use crate::analyzer::{AudioAnalyzerNode, AudioInputNode, AudioOutputNode};
use crate::cli::Config;
use crate::controller::Controller;
use crate::recorder::{AudioInput, AudioInputConfig, AudioOutput, AudioOutputConfig};
use crate::viewer::Viewer;
use analysis::{mel::Hz, mel::Mel, Analyzer};
use anyhow::Result;
use ndarray::{
    concatenate, indices, Array, IntoDimension, Ix, NdIndex, RemoveAxis, ScalarOperand, Slice, Zip,
};
use num::traits::{Float, FloatConst, NumCast, Zero};
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;
use tokio::sync::{mpsc, oneshot, watch, RwLock};

// pub type Sample = f32;
// pub type MyStatus = String;

#[derive(Clone)]
// pub struct Session<VU, CU, S> {
pub struct Session<VU, CU> {
    // viewers: HashMap<String, RwLock<Viewer<U, B::Rec>>>,
    // recorder: Option<Arc<R>>,
    // running analyzers
    // analyzers: RwLock<Vec<RwLock<AudioAnalyzer>>>,
    // is_analyzing: bool,
    /// session configuration
    pub config: Config,

    /// connected controllers
    pub controllers: Arc<RwLock<HashMap<String, RwLock<Controller<CU>>>>>,

    /// connected viewers
    // pub viewers: RwLock<HashMap<proto::grpc::InstanceId, RwLock<Viewer<VU, S>>>>,
    pub viewers: Arc<RwLock<HashMap<proto::grpc::InstanceId, RwLock<Viewer<VU>>>>>,

    /// all running analyzers of this session
    pub analyzers: Arc<
        RwLock<
            HashMap<proto::grpc::AudioAnalyzerDescriptor, RwLock<AudioAnalyzerNode<crate::Sample>>>,
        >,
    >,

    /// all running input streams of this session
    pub input_streams: Arc<
        RwLock<HashMap<proto::grpc::AudioInputDescriptor, RwLock<AudioInputNode<crate::Sample>>>>,
    >,

    /// all running output streams of this session
    pub output_streams: Arc<
        RwLock<HashMap<proto::grpc::AudioOutputDescriptor, RwLock<AudioOutputNode<crate::Sample>>>>,
    >,
}

// impl<VU, CU, S> Session<VU, CU, S> {
impl<VU, CU> Session<VU, CU> {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            viewers: Arc::new(RwLock::new(HashMap::new())),
            controllers: Arc::new(RwLock::new(HashMap::new())),
            analyzers: Arc::new(RwLock::new(HashMap::new())),
            input_streams: Arc::new(RwLock::new(HashMap::new())),
            output_streams: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    // async fn start_analysis<T, B>(&mut self, audio_backend: &Arc<B>) -> Result<()>
    pub fn remove_analyzer() {}
    pub fn add_analyzer() {}

    async fn start_analysis<T>(&mut self) -> Result<()>
    where
        // B: AudioBackend + Sync + Send + 'static,
        T: Float
            + FloatConst
            + Mel
            + Hz
            + Sync
            + Send
            + std::fmt::Debug
            + Default
            + ScalarOperand
            + 'static,
    {
        // self.is_analyzing = true;
        // let audio_backend = audio_backend.clone();

        // // create a tokio runtime to perform async operations in threads
        // let rt = tokio::runtime::Builder::new_current_thread()
        //     .enable_all()
        //     .build()?;

        // let (result_tx, result_rx) = std::sync::mpsc::channel();
        // let (rec_tx, rec_rx) = std::sync::mpsc::channel();
        // // let rec_file_name = file_name.clone();
        // let play_file = self.config.play_file.clone();
        // let builder = thread::Builder::new();
        // let recorder_thread = builder.name("recorder thread".to_string()).spawn(move || {
        //     println!("starting the recording...");
        //     let rec = audio_backend.new_recorder().expect("create recorder");

        //     // if let Some(file) = self.state.read().await.config.play_file {
        //     if let Some(file) = play_file {
        //         if let Err(err) = rec.stream_file(
        //             PathBuf::from(file),
        //             move |samples: Result<Array2<T>>, sample_rate, nchannels| {
        //                 if let Err(err) = rec_tx.send((samples, sample_rate, nchannels)) {
        //                     panic!("{}", err);
        //                 }
        //             },
        //         ) {
        //             eprintln!("failed to stream input: {}", err);
        //         };
        //     } else {
        //         if let Err(err) = rec.stream_input(
        //             true,
        //             move |samples: Result<Array2<T>>, sample_rate, nchannels| {
        //                 if let Err(err) = rec_tx.send((samples, sample_rate, nchannels)) {
        //                     panic!("{}", err);
        //                 }
        //             },
        //         ) {
        //             eprintln!("failed to stream input: {}", err);
        //         }
        //     }
        //     println!("playback is over");
        // })?;

        // // thread that collects and analyzes samples
        // let builder = thread::Builder::new();
        // let analysis_thread = builder.name("analysis thread".to_string()).spawn(move || {
        //     // let (sample_rate, nchannels) = match self.config.play_file {
        //     //      Some(file) =>
        //     //         <B as AudioBackend>::Rec::get_file_info(PathBuf::from(file).clone()).unwrap();
        //     //      None => {
        //     //         let input_config = self.input_device.default_input_config()?;
        //     //      }
        //     // };
        //     let buffer_window_size = 2048;
        //     let analyzer_opts = SpectralAnalyzerOptions {
        //         window_size: buffer_window_size,
        //         mel_bands: 24,
        //         // nchannels: nchannels,
        //         // sample_rate: sample_rate,
        //         fps: 60,
        //         ..SpectralAnalyzerOptions::default()
        //     };
        //     let mut analyzer = SpectralAnalyzer::<T>::new(analyzer_opts).unwrap();
        //     let mut buffer = Array2::<T>::zeros((0, 2));
        //     loop {
        //         match rec_rx.recv() {
        //             Ok((Ok(samples), sample_rate, nchannels)) => {
        //                 // println!("new samples: {:?}", samples.shape());
        //                 analyzer.options.nchannels = nchannels;
        //                 analyzer.options.sample_rate = sample_rate;
        //                 if let Err(err) = buffer.append(Axis(0), samples.view()) {
        //                     eprintln!("failed to extend buffer: {}", err);
        //                 }
        //                 // println!("size of buffer: {:?}", buffer.shape());
        //                 // todo: maybe measure the processing time here and try to keep the real time
        //                 let buffer_size = buffer.len_of(Axis(0));
        //                 if buffer_size > NumCast::from(sample_rate * 1).unwrap() {
        //                     panic!("more than one second in the buffer");
        //                 }

        //                 let ready_buffers = buffer_size / buffer_window_size;
        //                 let mut processed = 0;

        //                 // process the chunks
        //                 for i in (0..ready_buffers) {
        //                     let start = i * buffer_window_size;
        //                     let end = (i + 1) * buffer_window_size;
        //                     // println!("analyzing from {} to {}", start, end);
        //                     let chunk = buffer
        //                         .slice_axis(Axis(0), Slice::from(start..end))
        //                         .to_owned();
        //                     if let Err(err) = result_tx.send(analyzer.analyze_samples(chunk)) {
        //                         eprintln!("failed to send result: {}", err);
        //                     }
        //                     processed += 1;
        //                 }
        //                 buffer.slice_axis_inplace(
        //                     Axis(0),
        //                     Slice::from((processed * buffer_window_size)..),
        //                 );
        //             }
        //             Ok((Err(err), _, _)) => {
        //                 println!("error while recording samples: {}", err);
        //             }
        //             Err(err) => {
        //                 // println!("failed to receive new samples: {}", err);
        //             }
        //         }
        //     }
        // })?;

        // // wait for analysis results and send them to the user
        // let stream_tx = self.connection.clone();
        // let builder = thread::Builder::new();
        // let update_thread = builder.name("upate thread".to_string()).spawn(move || {
        //     let mut seq_num = 0;
        //     loop {
        //         match result_rx.recv() {
        //             Ok(Err(analysis_err)) => {
        //                 println!("{}", analysis_err);
        //             }
        //             Ok(Ok(mut analysis_result)) => {
        //                 analysis_result.seq_num = seq_num;
        //                 let analysis_result_update = Update {
        //                     update: Some(update::Update::AudioAnalysisResult(analysis_result)),
        //                 };
        //                 // let stream_tx = self.connection;
        //                 match rt.block_on(async {
        //                     let rx = stream_tx.read().await;
        //                     // rx.send(Ok(analysis_result_update)).await
        //                     rx.send(analysis_result_update).await
        //                 }) {
        //                     Err(err) => println!("{}", err),
        //                     Ok(_) => seq_num = seq_num + 1,
        //                 };
        //             }
        //             Err(recv_err) => {
        //                 println!("{}", recv_err);
        //                 // break;
        //             }
        //         }
        //     }
        // });

        // let analysis = AudioAnalysis {
        //     recorder_thread,
        //     is_running: true,
        // };
        Ok(())
    }
}
