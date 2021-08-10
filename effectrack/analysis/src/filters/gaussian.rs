use anyhow::Result;
use common::sorting::DebugMinMax;
use ndarray::parallel::prelude::*;
use ndarray::prelude::*;
use ndarray::{
    concatenate, indices, Array, IntoDimension, Ix, NdIndex, RemoveAxis, ScalarOperand, Slice, Zip,
};
use num::traits::{Float, FloatConst, NumCast, NumOps, One};
use num::pow::pow;

pub enum Symmetry {
    Symmetric,
    Antisymmetric,
}

#[derive(Default, Debug)]
pub struct GaussianFilter1d<T> {
    truncate: Option<T>,
}

impl<T> GaussianFilter1d<T>
where
    T: Float + FloatConst + std::fmt::Debug + Sync + Send,
    // T: Float + FloatConst + std::fmt::Debug + Send,
{
    pub fn apply(&self, array: &Array1<T>, axis: Axis, sigma: T) -> Array1<T> {
        // see: https://github.com/scipy/scipy/blob/v0.15.1/scipy/ndimage/filters.py#L181
        let truncate = self.truncate.unwrap_or(T::from(4.0).unwrap());
        let sd = sigma; // standard deviation
        let lw = truncate * sd + T::from(0.5).unwrap(); // the +0.5 is just ceil?
        let lw: usize = NumCast::from(lw).unwrap();
        let mut weights = Array1::<T>::zeros(2 * lw + 1);
        let mut weights = Array1::<T>::zeros(2 * lw + 1);
        weights[lw] = T::one();
        let mut sum = T::one();
        let sd = pow(sd, 2);

        // println!("lw: {:?}", lw);
        // println!("sd: {:?}", sd);

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
        // println!("weights: {:?}", weights);
        // todo: implement the derivations for the order of the filter
        Self::correlate(array, weights)
    }

    // deprecated
    pub fn correlate1d<D: Dimension + RemoveAxis>(
        array: &Array<T, D>,
        w: Array1<T>,
        axis: Axis,
        sigma: T,
    ) -> &Array<T, D> {
        // see: https://github.com/scipy/scipy/blob/701ffcc8a6f04509d115aac5e5681c538b5265a2/scipy/ndimage/src/ni_filters.c#L38

        // test for symmetry or anti-symmetry
        let filter_size = w.len();
        // size left and right are around a center, e.g.
        // -3 -2 -1 - 0 - 1 2 3
        let size1 = filter_size / 2; // e.g. 6 / 2 = 3
        let size2 = filter_size - size1 - 1;
        let mut symmetry: Option<Symmetry> = None;
        let side1_idx = Array1::from_iter(1..size1);
        if (size1 == size2) {
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
        array
    }

    pub fn correlate<D>(array: &Array<T, D>, weights: Array<T, D>) -> Array<T, D>
    where
        D: Dimension + RemoveAxis + Send + std::marker::Copy,
        D::Pattern: Send + NdIndex<D> + Copy,
    {
        let centers = Array1::from_iter(weights.shape().iter().map(|d| d / 2));
        let weight_offsets = Zip::from(indices(weights.raw_dim())).map_collect(|w_idx| {
            let w_idx = w_idx.into_dimension().as_array_view().mapv(|c| c as isize);
            let centers = centers.mapv(|c| c as isize);
            return w_idx - centers;
        });
        // println!("weight offsets: {:?}", weight_offsets);
        // println!("centers of {:?} are {:?}", weights.raw_dim(), centers);

        let bounds = array.raw_dim().as_array_view().to_owned();
        let mut result = array.to_owned();

        Zip::indexed(&mut result)
            .into_par_iter()
            .for_each(|(center_idx, a)| {
                // .for_each(|center_idx, a| {
                let center_idx = center_idx.into_dimension();
                *a = Zip::from(&weights)
                    .and(&weight_offsets)
                    .fold(T::zero(), |acc, w, w_off| {
                        let a_idx = Zip::from(&center_idx.as_array_view())
                            .and(w_off)
                            .map_collect(|i, w_off| *i as isize - w_off);
                        let out_of_bounds = !Zip::from(&a_idx)
                            .and(&bounds)
                            .all(|i, bound| 0 <= *i && *i < *bound as isize);
                        if !out_of_bounds {
                            let mut idx = center_idx.to_owned();
                            for (idx_d_idx, idx_d) in idx.slice_mut().iter_mut().enumerate() {
                                *idx_d = a_idx[idx_d_idx] as usize;
                            }
                            // let debug_idx = Array1::from(vec![1, 1]);
                            // if center_idx.as_array_view() == debug_idx {
                            //     println!(
                            //         "{:?}: {:?} * {:?} ({:?})",
                            //         center_idx, *w, idx, array[idx]
                            //     );
                            // }
                            acc + *w * array[idx]
                        } else {
                            acc
                        }
                    });
            });
        result
    }
}
