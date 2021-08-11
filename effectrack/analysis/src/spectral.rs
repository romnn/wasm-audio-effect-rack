extern crate common;

use crate::fft::FFT;
use crate::filters::{exp::ExpSmoothingFilter, gaussian::GaussianFilter1d};
use crate::mel;
use crate::mel::{FilterBankMat, Hz, Mel, MelFilterBank};
use crate::windows::{HammingWindow, HannWindow, Window};
use crate::Analyzer;
use anyhow::Result;
use common::sorting::DebugMinMax;
use ndarray::parallel::prelude::*;
use ndarray::prelude::*;
use ndarray::{
    concatenate, indices, Array, IntoDimension, Ix, NdIndex, RemoveAxis, ScalarOperand, Slice, Zip,
};
use num::pow::pow;
use num::traits::{Float, FloatConst, NumCast, NumOps, One};
// use proto::audio::analysis::audio_analysis_result::Result as SpecificAudioAnalysisResult;
use proto::audio::analysis::audio_analysis_result;
// ::Result as SpecificAudioAnalysisResult;
use proto::audio::analysis::{AudioAnalysisResult, SpectralAudioAnalysisResult};
use std::error;
use std::fmt;
use std::sync::mpsc::*;

#[derive(Debug)]
enum SpectralAnalyzerError {
    InvalidParameter(String),
    MissingParameter(String),
}

impl fmt::Display for SpectralAnalyzerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidParameter(msg) => {
                write!(f, "invalid parameter: {}", msg)
            }
            Self::MissingParameter(msg) => {
                write!(f, "missing parameter: {}", msg)
            }
        }
    }
}

impl error::Error for SpectralAnalyzerError {}

// todo: replace with proto
#[derive(Debug)]
pub struct SpectralAnalyzerOptions {
    // pub window_size: Option<usize>,
    pub mel_bands: usize,
    pub sample_rate: u32,
    pub nchannels: u16,
    pub fps: u16,
}

impl Default for SpectralAnalyzerOptions {
    fn default() -> Self {
        Self {
            // window_size: 1024,
            // window_size: None,
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
        let sample_rate_f: f32 = NumCast::from(options.sample_rate).unwrap();
        let fps_f: f32 = NumCast::from(options.fps).unwrap();

        let mel_samples: f32 = sample_rate_f * 1.0 / (2.0 * fps_f);
        let mel_samples: usize = NumCast::from(mel_samples).unwrap();

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

        // let padding = 2usize.powi(next_pwr_two) - buffer_window_size;
        // println!("nptwo {}, ws {}", next_pwr_two, buffer_window_size);
        let padding = next_pwr_two - buffer_window_size;
        let hann_window = Window::hamming(buffer_window_size);

        // println!("mel samples: {}", mel_samples);
        // println!("mel samples: {}", mel_samples);

        let mel_opts = mel::FilterBankParameters::<T> {
            num_mel_bands: options.mel_bands,
            freq_min: T::from(200).unwrap(),
            freq_max: T::from(12000).unwrap(),
            fft_window_size: next_pwr_two,
            // fft_size: 1024,                   // we dont care about it
            // num_fft_bands: Some(mel_samples), // (fft_size/2)+1
            sample_rate: options.sample_rate,
            htk: true,
            norm: None,
            ..mel::FilterBankParameters::default()
        };
        let mel_filterbank = mel::FilterBank::<T, Array2<T>>::new(Some(mel_opts));
        let gaussian_filter = GaussianFilter1d::default();
        let mut mel_gain_exp_filter = ExpSmoothingFilter::new(
            Array1::from(vec![T::from(1e-1).unwrap(); options.mel_bands]),
            T::from(0.01).unwrap(),
            T::from(0.99).unwrap(),
        )?;
        let mut mel_exp_filter = ExpSmoothingFilter::new(
            Array1::from(vec![T::from(1e-1).unwrap(); options.mel_bands]),
            T::from(0.5).unwrap(),
            T::from(0.99).unwrap(),
        )?;
        let mut gain_exp_filter = ExpSmoothingFilter::new(
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
        + ScalarOperand,
{
    fn window_size(&self) -> usize {
        self.buffer_window_size
    }

    fn descriptor(&self) -> proto::grpc::AudioAnalyzerDescriptor {
        proto::grpc::AudioAnalyzerDescriptor {
            name: "SpectralAnalyzer".to_string(),
            input: None,
            // input: self.input_desciptor
        }
    }

    fn analyze_samples(&mut self, samples: Array2<T>) -> Result<AudioAnalysisResult> {
        // todo: make this nicer with some chaining of processing
        // e.g. make mono, perform fft etc
        let samples = samples.mapv(|v| v.abs());
        // combine channels and choose the maximum
        let mut samples: Array1<T> = samples.map_axis(Axis(1), |row| {
            row.iter().fold(T::zero(), |acc, v| acc.max(*v))
        });
        let volume = samples
            .iter()
            .fold(T::neg_infinity(), |acc: T, &b| acc.max(b));
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

        let mel_gaussian = self.gaussian_filter.apply(&mel, Axis(0), T::one());

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
        // let mel = mel * T::from(255.0).unwrap();

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

// impl SpectralAnalyzer<T> {
//     fn temp<D>(
//         samples: Array<T, D>,
//         sample_rate: u32,
//         nchannels: u32,
//         fps: u32,
//     ) -> Result<AnalysisResult>
//     where
//         T: Float
//             + FloatConst
//             + One
//             + Hz
//             + Mel
//             + Send
//             + Sync
//             + Default
//             + std::fmt::Debug
//             + ScalarOperand,
//         D: Dimension,
//     {
//         // println!("initial samples: {:?}", samples);

//         // todo: make this nicer with some chaining of processing, e.g. make mono, perform fft etc
//         // make mono
//         let samples = samples.mapv(|v| v.abs());

//         // combine channels and choose the maximum
//         let mut samples: Array1<T> = samples.map_axis(Axis(1), |row| {
//             row.iter().fold(T::zero(), |acc, v| acc.max(*v))
//         });

//         let volume = samples
//             .iter()
//             .fold(T::neg_infinity(), |acc: T, &b| acc.max(b))
//             .to_f32()
//             .unwrap();
//         // .unwrap_or(0f32);

//         println!("volume: {:?}", volume);
//         let window_size: u32 = NumCast::from(samples.len()).unwrap();

//         // pad with zeros until the next power of two
//         let next_power_of_two: f32 = NumCast::from(window_size).unwrap();
//         let next_power_of_two: u32 = NumCast::from(next_power_of_two.log2().ceil()).unwrap();
//         let padding = 2u32.pow(next_power_of_two) - window_size;

//         println!("next_power_of_two: {:?}", next_power_of_two);
//         println!("padding: {:?}", padding);

//         // println!("samples: {:?}", samples.into_shape((samples.len(), 1)).unwrap());
//         let window_size = samples.len();
//         let hwindow = Window::hamming(window_size);
//         hwindow.apply(&mut samples);
//         println!(
//             "after hamming: {:?}",
//             samples.slice(s![..5]).view().insert_axis(Axis(1))
//         );

//         samples
//             .append(Axis(0), Array::zeros(padding as usize).view())
//             .unwrap();
//         println!(
//             "after padding: {:?}",
//             samples.slice(s![..5]).view().insert_axis(Axis(1))
//         );

//         let ys = samples
//             .fft()?
//             .slice(s![..window_size / 2])
//             .mapv(|v| T::from(v.norm()).unwrap());
//         println!("fft ys: {:?}", ys.shape());
//         println!(
//             "fft ys: {:?}",
//             ys.slice(s![..5]).view().insert_axis(Axis(1))
//         );

//         let a = ys.insert_axis(Axis(1));
//         // let a = a.t();
//         println!(
//             "ys.T {:?}",
//             // a.shape(),
//             a.slice(s![..5, ..])
//         );

//         let sample_rate_f: f32 = NumCast::from(sample_rate).unwrap();
//         let fps_f: f32 = NumCast::from(fps).unwrap();
//         let mel_samples: f32 = sample_rate_f * 1.0 / (2.0 * fps_f);
//         let mel_samples: usize = NumCast::from(mel_samples).unwrap();
//         println!("mel samples: {:?}", mel_samples);

//         let num_fft_bands = mel_samples;
//         let num_mel_bands = 24;
//         let mel_opts = mel::FilterBankParameters::<T> {
//             num_mel_bands: num_mel_bands,
//             freq_min: T::from(200).unwrap(),
//             freq_max: T::from(12000).unwrap(),
//             fft_size: 1024,                   // we dont care about it
//             num_fft_bands: Some(mel_samples), // (fft_size/2)+1
//             sample_rate: 44100,
//             htk: true,
//             norm: None,
//             ..mel::FilterBankParameters::default()
//         };
//         println!("mel options: {:?}", mel_opts);
//         let mel_filterbank = mel::FilterBank::<T, Array2<T>>::new(Some(mel_opts));
//         let weights = mel_filterbank.weights().t();
//         println!(
//             "mel_y.T {:?} {:?} \n (min: {:?}, max: {:?})",
//             weights.shape(),
//             weights.slice(s![..5, ..]),
//             weights.to_owned().to_vec().debug_min(),
//             weights.to_owned().to_vec().debug_max(),
//         );
//         let mel = a * weights;
//         println!(
//             "mel_y {:?} {:?} \n (min: {:?}, max: {:?})",
//             mel.shape(),
//             mel.slice(s![..5, ..]),
//             mel.to_owned().to_vec().debug_min(),
//             mel.to_owned().to_vec().debug_max(),
//         );

//         let mel = mel
//             .fold_axis(Axis(0), T::zero(), |acc, e| *acc + *e)
//             .mapv(|e| pow(e, 2));
//         println!(
//             "mel {:?} {:?} \n (min: {:?}, max: {:?})",
//             mel.shape(),
//             mel.slice(s![..5]),
//             mel.to_owned().to_vec().debug_min(),
//             mel.to_owned().to_vec().debug_max(),
//         );

//         let filter = GaussianFilter1d::default();
//         let mel_gaussian = filter.apply(&mel, Axis(0), T::one());
//         println!(
//             "gaussian mel {:?} {:?} \n (min: {:?}, max: {:?})",
//             mel_gaussian.shape(),
//             mel_gaussian.slice(s![..5]),
//             mel_gaussian.to_owned().to_vec().debug_min(),
//             mel_gaussian.to_owned().to_vec().debug_max(),
//         );

//         let gain_update = mel_gaussian
//             .to_owned()
//             .to_vec()
//             .iter()
//             .fold(T::neg_infinity(), |acc, &v| acc.max(v));
//         println!("max mel {:?}", gain_update);

//         let mut mel_gain_filter = ExpSmoothingFilter::new(
//             Array1::from(vec![T::from(1e-1).unwrap(); num_mel_bands]),
//             T::from(0.01).unwrap(),
//             T::from(0.99).unwrap(),
//         )?;
//         let mel_gain = mel_gain_filter.update_scalar(gain_update);
//         println!(
//             "mel gain value {:?} {:?} \n (min: {:?}, max: {:?})",
//             mel_gain.shape(),
//             mel_gain.slice(s![..5]),
//             mel_gain.to_owned().to_vec().debug_min(),
//             mel_gain.to_owned().to_vec().debug_max(),
//         );

//         println!(
//             "mel shape {:?} mel gain shape {:?}",
//             mel.shape(),
//             mel_gain.shape(),
//         );

//         let mel = mel / mel_gain;
//         println!(
//             "mel after gain norm {:?} {:?} \n (min: {:?}, max: {:?})",
//             mel.shape(),
//             mel.slice(s![..5]),
//             mel.to_owned().to_vec().debug_min(),
//             mel.to_owned().to_vec().debug_max(),
//         );

//         let mut mel_filter = ExpSmoothingFilter::new(
//             Array1::from(vec![T::from(1e-1).unwrap(); num_mel_bands]),
//             T::from(0.5).unwrap(),
//             T::from(0.99).unwrap(),
//         )?;
//         let mel = mel_filter.update(&mel);
//         println!(
//             "mel after smoothing: {:?} {:?} \n (min: {:?}, max: {:?})",
//             mel.shape(),
//             mel.slice(s![..5]),
//             mel.to_owned().to_vec().debug_min(),
//             mel.to_owned().to_vec().debug_max(),
//         );

//         // let mel = pow(*mel, 2);
//         let mel = mel.mapv(|v| v.powi(2));
//         let mut gain_filter = ExpSmoothingFilter::new(
//             Array1::from(vec![T::from(1e-1).unwrap(); num_mel_bands]),
//             T::from(0.01).unwrap(),
//             T::from(0.99).unwrap(),
//         )?;
//         let gain = gain_filter.update(&mel);
//         let mel = mel / gain;
//         let mel = mel * T::from(255.0).unwrap();

//         Ok(AnalysisResult {
//             // volume: volume.to_f64().unwrap(),
//             volume: volume,
//             ..AnalysisResult::default()
//         })
//     }
// }

#[cfg(test)]
mod tests {
    use crate::spectral::{GaussianFilter1d, SpectralAnalyzer};
    use ndarray::prelude::*;
    use ndarray::Array;
    use ndarray::{concatenate, stack, RemoveAxis, Slice};

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
