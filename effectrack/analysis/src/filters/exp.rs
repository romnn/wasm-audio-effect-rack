use anyhow::Result;
use ndarray::prelude::*;
use ndarray::{concatenate, Array, Zip, Ix, RemoveAxis, Slice};
use num::{traits::FloatConst, Float, NumCast};
use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ExpSmoothingFilterError {
    msg: String,
}

impl ExpSmoothingFilterError {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}

impl error::Error for ExpSmoothingFilterError {}

impl fmt::Display for ExpSmoothingFilterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "exp smoothing filter error: {}", self.msg)
    }
}

const MIN_ALPHA: f32 = 0.0;
const MAX_ALPHA: f32 = 1.0;

#[derive(Debug)]
pub struct ExpSmoothingFilter<T, A> {
    pub val: A,
    // do not allow setting them directly
    alpha_decay: T,
    alpha_rise: T,
}

impl<T, D> ExpSmoothingFilter<T, Array<T, D>>
where
    // T: Float + FloatConst + std::fmt::Debug + Sync + Send,
    T: Float + FloatConst + std::fmt::Debug + Send,
    D: Dimension,
{
    pub fn new(val: Array<T, D>, alpha_decay: T, alpha_rise: T) -> Result<Self> {
        let mut filter = Self {
            val: val,
            alpha_decay: T::from(0.5).unwrap(),
            alpha_rise: T::from(0.5).unwrap(),
        };
        filter.set_alpha_decay(alpha_decay)?;
        filter.set_alpha_rise(alpha_rise)?;
        Ok(filter)
    }

    pub fn is_valid_alpha(alpha: T) -> bool {
        T::from(MIN_ALPHA).unwrap() < alpha && alpha < T::from(MAX_ALPHA).unwrap()
    }

    // todo: or just clip the value?
    pub fn set_alpha_decay(&mut self, alpha_decay: T) -> Result<()> {
        // Small rise / decay factors = more smoothing
        if Self::is_valid_alpha(alpha_decay) {
            self.alpha_decay = alpha_decay;
            Ok(())
        } else {
            Err(ExpSmoothingFilterError::new("Invalid decay smoothing factor".to_string()).into())
        }
    }

    // todo: or just clip the value?
    pub fn set_alpha_rise(&mut self, alpha_rise: T) -> Result<()> {
        // Small rise / decay factors = more smoothing
        if Self::is_valid_alpha(alpha_rise) {
            self.alpha_rise = alpha_rise;
            Ok(())
        } else {
            Err(ExpSmoothingFilterError::new("Invalid rise smoothing factor".to_string()).into())
        }
    }
}

// impl<T, D> Default for ExpSmoothingFilter<T, D>
// where
//     T: Float + FloatConst + std::fmt::Debug + Sync + Send,
//     D: Dimension
// {
//     fn default() -> Self {
//         Self {
//             val: Array1::<T>::zeros(10),
//             alpha_decay: T::from(0.5).unwrap(),
//             alpha_rise: T::from(0.5).unwrap(),
//         }
//     }
// }

// impl<T> ExpSmoothingFilter<T, T>
// where
//     T: Float + FloatConst + std::fmt::Debug + Sync + Send,
// {
//     pub fn update(&mut self, new_val: T) -> T {
//         let alpha = if new_val > self.val {
//             self.alpha_rise
//         } else {
//             self.alpha_decay
//         };
//         self.val = alpha * new_val + (T::one() - alpha) * self.val;
//         self.val
//     }
// }

impl<T, D> ExpSmoothingFilter<T, Array<T, D>>
where
    // T: Float + FloatConst + std::fmt::Debug + Sync + Send,
    T: Float + FloatConst + std::fmt::Debug + Send,
    D: Dimension,
{
    pub fn update(&mut self, new_values: &Array<T,D>) -> &Array<T, D> {
        let alpha = &Zip::from(&self.val).and(new_values).map_collect(|a, val| {
            if *val - *a > T::zero() {
                self.alpha_rise
            } else {
                self.alpha_decay
            }
        });
        // let a = alpha.mapv(|a| a * new_val);
        let a = alpha * new_values;
        let b = alpha.mapv(|a| T::one() - a) * &self.val;
        self.val = a + b;
        &self.val
    }

    pub fn update_scalar(&mut self, new_val: T) -> &Array<T, D> {
        let alpha = self.val.mapv(|a| {
            if new_val - a > T::zero() {
                self.alpha_rise
            } else {
                self.alpha_decay
            }
        });
        let a = alpha.mapv(|a| a * new_val);
        let b = alpha.mapv(|a| T::one() - a) * &self.val;
        self.val = a + b;
        &self.val
    }
}
