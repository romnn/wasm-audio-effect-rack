use num::traits::{Bounded, FromPrimitive, Num, Zero};
use std::mem::size_of;

pub fn quantize<T>(input: f64) -> T
where
    T: Num + FromPrimitive + Bounded + Zero,
{
    let quantization_levels = 2.0_f64.powf(size_of::<T>() as f64 * 8.0) - 1.0;
    // defaults to 0 on quantization failure for whatever reason
    T::from_f64(input * (quantization_levels / 2.0)).unwrap_or_else(T::zero)
}

pub fn quantize_samples<T>(input: &[f64]) -> Vec<T>
where
    T: Num + FromPrimitive + Bounded + Zero,
{
    input.iter().map(|s| quantize::<T>(*s)).collect()
}
