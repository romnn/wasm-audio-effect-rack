use anyhow::Result;

pub mod spectral;
pub mod mel;
pub mod windows;
pub mod fft;

#[derive(Debug, Default)]
pub struct AnalysisResult {
    pub volume: f32,
}

// todo: make generic and allow multiple analysis results
pub trait Analyzer {
    fn analyze(&self, samples: &[f32]) -> Result<AnalysisResult>;
}
