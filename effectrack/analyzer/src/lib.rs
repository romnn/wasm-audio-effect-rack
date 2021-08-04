use anyhow::Result;
use std::sync::mpsc::*;

pub mod fft;
pub mod mel;
pub mod spectral;
pub mod windows;

mod filters {
    pub mod exp;
    pub mod gaussian;
}

#[derive(Debug, Default)]
pub struct AnalysisResult {
    pub volume: f32,
}

// trait: into proto audio analysis result oneof that can be sent to the frontend
// todo: make generic and allow multiple analysis results
pub trait Analyzer<T> {
    // no threading here as this should be transparent to the user
    fn analyze_samples(&mut self, samples: T) -> Result<AnalysisResult>;
    // fn analyze_stream(
    //     &self,
    //     input: mpsc::Receiver<Array<T, D>>,
    //     output: Option<mpsc::Sender<AnalysisResult>>
    // ) -> Result<AnalysisResult>;
    // todo: connect input
    // todo: connect output
}
