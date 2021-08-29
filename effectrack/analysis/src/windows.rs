use ndarray::prelude::*;
use num::{traits::FloatConst, Float};
use std::ops::Mul;

#[derive(Debug)]
pub struct Window<T> {
    pub length: usize,
    pub window: Array1<T>,
}

impl<T> Window<T>
where
    T: Float,
{
    pub fn apply(&self, samples: &mut Array1<T>) {
        *samples = samples.to_owned().mul(&self.window);
    }
}

pub trait HannWindow<T> {
    fn hann(length: usize) -> Self;
}

impl<T> HannWindow<T> for Window<T>
where
    T: Float + FloatConst,
{
    fn hann(length: usize) -> Self {
        let mut window = Array::zeros(length);
        for (idx, value) in window.indexed_iter_mut() {
            let two_pi_i = T::from(2.0).unwrap() * T::PI() * T::from(idx).unwrap();
            let multiplier = (two_pi_i / T::from(length).unwrap()).cos();
            let multiplier = T::one() - multiplier;
            let multiplier = T::from(0.5).unwrap() * multiplier;
            *value = multiplier;
        }
        // let samples_len_f32 = samples.len() as f32;
        // for i in 0..samples.len() {
        //     let two_pi_i = 2.0 * PI * i as f32;
        //     let idontknowthename = cosf(two_pi_i / samples_len_f32);
        //     let multiplier = 0.5 * (1.0 - idontknowthename);
        //     windowed_samples.push(multiplier * samples[i])
        // }
        Self { length, window }
    }
}

pub trait HammingWindow<T> {
    fn hamming(length: usize) -> Self;
}

impl<T> HammingWindow<T> for Window<T>
where
    T: Float + FloatConst,
{
    fn hamming(length: usize) -> Self {
        let mut window = Array::zeros(length);
        for (idx, value) in window.indexed_iter_mut() {
            let two_pi_i = T::from(2.0).unwrap() * T::PI() * T::from(idx).unwrap();
            let multiplier = (two_pi_i / (T::from(length - 1).unwrap())).cos();
            let multiplier = T::from(0.46).unwrap() * multiplier;
            let multiplier = T::from(0.54).unwrap() - multiplier;
            *value = multiplier;
        }
        // let samples_len_f32 = samples.len() as f32;
        // for i in 0..samples.len() {
        //     let multiplier = 0.54 - (0.46 * (2.0 * PI * i as f32 / cosf(samples_len_f32 - 1.0)));
        //     windowed_samples.push(multiplier * samples[i])
        // }
        Self { length, window }
    }
}
