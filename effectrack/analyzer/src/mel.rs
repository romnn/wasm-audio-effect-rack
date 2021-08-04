use anyhow::Result;
use ndarray::prelude::*;
use ndarray::{concatenate, Array, Ix, RemoveAxis, Slice};
use num::{traits::FloatConst, Float, NumCast, Zero};
use std::error;
use std::fmt;

// some inspiration from https://github.com/songww/mel-filter/blob/main/src/lib.rs
// also from the python implementation

#[derive(Debug)]
enum MelFilterBankError {
    MissingParameter(String),
}

impl fmt::Display for MelFilterBankError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::MissingParameter(param) => write!(f, "missing parameter: {}", param),
        }
    }
}

impl error::Error for MelFilterBankError {}

#[derive(Debug)]
pub enum NormalizationFactor {
    /// Leave all the triangles aiming for a peak value of 1.0
    None,
    /// divide the triangular mel weights by the width of the mel band (area normalization).
    One,
}

// Compute an array of acoustic frequencies tuned to the mel scale.

// The mel scale is a quasi-logarithmic function of acoustic frequency designed such that perceptually similar pitch intervals (e.g. octaves) appear equal in width over the full hearing range.

// Because the definition of the mel scale is conditioned by a finite number of subjective psychoaoustical experiments, several implementations coexist in the audio signal processing literature 1. By default, librosa replicates the behavior of the well-established MATLAB Auditory Toolbox of Slaney 2. According to this default implementation, the conversion from Hertz to mel is linear below 1 kHz and logarithmic above 1 kHz. Another available implementation replicates the Hidden Markov Toolkit 3 (HTK) according to the following formula:

// pub trait Hz: Float + NumCast {
pub trait Hz: Float {
    fn to_mel(&self, htk: bool) -> Self {
        if htk {
            // use HTK formula to convert Hz to mel
            // this originates from the Hidden Markov Toolkit 3 (HTK)
            let mel = *self / Self::from(700.0).unwrap();
            let mel = (Self::one() + mel).log10();
            let mel = mel * Self::from(2595.0).unwrap();
            return mel;
        }
        // Use well-established MATLAB Slaney 2 Auditory Toolbox formula
        // Here, conversion from Hertz to mel is
        //  -> linear below 1 kHz
        //  -> logarithmic above 1 kHz
        // Fill in the linear part
        let f_min = Self::zero();
        let f_sp = Self::from(200.0 / 3.).unwrap();
        // beginning of log region (Hz)
        let min_log_hz = Self::from(1000.0).unwrap();
        let min_log_mel = (min_log_hz - f_min) / f_sp;
        // step size for log region
        let logstep = Self::from((6.4).ln() / 27.0).unwrap();
        if self >= &min_log_hz {
            min_log_mel + (*self / min_log_hz).ln() / logstep
        } else {
            (*self - f_min) / f_sp
        }
    }
}

impl Hz for f32 {}
impl Hz for f64 {}

pub trait Mel: Float + NumCast + Zero + Clone {
    fn to_hz(&self, htk: bool) -> Self {
        if htk {
            // use the HTK formula, see above
            let ten = Self::from(10.0).unwrap();
            let seven_hundred: Self = Self::from(700.0).unwrap();
            let hz = ten.powf(*self / Self::from(2595.0).unwrap());
            let hz = seven_hundred * (hz - Self::one());
            return hz;
        }
        // use the MATLAB Slaney 2 Auditory Toolbox formula, see above
        // Fill in the linear scale
        let f_min = Self::zero();
        let f_sp = Self::from(200.0 / 3.0).unwrap();

        // And now the logarithmic scale (above 1kHz)
        let min_log_hz = Self::from(1000.0).unwrap();
        let min_log_mel = (min_log_hz - f_min) / f_sp;
        // step size for log region
        let logstep = Self::from((6.4).ln() / 27.0).unwrap();
        if self >= &min_log_mel {
            min_log_hz * Self::exp(logstep * (*self - min_log_mel))
        } else {
            f_min + f_sp * *self
        }
    }
}

impl Mel for f32 {}
impl Mel for f64 {}

pub trait FilterBankMat<Mel> {
    // fn as_slice(&self) -> &[Mel];
    // fn as_mut_slice(&mut self) -> &mut [Mel];

    fn zeros(shape: &[usize; 2]) -> Self;
    // fn zeros(shape: &[usize; 2]) -> Array2<Mel>;

    // todo: think about property access that has to be possible somehow?
    // todo: dont call them rows, better mel / freq or so
    fn row_mut(&mut self, idx: usize) -> &mut [Mel];
    // fn row_mut(&mut self, idx: usize) -> ArrayViewMut1<Mel>; // &mut [Mel];
    fn shape(&self) -> &[usize];
    fn to_vec(self) -> Vec<Mel>;

    // fn shape(&self) -> Shape;
    // fn zeros(shape: &[usize; 2]) -> Self;
    // fn row_mut(&mut self, idx: usize) -> &mut [Mel];
}

impl<F: Mel + Zero> FilterBankMat<F> for Array2<F> {
    #[inline]
    fn to_vec(self) -> Vec<F> {
        // Self::Backend::to_vec(self) // .into_slice().unwrap()
        // Self::Backend::to_vec(self) // .into_slice().unwrap()
        // Self::zeros(shape.to_owned())
        // Self::
        // Array2::<F>::to_vec(self) // .into_slice().unwrap()
        self.into_raw_vec()
        // self.to_vec();
        // vec![]
    }
    // #[inline]
    // fn as_mut_slice(&mut self) -> &mut [F] {
    //     Array2::<F>::as_slice_memory_order_mut(self).unwrap()
    // }
    #[inline]
    fn shape(&self) -> &[usize] {
        self.shape().as_ref()
        // self.shape()
    }

    #[inline]
    fn zeros(shape: &[usize; 2]) -> Self
// where
        // Self: Sized,
    {
        Self::zeros(shape.to_owned())
    }

    #[inline]
    fn row_mut(&mut self, idx: usize) -> &mut [F] {
        // fn row_mut(&mut self, idx: usize) -> ArrayViewMut1<F> {
        // &mut [F] {
        Array2::<F>::row_mut(self, idx).into_slice().unwrap()
        // Array2::<F>::row_mut(self, idx).into_slice().unwrap()
        // Array2::<F>::row_mut(self, idx) // .into_slice().unwrap()
        // self.row_mut(idx) // .into_slice().unwrap()
    }
}

#[derive(Debug)]
pub struct FilterBankParameters<F: Hz> {
    /// Number of mel bands. Number of rows in melmat.
    pub num_mel_bands: usize,
    /// Minimum frequency for the first band.
    pub freq_min: F,
    /// Maximum frequency for the last band.
    pub freq_max: F,
    /// Size of the fft.
    pub fft_size: usize,
    /// Number of fft-frequency bands. Otherwise (fft_size/2)+1 is chosen.
    pub num_fft_bands: Option<usize>,
    /// Sample rate for the signals that will be used.
    pub sample_rate: usize,
    /// Use HTK formula for converting mel and hz
    pub htk: bool,
    /// Normalization
    pub norm: Option<NormalizationFactor>,
    // /// Number of mel bands. Number of rows in melmat.
    // pub num_mel_bands: Option<usize>,
    // /// Minimum frequency for the first band.
    // pub freq_min: Option<F>,
    // /// Maximum frequency for the last band.
    // pub freq_max: Option<F>,
    // /// Size of the fft.
    // // pub fft_size: Option<usize>,
    // /// Number of fft-frequency bands. This ist (fft_size/2)+1 !
    // pub num_fft_bands: Option<usize>,
    // /// Sample rate for the signals that will be used.
    // pub sample_rate: Option<usize>,
    // /// Use HTK formula for converting mel and hz
    // pub htk: Option<bool>,
}

impl<F: Hz> Default for FilterBankParameters<F> {
    fn default() -> Self {
        Self {
            num_mel_bands: 128,               // or 128
            freq_min: F::from(64).unwrap(),   // or 64
            freq_max: F::from(8000).unwrap(), // or 8000
            fft_size: 1024,
            num_fft_bands: None, // (fft_size/2)+1, so 513 in this case
            sample_rate: 44100,  // or 16000
            htk: false,
            norm: Some(NormalizationFactor::One),
            // num_mel_bands: Some(12), // or 128
            // freq_min: Some(F::from(64).unwrap()),
            // freq_max: Some(F::from(8000).unwrap()),
            // // fft_size: Some(1024),
            // num_fft_bands: Some(513), // (fft_size/2)+1
            // sample_rate: Some(16000),
            // htk: Some(true),
        }
    }
}
// num_mel_bands: Some(12), // or 128
// freq_min: Some(F::from(64).unwrap()),
// freq_max: Some(F::from(8000).unwrap()),
// fft_size: Some(1024),
// num_fft_bands: Some(513), // (fft_size/2)+1
// sample_rate: Some(16000),
// htk: Some(true),

// impl<F: Hz> FilterBankParameters<F> {
//     fn infer_missing(&mut self) {
//         let default_params = Self::default();
//         self.freq_max = self.freq_max.or_else(|| {
//             // we can compute the default freq_max from the sample rate
//             self.sample_rate.map(|sr| {
//                 let sr = F::from(sr).unwrap();
//                 sr * F::from(2.0).unwrap()
//             })
//         });
//         self.num_fft_bands = self.num_fft_bands.or_else(|| {
//             // we can compute the num of fft bands from the fft size
//             self.fft_size.map(|fft_size| (fft_size / 2) + 1)
//         });

//         self.num_mel_bands = self.num_mel_bands.or(default_params.num_mel_bands);
//         self.freq_min = self.freq_min.or(default_params.freq_min);
//         self.freq_max = self.freq_max.or(default_params.freq_max);
//         self.num_fft_bands = self.num_fft_bands.or(default_params.num_fft_bands);
//         self.htk = self.htk.or(default_params.htk);
//     }
// }

pub trait MelFilterBank<F: Hz, Mat: FilterBankMat<F>> {
    fn new(parameters: Option<FilterBankParameters<F>>) -> Self;
    // where
    // Self: Sized;
    fn weights(&self) -> &Mat;
}

// pub struct FilterBank<T: Mel, Mat: FilterBankMat<Mel>> {
#[derive(Debug)]
pub struct FilterBank<F, Mat>
where
    F: Mel + Hz,
{
    weights: Mat,
    fft_frequencies: Array1<F>,
    mel_frequencies: Array1<F>,
}

// Array2<F>
impl<F, Mat> MelFilterBank<F, Mat> for FilterBank<F, Mat>
// impl MelFilterBank<Mat> for FilterBank<Mat>
where
    F: Hz + Mel + Zero + std::fmt::Debug,
    Mat: FilterBankMat<F>,
    // Mat: AsRef<FilterBankMat<F>>,
{
    fn new(parameters: Option<FilterBankParameters<F>>) -> Self
// where
        // Self: Sized,
    {
        let default_params = FilterBankParameters::default();
        let params = parameters.unwrap_or(default_params);

        let default_num_fft_bands = 1 + params.fft_size / 2;
        let num_fft_bands = params.num_fft_bands.unwrap_or(default_num_fft_bands);

        let mut weights = Mat::zeros(&[params.num_mel_bands, num_fft_bands]);

        // center freqs of each FFT bin
        let fft_freqs = fft_frequencies::<F>(params.sample_rate, num_fft_bands);
        // println!(
        //     "fft_freqs are {:?} {:?}",
        //     fft_freqs.shape(),
        //     fft_freqs.slice(s![..10])
        // );
        // center freqs of mel bands - uniformly spaced between limits
        let mel_freqs = mel_frequencies(
            params.num_mel_bands + 2,
            params.freq_min,
            params.freq_max,
            params.htk,
        );
        // println!(
        //     "mel_freqs are {:?} {:?}",
        //     mel_freqs.shape(),
        //     mel_freqs.slice(s![..10])
        // );

        // fdiff is the width between each pair of adjacent mel centers
        let mut fdiff = mel_freqs.slice(s![..-1]).to_owned();
        fdiff.zip_mut_with(&mel_freqs.slice(s![1..]), |mut x, y| *x = *y - *x);
        // println!(
        //     "fdiff are {:?} {:?}",
        //     fdiff.shape(),
        //     fdiff.slice(s![..10])
        // );

        let mut ramps = Array2::<F>::zeros([mel_freqs.len(), fft_freqs.len()]);
        for ((mel_idx, freq_idx), ramp) in ramps.indexed_iter_mut() {
            *ramp = mel_freqs[mel_idx] - fft_freqs[freq_idx];
        }
        // println!(
        //     "ramps are {:?} {:?}",
        //     ramps.shape(),
        //     ramps.slice(s![..10,..])
        // );

        for mel_idx in 0..params.num_mel_bands {
            for freq_idx in 0..num_fft_bands {
                // lower and upper slopes for all bins
                let lower = -ramps[[mel_idx, freq_idx]] / fdiff[mel_idx];
                let upper = ramps[[mel_idx + 2, freq_idx]] / fdiff[mel_idx + 1];
                // println!("lower: {:?}", lower);
                // println!("upper: {:?}", upper);

                // then intersect them with each other and zero
                let intersect = F::zero().max(lower.min(upper));
                // println!("intersect: {:?}", intersect);
                weights.row_mut(mel_idx)[freq_idx] = intersect;
            }
        }

        if let Some(norm) = params.norm {
            panic!("todo");
            match norm {
                NormalizationFactor::One => {
                    // Slaney-style mel is scaled to be approx constant energy per channel
                    // enorm = 2.0 / (mel_f[2:n_mels+2] - mel_f[:n_mels])
                    // weights *= enorm[:, np.newaxis]

                    let two = F::from(2.).unwrap();
                    // let mut enorms = mel_freqs.slice(s![2..params.num_mel_bands + 2]).to_owned();
                    let mut enorms = mel_freqs.slice(s![2..]).to_owned();
                    enorms.zip_mut_with(&mel_freqs.slice(s![..-2]), |x, y| {
                        *x = two / (*x - *y);
                    });

                    for (idx, enorm) in enorms.indexed_iter() {
                        weights
                            .row_mut(idx)
                            .iter_mut()
                            .map(|v| *v = *v * *enorm)
                            .collect::<()>();
                        // .mapv_inplace(|v| v * *enorm);
                    }
                }
                _ => {}
            };
        };

        // let (center_mel_freqs, lower_edges_mel, upper_edges_mel) = mel_frequencies_py(
        //     params.num_mel_bands, //.unwrap_or(0),
        //     params.freq_min,      //.unwrap_or(F::zero()),
        //     params.freq_max,      // .unwrap_or(F::zero()),
        //     params.num_fft_bands, //.unwrap_or(0),
        //     params.htk,           //.unwrap_or(false),
        // );
        // center_frequencies_hz = mel_to_hertz(center_frequencies_mel)
        // let center_hz_freqs = center_mel_freqs.map
        // lower_edges_hz = mel_to_hertz(lower_edges_mel)
        // upper_edges_hz = mel_to_hertz(upper_edges_mel)
        // freqs = np.linspace(0.0, sample_rate / 2.0, num_fft_bands)
        // melmat = np.zeros((num_mel_bands, num_fft_bands))

        // (center_frequencies_mel, freqs)
        Self {
            weights,
            fft_frequencies: fft_freqs,
            mel_frequencies: mel_freqs,
        }
    }

    // fn matrix(&self) -> &Array2<F> {
    fn weights(&self) -> &Mat {
        &self.weights
    }
}

pub fn fft_frequencies<F: Hz>(sample_rate: usize, num_fft_bands: usize) -> Array1<F> {
    Array::linspace(
        F::zero(),
        F::from(sample_rate).unwrap() / F::from(2).unwrap(),
        num_fft_bands,
    )
}

pub fn mel_frequencies<T: Mel + Hz>(n_mels: usize, fmin: T, fmax: T, htk: bool) -> Array1<T> {
    let min_mel = fmin.to_mel(htk);
    let max_mel = fmax.to_mel(htk);
    Array::linspace(min_mel, max_mel, n_mels).map(|mel| mel.to_hz(htk))
}

// Returns centerfrequencies and band edges for a mel filter bank
// fn mel_frequencies_py<F: Hz>(
//     num_bands: usize,
//     freq_min: F,
//     freq_max: F,
//     num_fft_bands: usize,
//     htk: bool,
// ) -> (Array1<F>, Array1<F>, Array1<F>) {
//     let mel_max = freq_max.to_mel(htk);
//     let mel_min = freq_min.to_mel(htk);
//     let delta_mel = (mel_max - mel_min).abs();
//     let delta_mel = delta_mel / F::from(num_bands + 1).unwrap();
//     let mut mel_freqs = Array::range(F::zero(), F::from(num_bands + 2).unwrap(), F::one());
//     mel_freqs.mapv_inplace(|v| v * delta_mel + mel_min);
//     let lower_edges_mel = mel_freqs.slice(s![..-2]).to_owned();
//     let upper_edges_mel = mel_freqs.slice(s![2..]).to_owned();
//     let center_mel_freqs = mel_freqs.slice(s![1..-1]).to_owned();
//     (center_mel_freqs, lower_edges_mel, upper_edges_mel)
// }
