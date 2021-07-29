use crate::windows::{HammingWindow, Window};
use crate::{AnalysisResult, Analyzer};
use anyhow::Result;
use ndarray::prelude::*;
use ndarray::{concatenate, Array, Ix, RemoveAxis, Slice};
use num::{traits::FloatConst, Float, NumCast};

#[derive(Default, Debug)]
pub struct SpectralAnalyzerOptions {}

#[derive(Default, Debug)]
pub struct SpectralAnalyzer {}
impl SpectralAnalyzer {
    pub fn new(options: SpectralAnalyzerOptions) -> Self {
        Self {}
    }
}

impl Analyzer for SpectralAnalyzer {
    fn analyze(&self, samples: &[f32]) -> Result<AnalysisResult> {
        Ok(AnalysisResult::default())
    }
}

impl SpectralAnalyzer {
    fn temp<T>(samples: Array2<T>) -> Result<AnalysisResult>
    where
        T: Float + FloatConst + std::ops::MulAssign + std::fmt::Debug,
    {
        println!("initial samples: {:?}", samples);

        // make mono
        let samples = samples.mapv(|v| v.abs());

        // combine channels and choose the maximum
        let mut samples = samples.map_axis(Axis(1), |row| {
            row.iter().fold(T::zero(), |acc, v| acc.max(*v))
        });

        let volume = samples
            .iter()
            .fold(T::neg_infinity(), |a: T, &b| a.max(b))
            .to_f32()
            .unwrap();
        // .unwrap_or(0f32);

        println!("volume: {:?}", volume);
        let window_size: u32 = NumCast::from(samples.len()).unwrap();

        // pad with zeros until the next power of two
        let next_power_of_two: f32 = NumCast::from(window_size).unwrap();
        let next_power_of_two: u32 = NumCast::from(next_power_of_two.log2().ceil()).unwrap();
        let padding = 2u32.pow(next_power_of_two) - window_size;

        println!("next_power_of_two: {:?}", next_power_of_two);
        println!("padding: {:?}", padding);

        // println!("samples: {:?}", samples.into_shape((samples.len(), 1)).unwrap());
        let hwindow = Window::hamming(samples.len());
        hwindow.apply(&mut samples);
        // samples *= hwindow;
        println!("after hamming: {:?}", samples.view().insert_axis(Axis(1)));

        Ok(AnalysisResult {
            // volume: volume.to_f64().unwrap(),
            volume: volume,
            ..AnalysisResult::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::spectral::SpectralAnalyzer;
    use ndarray::prelude::*;
    use ndarray::Array;
    use ndarray::{concatenate, RemoveAxis, Slice};

    fn gen_samples<F>(length: usize, func: F) -> Array2<f32>
    where
        F: Fn(f32) -> f32,
    {
        Array::range(0., length as f32, 1.0)
            .mapv(|i| func(i))
            .into_shape([length, 1])
            .unwrap()
    }

    #[test]
    fn spectral_pipeline() {
        let gen_func = |x: f32| (0.7 * (x + 100.0).sin() + 0.3 * x.sin());
        let samples = gen_samples(10, gen_func);
        assert!(
            samples.slice(s![..4, 0]).abs_diff_eq(
                &array!(-0.35445595, 0.56885934, 0.969168, 0.47842807),
                f32::EPSILON
            ),
            "initial samples are not as expected"
        );
        let result = SpectralAnalyzer::temp(samples).expect("works");
        assert!(
            (result.volume - 0.969167981998589).abs() <= f32::EPSILON,
            "wrong volume"
        );
        assert_eq!(true, false, "all good");
    }
}
