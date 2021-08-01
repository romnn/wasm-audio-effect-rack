extern crate common;

use crate::fft::FFT;
use crate::mel;
use crate::mel::{FilterBankMat, Hz, Mel, MelFilterBank};
use crate::windows::{HammingWindow, Window};
use crate::{AnalysisResult, Analyzer};
use anyhow::Result;
use common::sorting::DebugMinMax;
use ndarray::parallel::prelude::*;
use ndarray::prelude::*;
use ndarray::{concatenate, indices, Array, IntoDimension, Ix, NdIndex, RemoveAxis, Slice, Zip};
// use num::{pow::pow, traits::FloatConst, traits::NumOps, Float, NumCast};
use num::pow::pow;
use num::traits::{Float, FloatConst, NumCast, NumOps};

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
    fn temp<T>(
        samples: Array2<T>,
        sample_rate: u32,
        nchannels: u32,
        fps: u32,
    ) -> Result<AnalysisResult>
    where
        T: Float + FloatConst + Hz + Mel + Send + Sync + Default + std::fmt::Debug,
    {
        //std::ops::MulAssign
        // println!("initial samples: {:?}", samples);

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
        let window_size = samples.len();
        let hwindow = Window::hamming(window_size);
        hwindow.apply(&mut samples);
        println!(
            "after hamming: {:?}",
            samples.slice(s![..5]).view().insert_axis(Axis(1))
        );

        samples
            .append(Axis(0), Array::zeros(padding as usize).view())
            .unwrap();
        println!(
            "after padding: {:?}",
            samples.slice(s![..5]).view().insert_axis(Axis(1))
        );

        let ys = samples
            .fft()?
            .slice(s![..window_size / 2])
            .mapv(|v| T::from(v.norm()).unwrap());
        println!("fft ys: {:?}", ys.shape());
        println!(
            "fft ys: {:?}",
            ys.slice(s![..5]).view().insert_axis(Axis(1))
        );

        let a = ys.insert_axis(Axis(1));
        // let a = a.t();
        println!(
            "ys.T {:?}",
            // a.shape(),
            a.slice(s![..5, ..]) // .view().insert_axis(Axis(1))
        );

        let sample_rate_f: f32 = NumCast::from(sample_rate).unwrap();
        let fps_f: f32 = NumCast::from(fps).unwrap();
        let mel_samples: f32 = sample_rate_f * 1.0 / (2.0 * fps_f);
        let mel_samples: usize = NumCast::from(mel_samples).unwrap();
        println!("mel samples: {:?}", mel_samples);

        let mel_opts = mel::FilterBankParameters::<T> {
            num_mel_bands: 24,
            freq_min: T::from(200).unwrap(),
            freq_max: T::from(12000).unwrap(),
            fft_size: 1024,                   // we dont care about it
            num_fft_bands: Some(mel_samples), // (fft_size/2)+1
            sample_rate: 44100,
            htk: true,
            norm: None,
            ..mel::FilterBankParameters::default()
        };
        println!("mel options: {:?}", mel_opts);
        let mel = mel::FilterBank::<T, Array2<T>>::new(Some(mel_opts));
        // let mel_vec = mel.weights.clone();
        // println!("mel options: {:?}", mel_vec.shape());
        // let mel_vec = mel.weights.to_vec();
        // .to_vec();
        // let mel_min = mel_vec.iter().fold(T::infinity(), |a: T, &b| a.min(b));
        // let mel_max = mel_vec.iter().fold(T::neg_infinity(), |a: T, &b| a.max(b));
        let weights = mel.weights().t();
        println!(
            "mel_y.T {:?} {:?} \n (min: {:?}, max: {:?})",
            weights.shape(),
            weights.slice(s![..5, ..]), // .view().insert_axis(Axis(1))
            weights.to_owned().to_vec().debug_min(),
            weights.to_owned().to_vec().debug_max(),
        );
        let mel_out = a * weights;
        println!(
            "mel_y {:?} {:?} \n (min: {:?}, max: {:?})",
            mel_out.shape(),
            mel_out.slice(s![..5, ..]), // .view().insert_axis(Axis(1))
            mel_out.to_owned().to_vec().debug_min(),
            mel_out.to_owned().to_vec().debug_max(),
        );

        let mel_out = mel_out
            .fold_axis(Axis(0), T::zero(), |acc, e| *acc + *e)
            .mapv(|e| pow(e, 2));
        // .mapv(|e| e.pow(2)); // T::from(2).unwrap().pow(e))
        println!(
            "mel {:?} {:?} \n (min: {:?}, max: {:?})",
            mel_out.shape(),
            mel_out.slice(s![..5]), // .view().insert_axis(Axis(1))
            mel_out.to_owned().to_vec().debug_min(),
            mel_out.to_owned().to_vec().debug_max(),
        );

        let filter = GaussianFilter1d::default();
        let mel_out = filter.apply(mel_out, Axis(0), T::one());
        println!(
            "gaussian mel {:?} {:?} \n (min: {:?}, max: {:?})",
            mel_out.shape(),
            mel_out.slice(s![..5]), // .view().insert_axis(Axis(1))
            mel_out.to_owned().to_vec().debug_min(),
            mel_out.to_owned().to_vec().debug_max(),
        );

        Ok(AnalysisResult {
            // volume: volume.to_f64().unwrap(),
            volume: volume,
            ..AnalysisResult::default()
        })
    }
}

pub enum Symmetry {
    Symmetric,
    Antisymmetric,
}

#[derive(Default, Debug)]
pub struct GaussianFilter1d<T> {
    // sigma: Option<T>,
    truncate: Option<T>,
}

impl<T> GaussianFilter1d<T>
where
    T: Float + FloatConst + std::fmt::Debug + Sync + Send, //+ NumOps, // + std::ops::AddAssign,
{
    pub fn apply<D: Dimension + RemoveAxis>(
        &self,
        array: Array<T, D>,
        axis: Axis,
        sigma: T,
    ) -> Array<T, D> {
        // see: https://github.com/scipy/scipy/blob/v0.15.1/scipy/ndimage/filters.py#L181
        let truncate = self.truncate.unwrap_or(T::from(4.0).unwrap());
        let sd = sigma; // standard deviation
        let lw = truncate * sd + T::from(0.5).unwrap(); // the +0.5 is just ceil?
        let lw: usize = NumCast::from(lw).unwrap();
        let mut weights = Array1::<T>::zeros(2 * lw + 1);
        weights[lw] = T::one();
        let mut sum = T::one();
        let sd = pow(sd, 2);

        println!("lw: {:?}", lw);
        println!("sd: {:?}", sd);

        // weights = [0.0] * (2 * lw + 1)
        // weights[lw] = 1.0
        // sum = 1.0
        // sd = sd * sd

        // calculate the kernel:
        for ii in 1..lw + 1 {
            let tmp = T::from(-0.5).unwrap() * T::from(ii.pow(2)).unwrap();
            let tmp = (tmp / sd).exp();
            weights[lw + ii] = tmp;
            weights[lw - ii] = tmp;
            sum = sum + T::from(2.0).unwrap() * tmp;
        }
        // normalize
        for ii in 0..2 * lw + 1 {
            weights[ii] = weights[ii] / sum;
        }
        // todo: implement the derivations for the order of the filter
        // for ii in range(1, lw + 1):
        //     tmp = math.exp(-0.5 * float(ii * ii) / sd)
        //     weights[lw + ii] = tmp
        //     weights[lw - ii] = tmp
        //     sum += 2.0 * tmp
        // for ii in range(2 * lw + 1):
        //     weights[ii] /= sum

        // correlate1d(input, weights, axis, output, mode, cval, 0)
        Self::correlate1d(array, weights, axis, sigma)
    }

    fn correlate1d<D: Dimension + RemoveAxis>(
        array: Array<T, D>, //: IntoParallelIterator,
        w: Array1<T>,
        axis: Axis,
        sigma: T,
    ) -> Array<T, D> {
        // see: https://github.com/scipy/scipy/blob/701ffcc8a6f04509d115aac5e5681c538b5265a2/scipy/ndimage/src/ni_filters.c#L38

        // test for symmetry or anti-symmetry
        let filter_size = w.len();
        // size left and right are around a center, e.g.
        // -3 -2 -1 - 0 - 1 2 3
        let size1 = filter_size / 2; // e.g. 6 / 2 = 3
        let size2 = filter_size - size1 - 1; // e.g. 6 - 3 - 1 = 2
                                             // if (filter_size & 0x1) { // is 1 (true) if odd else =symmetric
                                             // if (filter_size & 0x1) { // is 0 (false) if odd else =symmetric
                                             // if (odd) {}
                                             // let mut output = array.clone();
                                             // let mut output = Array2::zeros([10, 10]);
                                             // let mut output = Array::zeros(array.shape());
        let mut symmetry: Option<Symmetry> = None;
        let side1_idx = Array1::from_iter(1..size1);
        if (size1 == size2) {
            // same as odd filter size
            // let test = Array1::range(1.0, size1 as f64, 1.0).par_iter().all(|a| true);
            // let test = Array1::from_iter(1..size1).par_iter().all(|a| true);
            if side1_idx
                .par_iter()
                .all(|i| w[size1 + i] - w[size1 - i] < T::epsilon())
            {
                symmetry = Some(Symmetry::Symmetric);
            } else if side1_idx
                .par_iter()
                .all(|i| w[size1 + i] + w[size1 - i] < T::epsilon())
            {
                symmetry = Some(Symmetry::Antisymmetric);
            };
        }

        match symmetry {
            Some(Symmetry::Symmetric) => {
                // array.axis_iter(axis).into_par_iter().for_each(|s| {
                // array.axis_iter(axis).for_each(|a| {
                // array.indexed_iter_mut().into_par_iter().for_each(|a| {
                // array.indexed_iter_mut().for_each(|(idx, mut a)| {
                // output.par_iter_mut().indexed_iter_mut().for_each(|a| {
                // output.par_iter_mut().for_each(|a| {
                // println!("{:?}", a);
                // println!("{:?}", a.shape());
                // });
            }
            _ => {}
        };
        // /* the correlation calculation: */
        // if (symmetric > 0) {
        //     for(ll = 0; ll < length; ll++) {
        //         oline[ll] = iline[0] * fw[0];
        //         for(jj = -size1 ; jj < 0; jj++)
        //             oline[ll] += (iline[jj] + iline[-jj]) * fw[jj];
        //         ++iline;
        //     }
        // }

        //
        // todo: use rayon
        // see https://docs.rs/ndarray/0.15.3/ndarray/parallel/index.html
        // todo: make option for symmetry enum (sym/asym)
        // skip the whole line buffer magic and just use iterators all the time
        // also in other places
        // use par iter everywhere!

        array
    }

    fn correlate<D>(array: Array<T, D>, weights: Array<T, D>) -> Array<T, D>
    where
        D: Dimension + RemoveAxis + Send + std::marker::Copy,
        D::Pattern: Send + NdIndex<D> + Copy,
    {
        // let centers: Vec<usize> = weights.shape().iter().map(|d| d / 2).collect();
        let centers = Array1::from_iter(weights.shape().iter().map(|d| d / 2));
        let weight_offsets = Zip::from(indices(weights.raw_dim())).map_collect(|w_idx| {
            // println!("{:?}", w_idx);
            // println!("{:?}", w_idx.into_dimension());
            // let mut w_idx = w_idx.into_dimension().as_array_view();
            // w_idx.into_dimension().as_array_view_mut().zip_mut_with(

            // for (w_idx_d, &center_idx_d) in w_idx.slice_mut().iter_mut().zip(&centers) {
            //     let new_idx =
            //     *w_idx_d -= center_idx
            //     // *sz = if *sz < ws { 0 } else { *sz - ws + 1 };
            // }
            // return Some(w_idx);
            let w_idx = w_idx.into_dimension().as_array_view().mapv(|c| c as isize);
            let centers = centers.mapv(|c| c as isize);
            return w_idx - centers;
            // Zip::from(w_idx.into_dimension().as_array_view())
            //     .and(&centers)
            //     .map_collect(
            //         // .for_each(
            //         // w_idx.into_dimension().as_array_view().zip_with(
            //         // &centers,
            //         |w_idx, center| {
            //             // this is a test
            //             // let
            //             // *a = *a + *a;
            //             // *w_idx = *w_idx - center;
            //             *w_idx as isize - *center as isize
            //         },
            //     )
            // println!("after: {:?}", after);
        });
        println!("weight offsets: {:?}", weight_offsets);

        // for win in array.windows(weights.raw_dim()) {
        //     println!("{:?}", win);
        // }
        // .for_each(|win| println!("{:?}", win));
        // Array1::from(p
        println!("centers of {:?} are {:?}", weights.raw_dim(), centers);
        let bounds = array.raw_dim().as_array_view().to_owned();
        let mut result = array.to_owned();

        // return result;
        Zip::indexed(&mut result)
            // .into_par_iter()
            // .for_each(|(center_idx, a)| {
            .for_each(|center_idx, a| {
                // let idx = idx.into_dimension().as_array_view().mapv(|c| c as isize);
                let center_idx = center_idx.into_dimension();
                // .as_array_view().mapv(|c| c as isize);
                // println!("{:?} {:?}", idx, a);
                // *a = *a + *a;
                // let idx = idx.clone();
                // let test_idx = idx.clone();
                // *a = Zip::indexed(&weights)
                // let tidx = idx.clone();
                *a = Zip::from(&weights)
                    .and(&weight_offsets)
                    // .and(std::iter::repeat(idx))
                    // .and(&idx)
                    // .map_collect(|w_idx, w, woff|)
                    // .fold(T::zero(), |acc, w_idx, w, w_off| {
                    .fold(T::zero(), |acc, w, w_off| {
                        // let mut a_idx = idx.as_array_view().mapv(|c| c as isize);
                        // a_idx += w_off;

                        let a_idx = Zip::from(&center_idx.as_array_view())
                            .and(w_off)
                            .map_collect(|i, w_off| *i as isize - w_off);
                        let out_of_bounds = !Zip::from(&a_idx)
                            .and(&bounds)
                            .all(|i, bound| 0 <= *i && *i < *bound as isize);
                        if !out_of_bounds {
                            // let a_idx: NdIndex<D> = a_idx.mapv(|c| c as usize);
                            let mut idx = center_idx.to_owned();
                            for (idx_d_idx, idx_d) in idx.slice_mut().iter_mut().enumerate() {
                                *idx_d = a_idx[idx_d_idx] as usize;
                            }

                            // if idx.into_pattern() == (0, 4) {
                            let debug_idx = Array1::from(vec![1, 1]);
                            if center_idx.as_array_view() == debug_idx {
                                println!(
                                    "{:?}: {:?} * {:?} ({:?})",
                                    center_idx, *w, idx, array[idx]
                                );
                            }
                            // println!("{:?}", idx);
                            // acc + *w * array[Dim(a_idx)]
                            // acc + *w * array[a_idx.into_dimensionality().unwrap()]
                            // acc + *w * array[a_idx.as_slice().unwrap()]
                            acc + *w * array[idx]
                            // acc + *w * array.index(a_idx)
                        } else {
                            acc
                        }
                    });
                // *a = Zip::from(indices(weights.shape())).map_collect(|w_idx| w_idx);
                // fold all the weights in parallel
                // w_idx =
                // .fold(T::zero(), |acc, w_idx| {
                //         println!("weight index: {:?}", w_idx);
                //         acc
                //     });
                // acc + (*w * array[idx])
                // acc + (*w * array[idx])
                // acc + (*w * array[idx])
                // });
                // do a map reduce over the zipped weight matrix with bounds check
            });
        result
    }
}

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
        // let a = vec![
        //     vec![0., 1., 2., 3., 4.],
        //     vec![5., 6., 7., 8., 9.],
        //     vec![10., 11., 12., 13., 14.],
        // ];
        let (rows, cols) = (3, 5);
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
        // w[[2, 0]] = 0.;
        // w[[0, 2]] = 0.;
        println!("{:#?}", a);
        println!("{:#?}", w);
        let b = GaussianFilter1d::correlate(a, w);
        println!("{:#?}", b);

        assert_eq!(true, false, "all good");
        return;
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
