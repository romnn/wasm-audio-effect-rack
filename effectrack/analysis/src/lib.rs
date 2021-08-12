use anyhow::Result;
use proto::audio::analysis::{AudioAnalysisResult, SpectralAudioAnalysisResult};
use std::sync::mpsc::*;

pub mod fft;
pub mod mel;
pub mod spectral;
pub mod bpm;
pub mod windows;

mod filters {
    pub mod exp;
    pub mod gaussian;
}

// #[derive(Debug, Default)]
// pub struct AnalysisResult {
//     pub volume: f32,
// }

// trait: into proto audio analysis result oneof that can be sent to the frontend
// todo: make generic and allow multiple analysis results
pub trait Analyzer<T> {
    // no threading here as this should be transparent to the user
    // fn analyze_samples(&mut self, samples: T) -> Result<AnalysisResult>;
    /// analyzes samples of size [window_size, nchannels]
    fn analyze_samples(&mut self, samples: T) -> Result<AudioAnalysisResult>;
    /// the buffer window size for the analysis
    // fn window_size() -> usize;
    fn window_size(&self) -> usize;
    fn descriptor(&self) -> proto::grpc::AudioAnalyzerDescriptor;
    // fn analyze_stream(
    //     &self,
    //     input: mpsc::Receiver<Array<T, D>>,
    //     output: Option<mpsc::Sender<AnalysisResult>>
    // ) -> Result<AnalysisResult>;
    // todo: connect input
    // todo: connect output
}
