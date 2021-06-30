mod utils;

use common::filters::{bandpass_filter, convolve, cutoff_from_frequency};
use common::sorting::SortArray;
use common::synth::quantize_samples;
use ndarray::prelude::*;
use ndarray::{RemoveAxis, Slice};
use std::cmp;
use wasm_bindgen::prelude::*;

// use num_traits::Float;
// use num_traits::{self, FromPrimitive, Zero};
// use std::ops::{Add, Div, Mul};

// use ndarray::imp_prelude::*;
// use ndarray::itertools::enumerate;
// use ndarray::numeric_util;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn console_log_one(msg: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (console_log_one(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub struct WasmBPMDetector {
    sample_rate: usize,
    lowpass_cutoff_freq_hz: f64,
    highpass_cutoff_freq_hz: f64,
    band: f64,
}

type Peak = i16;

/// # Numerical Methods for Arrays
// impl<A, S, D> ArrayBase<S, D>
// where
//     S: Data<Elem = A>,
//     D: Dimension,
// {
//     pub fn find_max_axis(&self, axis: Axis) -> Array<A, D::Smaller>
//     where
//         A: Clone + Zero + Add<Output = A>,
//         D: RemoveAxis,
//     {
//         let n = self.len_of(axis);
//         let mut res = Array::zeros(self.raw_dim().remove_axis(axis));
//         let stride = self.strides()[axis.index()];
//         if self.ndim() == 2 && stride == 1 {
//             // contiguous along the axis we are summing
//             let ax = axis.index();
//             for (i, elt) in enumerate(&mut res) {
//                 *elt = self.index_axis(Axis(1 - ax), i).max();
//             }
//         } else {
//             for i in 0..n {
//                 let view = self.index_axis(axis, i);
//                 res = res + &view;
//             }
//         }
//         res
//     }

//     pub fn max_axis(&self, axis: Axis) -> Option<Array<A, D::Smaller>>
//     where
//         A: Clone + Zero + FromPrimitive + Add<Output = A> + Div<Output = A>,
//         D: RemoveAxis,
//     {
//         let axis_length = self.len_of(axis);
//         if axis_length == 0 {
//             None
//         } else {
//             let axis_length =
//                 A::from_usize(axis_length).expect("Converting axis length to A must not fail");
//             Some(self.find_max_axis(axis));
//             // Some(sum / aview0(&axis_length))
//         }
//     }
// }

#[wasm_bindgen]
impl WasmBPMDetector {
    pub fn new(sample_rate: usize) -> WasmBPMDetector {
        utils::set_panic_hook();
        WasmBPMDetector {
            sample_rate,
            lowpass_cutoff_freq_hz: 150.0,
            highpass_cutoff_freq_hz: 100.0,
            band: 0.01,
        }
    }

    // fn detect_peaks(&self, samples: Array<Peak, Dim<[usize; 1]>>) -> Vec<Peak> {
    fn detect_peaks(&self, samples: Array<Peak, Dim<[usize; 1]>>) -> Vec<Peak> {
        let part_size = self.sample_rate / 2;
        let parts = samples.len() / part_size;
        let mut peaks: Vec<Peak> = Vec::new();

        // divide the audio samples into parts and identify the loudest sample in each part
        // each part is 0.5 seconds long - or 22,050 samples.
        // This will give us 60 'beats' - we will only take the loudest half of those.
        // This will allow us to ignore breaks, and allow us to address tracks with BPM <120.
        for part in 0..parts {
            let test = samples.slice_axis(
                Axis(0),
                Slice::from(part * part_size..(part + 1) * part_size),
            );
            // form (idx, val) pairs
            let indexed = Array::from_iter(test.indexed_iter()); // .map(|v| v.to_slice()));
                                                                 // console_log!("indexed: {:?}", indexed);
                                                                 // console_log!("indexed shape: {:?}", indexed.shape());
                                                                 // data_ids.sort_by_key(|&a| data[a as usize]);
                                                                 // let sorted = indexed.sort_axis_by(Axis(1), |a, b| indexed[[a, 1]] > indexed[[b, 1]]);
            let sorted = indexed.sort_axis_by(Axis(0), |a, b| indexed[a].1 > indexed[b].1);
            // console_log!("sorted: {:?}", sorted);
            // let test = samples.slice_axis(
            //     Axis(0),
            //     Slice::from(part * part_size..(part + 1) * part_size),
            // );
            // let test = test.sort_axis_by(Axis(0), |a, b| test[[a]] > test[[b]]);

            // let test = &samples[part * part_size..(part + 1) * part_size];
            let test = peaks.push(0);
        }
        peaks

        // var partSize = 22050,
        //   parts = data[0].length / partSize,
        //   peaks = [];

        // for (var i = 0; i < parts; i++) {
        // var max = 0;
        // for (var j = i * partSize; j < (i + 1) * partSize; j++) {
        //   var volume = Math.max(Math.abs(data[0][j]), Math.abs(data[1][j]));
        //   if (!max || (volume > max.volume)) {
        //     max = {
        //       position: j,
        //       volume: volume
        //     };
        //   }
        // }
        // peaks.push(max);
        // }

        // // We then sort the peaks according to volume...

        // peaks.sort(function(a, b) {
        // return b.volume - a.volume;
        // });

        // // ...take the loundest half of those...

        // peaks = peaks.splice(0, peaks.length * 0.5);

        // // ...and re-sort it back based on position.

        // peaks.sort(function(a, b) {
        // return a.position - b.position;
        // });

        // return peaks;
    }

    pub fn detect_bpm(&mut self, num_sample: i32, channels: i8, samples: Vec<f32>) -> f32 {
        // if audio_samples.len() < self.fft_size {
        //     panic!("Insufficient samples passed to detect_bpm(). Expected an array containing {} elements but got {}", self.fft_size, audio_samples.len());
        // }

        // console_log!("samples: {:?}", samples);
        let num_samples = samples.len();
        let samples = Array::from_iter(samples)
            .into_shape([num_samples / (channels as usize), channels as usize])
            .unwrap();

        // use absolute values for the amplitudes
        let samples = samples.mapv(|v| v.abs() as f64);

        // combine channels and choose the maximum
        let samples = samples.map_axis(Axis(1), |row| {
            row.iter().fold(0.0 as f64, |acc, v| acc.max(*v))
        });
        self.sample_rate = 128;
        // console_log!("sample rate: {:?}", self.sample_rate);

        // let samples = samples.max_axis(Axis(1)).unwrap();
        // let ax = Axis(1);
        // let mut res = Array::zeros(samples.raw_dim().remove_axis(ax));
        // for (i, elt) in (&mut res).into_iter().enumerate() {
        //     *elt = samples.index_axis(Axis(1 - ax.index()), i).as_slice_mut().unwrap().max();
        // }
        // let samples = samples.mmean_axis(Axis(1)).unwrap();
        // console_log!("samples: {:?}", samples);

        // self.fft_size = samples.len();
        // self.band = 0.01;
        // let samples: Vec<f64> = samples.iter().map(|&n| n as f64).collect();

        // Include only notes that exceed a power threshold which relates to the
        // amplitude of frequencies in the signal. Use the library's suggested
        // default value of 5.0.
        // const POWER_THRESHOLD: f32 = 5.0;

        // The clarity measure describes how coherent the sound of a note is. For
        // example, the background sound in a crowded room would typically be would
        // have low clarity and a ringing tuning fork would have high clarity.
        // This threshold is used to accept detect notes that are clear enough
        // (valid values are in the range 0-1).
        // const CLARITY_THRESHOLD: f32 = 0.6;

        // filter the signal with a low and high pass filter
        if (num_sample % (100 * 128) == 0) {
            let filter = bandpass_filter(
                cutoff_from_frequency(self.highpass_cutoff_freq_hz, self.sample_rate),
                cutoff_from_frequency(self.lowpass_cutoff_freq_hz, self.sample_rate),
                self.band,
            );
            let filtered_samples = Array::from_iter(quantize_samples::<Peak>(&convolve(
                &filter,
                samples.as_slice().unwrap(),
            )));
            // console_log!("filtered: {:?}", filtered_samples);
            let peaks = self.detect_peaks(filtered_samples);
        }

        // var peaks = getPeaks([buffer.getChannelData(0), buffer.getChannelData(1)]);
        // var groups = getIntervals(peaks);
        //
        // samples[0] as f32;
        // let optional_pitch = self.detector.get_pitch(
        //     &audio_samples,
        //     self.sample_rate,
        //     POWER_THRESHOLD,
        //     CLARITY_THRESHOLD,
        // );

        // match optional_pitch {
        //     Some(pitch) => pitch.frequency,
        //     None => 0.0,
        // }
        0.3
    }
}
