use ndarray::prelude::*;
use num::{traits::FloatConst, Float, Num};

// type Item = Float + FloatConst + std::ops::MulAssign;

#[derive(Debug)]
pub struct Window<T> {
    pub length: usize,
    // pub window: &'a Array1<T>,
    pub window: Array1<T>,
}

// impl<T> Window<T> for Window<T>
impl<T> Window<T>
where
    T: Float + std::ops::MulAssign,
    // T: std::ops::MulAssign,
    // impl<'a, T> HannWindow<'a, T>
    // pub fn hann_window<T: Float + FloatConst + std::ops::MulAssign>(length: usize) -> Array1<T> {
    // where
    // T: Float + FloatConst + std::ops::MulAssign,
{
    // pub fn apply_hann_window<T: Float + FloatConst + std::ops::MulAssign>(samples: &mut Array1<T>) {
    pub fn apply(&self, samples: &mut Array1<T>) {
        // let cuck = &self.window;
        // *samples *= cuck;
        // *samples = *samples * &self.window
        *samples *= &self.window
        // *samples *= T::from(2).unwrap();
    }

    // pub fn build_and_apply(samples: &mut Array1<T>) {
    //     // let window = Self.&hann_window(samples.len());
    //     let window = Self::new(samples.len());
    //     // .&hann_window(samples.len());
    //     window.apply(samples);
    //     // *samples *= window;
    // }
}

pub trait HannWindow<T> {
    fn hann(length: usize) -> Self;
    // fn apply(&self, samples: &mut Array1<T>);
}

impl<T> HannWindow<T> for Window<T>
// impl<T> Window<T>
// impl<'a, T> HannWindow<'a, T>
// pub fn hann_window<T: Float + FloatConst + std::ops::MulAssign>(length: usize) -> Array1<T> {
where
    T: Float + FloatConst + std::ops::MulAssign,
{
    // pub fn hann_window(length: usize) -> Array1<T> {
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
    // fn apply(&self, samples: &mut Array1<T>);
}

impl<T> HammingWindow<T> for Window<T>
// impl<T> Window<T>
// impl<'a, T> HannWindow<'a, T>
// pub fn hann_window<T: Float + FloatConst + std::ops::MulAssign>(length: usize) -> Array1<T> {
where
    T: Float + FloatConst + std::ops::MulAssign,
{
    // pub fn hann_window(length: usize) -> Array1<T> {
    fn hamming(length: usize) -> Self {
        let mut window = Array::zeros(length);
        for (idx, value) in window.indexed_iter_mut() {
            let two_pi_i = T::from(2.0).unwrap() * T::PI() * T::from(idx).unwrap();
            let multiplier = (two_pi_i / (T::from(length - 1).unwrap())).cos();
            let multiplier = T::from(0.46).unwrap() * multiplier;
            let multiplier = T::from(0.54).unwrap() - multiplier;
            *value = multiplier;
        }

        Self { length, window }
    }
}

// pub fn hamming_window<T: Float + FloatConst + std::ops::MulAssign>(length: usize) -> Array1<T> {
//     let mut window = Array::zeros(length);
//     for (idx, value) in window.indexed_iter_mut() {
//         let two_pi_i = T::from(2.0).unwrap() * T::PI() * T::from(idx).unwrap();
//         let multiplier = (two_pi_i / (T::from(length - 1).unwrap())).cos();
//         let multiplier = T::from(0.46).unwrap() * multiplier;
//         let multiplier = T::from(0.54).unwrap() - multiplier;
//         *value = multiplier;
//     }
//     // let samples_len_f32 = samples.len() as f32;
//     // for i in 0..samples.len() {
//     //     let multiplier = 0.54 - (0.46 * (2.0 * PI * i as f32 / cosf(samples_len_f32 - 1.0)));
//     //     windowed_samples.push(multiplier * samples[i])
//     // }
//     window
// }

// pub fn apply_hamming_window<T: Float + FloatConst + std::ops::MulAssign>(samples: &mut Array1<T>) {
//     let window = &hamming_window(samples.len());
//     *samples *= window;
// }
