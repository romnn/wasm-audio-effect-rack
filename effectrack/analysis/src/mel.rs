use ndarray::prelude::*;
use ndarray::{Array};
use num::{Float, NumCast, Zero};

// some inspiration from https://github.com/songww/mel-filter/blob/main/src/lib.rs
// also from the python implementation

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
    fn zeros(shape: &[usize; 2]) -> Self;

    fn row_mut(&mut self, idx: usize) -> &mut [Mel];
    fn shape(&self) -> &[usize];
    fn to_vec(self) -> Vec<Mel>;
}

impl<F: Mel + Zero> FilterBankMat<F> for Array2<F> {
    #[inline]
    fn to_vec(self) -> Vec<F> {
        self.into_raw_vec()
    }
    #[inline]
    fn shape(&self) -> &[usize] {
        self.shape().as_ref()
    }
    #[inline]
    fn zeros(shape: &[usize; 2]) -> Self {
        Self::zeros(shape.to_owned())
    }

    #[inline]
    fn row_mut(&mut self, idx: usize) -> &mut [F] {
        Array2::<F>::row_mut(self, idx).into_slice().unwrap()
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
    /// Number of samples of frequency-domain data
    pub fft_window_size: usize,
    /// Sample rate for the signals that will be used.
    pub sample_rate: u32,
    /// Use HTK formula for converting mel and hz
    pub htk: bool,
    /// Normalization
    pub norm: Option<NormalizationFactor>,
}

impl<F: Hz> Default for FilterBankParameters<F> {
    fn default() -> Self {
        Self {
            num_mel_bands: 128,
            freq_min: F::from(64).unwrap(),
            freq_max: F::from(8000).unwrap(),
            fft_window_size: 1024,
            sample_rate: 44100,
            htk: false,
            norm: Some(NormalizationFactor::One),
        }
    }
}

pub trait MelFilterBank<F: Hz, Mat: FilterBankMat<F>> {
    fn new(parameters: Option<FilterBankParameters<F>>) -> Self;
    fn weights(&self) -> &Mat;
}

#[derive(Debug)]
pub struct FilterBank<F, Mat>
where
    F: Mel + Hz,
{
    weights: Mat,
    fft_frequencies: Array1<F>,
    mel_frequencies: Array1<F>,
}

impl<F, Mat> MelFilterBank<F, Mat> for FilterBank<F, Mat>
where
    F: Hz + Mel + Zero + std::fmt::Debug,
    Mat: FilterBankMat<F>,
{
    fn new(parameters: Option<FilterBankParameters<F>>) -> Self {
        let default_params = FilterBankParameters::default();
        let params = parameters.unwrap_or(default_params);

        let num_fft_bands = 1 + params.fft_window_size / 2;

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
        fdiff.zip_mut_with(&mel_freqs.slice(s![1..]), |x, y| *x = *y - *x);
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
            match norm {
                NormalizationFactor::One => {
                    // Slaney-style mel is scaled to be approx constant energy per channel
                    // enorm = 2.0 / (mel_f[2:n_mels+2] - mel_f[:n_mels])
                    // weights *= enorm[:, np.newaxis]
                    let two = F::from(2.).unwrap();
                    let mut enorms = mel_freqs.slice(s![2..]).to_owned();
                    enorms.zip_mut_with(&mel_freqs.slice(s![..-2]), |x, y| {
                        *x = two / (*x - *y);
                    });

                    for (idx, enorm) in enorms.indexed_iter() {
                        let _ = weights
                            .row_mut(idx)
                            .iter_mut()
                            .map(|v| *v = *v * *enorm)
                            .collect::<()>();
                    }
                }
                _ => {}
            };
        };
        Self {
            weights,
            fft_frequencies: fft_freqs,
            mel_frequencies: mel_freqs,
        }
    }

    fn weights(&self) -> &Mat {
        &self.weights
    }
}

pub fn fft_frequencies<F: Hz>(sample_rate: u32, num_fft_bands: usize) -> Array1<F> {
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
