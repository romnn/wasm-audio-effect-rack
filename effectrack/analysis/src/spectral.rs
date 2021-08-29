extern crate common;

use crate::fft::FFT;
use crate::filters::{exp::ExpSmoothingFilter, gaussian::GaussianFilter1d};
use crate::mel;
use crate::mel::{Hz, Mel, MelFilterBank};
use crate::windows::{HammingWindow, Window};
use crate::Analyzer;
use anyhow::Result;
use ndarray::parallel::prelude::*;
use ndarray::prelude::*;
use ndarray::{
    Array, ScalarOperand, Zip,
};
use num::pow::pow;
use num::traits::{Float, FloatConst, NumCast, One};
use proto::audio::analysis::audio_analysis_result;
use proto::audio::analysis::{AudioAnalysisResult, SpectralAudioAnalysisResult};
use std::error;
use std::fmt;

#[derive(Debug)]
enum SpectralAnalyzerError {
    InvalidParameter(String),
}

impl fmt::Display for SpectralAnalyzerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidParameter(msg) => {
                write!(f, "invalid parameter: {}", msg)
            }
        }
    }
}

impl error::Error for SpectralAnalyzerError {}

// todo: replace with proto
#[derive(Debug)]
pub struct SpectralAnalyzerOptions {
    pub mel_bands: usize,
    pub sample_rate: u32,
    pub nchannels: u16,
    pub fps: u16,
}

impl Default for SpectralAnalyzerOptions {
    fn default() -> Self {
        Self {
            nchannels: 2,
            sample_rate: 44100,
            fps: 60,
            mel_bands: 24,
        }
    }
}

#[derive(Debug)]
pub struct SpectralAnalyzer<T>
where
    T: Hz + Mel + Float + FloatConst,
{
    pub options: SpectralAnalyzerOptions,
    pub buffer_window_size: usize,
    mel_gain_exp_filter: ExpSmoothingFilter<T, Array1<T>>,
    mel_exp_filter: ExpSmoothingFilter<T, Array1<T>>,
    gain_exp_filter: ExpSmoothingFilter<T, Array1<T>>,
    gaussian_filter: GaussianFilter1d<T>,
    hann_window: Window<T>,
    padding: usize,
    mel_filterbank: mel::FilterBank<T, Array2<T>>,
    buffer: Array1<T>,
}

impl<T> SpectralAnalyzer<T>
where
    T: Hz + Mel + Float + FloatConst + Default + Send + std::fmt::Debug,
{
    // buffer size and channel
    pub fn new(options: SpectralAnalyzerOptions) -> Result<Self> {
        // let sample_rate_f: f32 = NumCast::from(options.sample_rate).unwrap();
        // let fps_f: f32 = NumCast::from(options.fps).unwrap();
        // let mel_samples: f32 = sample_rate_f * 1.0 / (2.0 * fps_f);
        // let mel_samples: usize = NumCast::from(mel_samples).unwrap();

        let buffer_window_size = 2048;
        let next_pwr_two: f32 = NumCast::from(buffer_window_size).unwrap();
        let next_pwr_two: usize = NumCast::from(next_pwr_two.log2().ceil()).unwrap();
        let next_pwr_two = pow(2, next_pwr_two);

        if next_pwr_two > 4096 {
            return Err(SpectralAnalyzerError::InvalidParameter(
                format!(
                    "window size mut be between 0 and 4096, got {}",
                    next_pwr_two
                )
                .to_string(),
            )
            .into());
        }

        let padding = next_pwr_two - buffer_window_size;
        let hann_window = Window::hamming(buffer_window_size);

        let mel_opts = mel::FilterBankParameters::<T> {
            num_mel_bands: options.mel_bands,
            freq_min: T::from(200).unwrap(),
            freq_max: T::from(12000).unwrap(),
            fft_window_size: next_pwr_two,
            sample_rate: options.sample_rate,
            htk: true,
            norm: None,
            ..mel::FilterBankParameters::default()
        };
        let mel_filterbank = mel::FilterBank::<T, Array2<T>>::new(Some(mel_opts));
        let gaussian_filter = GaussianFilter1d::default();
        let mel_gain_exp_filter = ExpSmoothingFilter::new(
            Array1::from(vec![T::from(1e-1).unwrap(); options.mel_bands]),
            T::from(0.01).unwrap(),
            T::from(0.99).unwrap(),
        )?;
        let mel_exp_filter = ExpSmoothingFilter::new(
            Array1::from(vec![T::from(1e-1).unwrap(); options.mel_bands]),
            T::from(0.5).unwrap(),
            T::from(0.99).unwrap(),
        )?;
        let gain_exp_filter = ExpSmoothingFilter::new(
            Array1::from(vec![T::from(1e-1).unwrap(); options.mel_bands]),
            T::from(0.01).unwrap(),
            T::from(0.99).unwrap(),
        )?;

        Ok(Self {
            buffer: Array1::<T>::zeros(100),
            buffer_window_size,
            options,
            mel_filterbank,
            mel_gain_exp_filter,
            mel_exp_filter,
            gain_exp_filter,
            gaussian_filter,
            hann_window,
            padding,
        })
    }
}

impl<T> Analyzer<Array2<T>> for SpectralAnalyzer<T>
where
    T: Float
        + FloatConst
        + One
        + Hz
        + Mel
        + Send
        + Sync
        + Default
        + std::fmt::Debug
        // + std::cmp::Ord
        + ScalarOperand,
{
    fn window_size(&self) -> usize {
        self.buffer_window_size
    }

    fn descriptor(&self) -> proto::grpc::AudioAnalyzerDescriptor {
        proto::grpc::AudioAnalyzerDescriptor {
            name: "SpectralAnalyzer".to_string(),
            input: None,
        }
    }

    fn analyze_samples(&mut self, mut samples: Array2<T>) -> Result<AudioAnalysisResult> {
        // todo: make this nicer with some chaining of processing
        // e.g. make mono, perform fft etc
        // let samples = samples.par_mapv_inplace(|v| v.abs()).collect();
        samples.par_mapv_inplace(|v| v.abs());
        // combine channels and choose the maximum
        let mut samples: Array1<T> = Zip::from(samples.axis_iter(Axis(1)))
            .par_map_collect(|row| row.iter().fold(T::zero(), |acc, v| acc.max(*v)));
        // let mut samples: Array1<T> = samples.map_axis(Axis(1), |row| {
        //     row.iter().fold(T::zero(), |acc, v| acc.max(*v))
        // });
        let volume = samples
            .into_par_iter()
            .map(|v| *v)
            .reduce_with(|acc, b| acc.max(b))
            .unwrap_or(T::zero());
        let volume: f32 = NumCast::from(volume).unwrap();
        // println!("volume: {:?}", volume);

        assert!(samples.len() == self.window_size());
        self.hann_window.apply(&mut samples);

        // pad with zeros until the next power of two
        samples.append(Axis(0), Array::zeros(self.padding).view())?;
        let ys = samples
            .fft()?
            // .slice(s![..samples.len() / 2])
            .mapv(|v| T::from(v.norm()).unwrap());
        let a = ys.insert_axis(Axis(1));

        let mel_weights = self.mel_filterbank.weights().t();
        // println!("mel.T: {:?}, ys(2d): {:?}", mel_weights.shape(), a.shape());
        let mel = a * mel_weights;

        let mel = mel
            .fold_axis(Axis(0), T::zero(), |acc, e| *acc + *e)
            .mapv(|e| pow(e, 2));

        let mel_gaussian = self.gaussian_filter.apply(&mel, T::one());

        let gain_update = mel_gaussian
            .to_owned()
            .to_vec()
            .iter()
            .fold(T::neg_infinity(), |acc, &v| acc.max(v));

        let mel_gain = self.mel_gain_exp_filter.update_scalar(gain_update);
        let mel = mel / mel_gain;

        let mel = self.mel_exp_filter.update(&mel);
        let mel = mel.mapv(|v| v.powi(2));
        let gain = self.gain_exp_filter.update(&mel);
        let mel = mel / gain;

        let result = SpectralAudioAnalysisResult {
            volume: volume,
            num_mel_bands: NumCast::from(self.options.mel_bands).unwrap(),
            mel_bands: mel.mapv(|v| NumCast::from(v).unwrap()).to_vec(),
        };
        Ok(AudioAnalysisResult {
            seq_num: 0,
            window_size: NumCast::from(self.window_size()).unwrap(),
            result: Some(audio_analysis_result::Result::Spectral(result)),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::spectral::{GaussianFilter1d, SpectralAnalyzer};
    use ndarray::prelude::*;
    use ndarray::Array;
    use ndarray::{concatenate, stack, RemoveAxis, Slice};

    fn gen_samples<F>(length: usize, func: F) -> Array1<f32>
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
        if false {
            let (rows, cols) = (1, 5);
            let a = Array1::<f32>::range(0.0, (cols * rows) as f32, 1.0)
                .into_shape((rows, cols))
                .unwrap();
            let a = stack![Axis(1), a, a, a].into_shape((3, 5)).unwrap();
            let mut w = Array2::<f32>::ones((3, 3));
            w[[0, 0]] = 0.;
            w[[0, 2]] = 0.;
            // w[[1, 0]] = 0.;
            // w[[1, 2]] = 0.;
            w[[2, 0]] = 0.;
            w[[2, 2]] = 0.;
            println!("{:#?}", a);
            println!("{:#?}", w);
            let b = GaussianFilter1d::correlate(&a, w);
            println!("{:#?}", b);

            assert_eq!(true, false, "all good");
            return;
        }
        let gen_func = |x: f32| (0.7 * (x + 100.0).sin() + 0.3 * x.sin());

        let sample_rate = 44100;
        let fps = 60;
        let nchannels = 1;

        let sample_count = 735;

        let samples = gen_samples(sample_count, gen_func);
        assert!(
            samples.slice(s![..4, 0]).abs_diff_eq(
                &array!(-0.35445595, 0.56885934, 0.969168, 0.47842807),
                f32::EPSILON
            ),
            "initial samples are not as expected"
        );
        let result = SpectralAnalyzer::temp(samples, sample_rate, nchannels, fps).expect("works");
        assert!(
            (result.volume - 0.97065383).abs() <= f32::EPSILON,
            "wrong volume"
        );
        assert_eq!(true, false, "all good");
    }
}
