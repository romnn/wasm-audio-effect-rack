use crate::Analyzer;
use anyhow::Result;
use common::filters::{bandpass_filter, convolve, cutoff_from_frequency};
use common::sorting::SortArray;
use common::synth::quantize_samples;
use ndarray::prelude::*;
use ndarray::{concatenate, RemoveAxis, ScalarOperand, Slice};
use num::{traits::FloatConst, Float, Num, NumCast, One};
use std::cmp;
use std::error;
use std::fmt;
use std::time::{Duration, Instant};
// use wasm_bindgen::prelude::*;

type Peak = i16;

#[derive(Debug)]
enum BpmDetectionError {
    InvalidParameter(String),
    NoMatches(),
}

impl fmt::Display for BpmDetectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidParameter(msg) => {
                write!(f, "invalid parameter: {}", msg)
            }
            Self::NoMatches() => {
                write!(f, "did not find sufficient matches")
            }
        }
    }
}

impl error::Error for BpmDetectionError {}

// todo: replace with proto
#[derive(Debug)]
pub struct BpmDetectionAnalyzerConfig {
    pub lowpass_cutoff_freq_hz: f64,
    pub highpass_cutoff_freq_hz: f64,
    pub band: f64,
    pub sample_rate: u32,
    pub nchannels: u16,
}

impl Default for BpmDetectionAnalyzerConfig {
    fn default() -> Self {
        Self {
            lowpass_cutoff_freq_hz: 150.0,
            highpass_cutoff_freq_hz: 100.0,
            band: 0.01,
            nchannels: 2,
            sample_rate: 44100,
        }
    }
}

#[derive(Debug)]
// pub struct BpmDetectionAnalyzer<T>
pub struct BpmDetectionAnalyzer
// where
//     T: Float + FloatConst,
{
    pub config: BpmDetectionAnalyzerConfig,
    pub chunk_size: usize,
    pub buffer_window_size: usize,
    // mel_gain_exp_filter: ExpSmoothingFilter<T, Array1<T>>,
    // mel_exp_filter: ExpSmoothingFilter<T, Array1<T>>,
    // gain_exp_filter: ExpSmoothingFilter<T, Array1<T>>,
    // gaussian_filter: GaussianFilter1d<T>,
    // hann_window: Window<T>,
    // padding: usize,
    // mel_filterbank: mel::FilterBank<T, Array2<T>>,
    // buffer: Array1<Peak>,
    buffer: Array1<f64>,
    // buffer: Vec<Peak>,
}

struct PeakIntervalGroup {
    pub tempo: f32,
    pub count: usize,
}

// impl BpmDetectionAnalyzer<T>
impl BpmDetectionAnalyzer
// impl<T> BpmDetectionAnalyzer<T>
// where
//     T: Float + FloatConst + One + Send + Sync + Default + std::fmt::Debug + ScalarOperand,
{
    pub fn new(config: BpmDetectionAnalyzerConfig) -> Result<Self> {
        let sample_rate: f32 = NumCast::from(config.sample_rate).unwrap();
        // the larger the total window the more precise the bpm
        let chunk_size: usize = NumCast::from(sample_rate * 0.5).unwrap();
        // let buffer_window_size: usize = NumCast::from(sample_rate * 10.0).unwrap();
        let buffer_window_size: usize = NumCast::from(sample_rate * 10.0).unwrap();
        // let buffer_window_size = 20 * chunk;
        // let buffer_window_size: usize = NumCast::from(5.0 * sample_rate).unwrap();
        // let buffer_window_size: usize = NumCast::from(0.5 * sample_rate).unwrap();
        Ok(Self {
            config,
            chunk_size,
            buffer_window_size,
            buffer: Array1::zeros(0),
            // buffer: Vec::new(),
        })
    }

    fn find_intervals(&self, peaks: Vec<(usize, Peak)>) -> Vec<PeakIntervalGroup> {
        // What we now do is get all of our peaks, and then measure the distance to
        // other peaks, to create intervals.  Then based on the distance between
        // those peaks (the distance of the intervals) we can calculate the BPM of
        // that particular interval.

        // The interval that is seen the most should have the BPM that corresponds
        // to the track itself.

        let mut groups: Vec<PeakIntervalGroup> = Vec::new();

        // peaks.forEach(function(peak, index) {
        // for (idx, (position, peak)) in peaks.enumerate() {
        // println!("peaks are: {:?}", peaks);
        for (idx, peak) in peaks.iter().enumerate() {
            // for (var i = 1; (index + i) < peaks.len() && i < 10; i++) {
            // println!("idx: {}", idx);
            for i in (idx + 1)..(peaks.len().min(idx + 10)) {
                // println!("i: {}", i);
                // (var i = 1; (index + i) < peaks.len() && i < 10; i++) {

                let minute: f32 = NumCast::from(60 * self.config.sample_rate).unwrap();
                // println!("peaks[i={}]: {:?}", i, peaks[i]);
                // println!("peaks[idx={}]: {:?}", idx, peaks[idx]);
                let distance: f32 = NumCast::from(peaks[i].0 - peaks[idx].0).unwrap();
                let tempo = minute / distance;
                let mut group = PeakIntervalGroup { tempo, count: 1 };
                // let group = {
                //   tempo: (60 * 44100) / (peaks[index + i].position - peak.position),
                //   count: 1
                // };

                while group.tempo < 90.0 {
                    group.tempo *= 2.0;
                }

                while group.tempo > 180.0 {
                    group.tempo /= 2.0;
                }

                group.tempo = group.tempo.round();

                for other_group in groups.iter_mut() {
                    if ((other_group.tempo - group.tempo).abs() < f32::EPSILON) {
                        other_group.count += 1;
                    }
                }

                if !groups.iter().any(|other_group: &PeakIntervalGroup| {
                    (other_group.tempo - group.tempo).abs() < f32::EPSILON
                }) {
                    groups.push(group);
                }
            }
        }
        groups
    }

    fn detect_peaks(&self, samples: Array1<Peak>) -> Vec<(usize, Peak)> {
        // let part_size: f32 = self.config.sample_rate as f32 / 2.0f32;
        let part_size = self.chunk_size;
        let parts: f32 = samples.len() as f32 / part_size as f32;
        // let parts: usize = parts as usize;
        let parts: usize = NumCast::from(parts).unwrap();
        // let parts: usize = NumCast::from(parts).unwrap();
        // let part_size: usize = part_size as usize;
        let mut peaks: Vec<(usize, Peak)> = Vec::new();

        // divide the audio samples into parts and identify the loudest sample in each part
        // each part is 0.5 seconds long - or 22,050 samples.
        // This will give us 60 'beats' - we will only take the loudest half of those.
        // This will allow us to ignore breaks, and allow us to address tracks with BPM <120.
        // println!(
        //     "parts: {}, part_size: {}, buffer_len: {}",
        //     parts,
        //     part_size,
        //     samples.len()
        // );
        for part in 0..parts {
            let chunk = samples.slice_axis(
                Axis(0),
                Slice::from(part * part_size..(part + 1) * part_size),
            );
            let mut max: Option<(usize, &Peak)> = None;
            for new in chunk.indexed_iter() {
                max = match max {
                    Some(old) => {
                        if old.1 > new.1 {
                            Some(old)
                        } else {
                            Some(new)
                        }
                    }
                    None => Some(new),
                };
            }
            if let Some((idx, peak)) = max {
                peaks.push((part * part_size + idx, *peak));
            }
            // let max = chunk.fold(0, |acc, v| acc.max(*v));
            // let max = chunk.indexed_iter().fold(0, |acc, v| {
            //     if v.1 > acc {
            //         v.0
            //     } else {
            //         acc
            //     acc.max(*v)
            // });
        }
        // form (idx, val) pairs
        // let indexed = Array::from_iter(peaks.indexed_iter());
        // let indexed = Array::from_iter(peaks).indexed_iter();
        // let indexed = Array::from(peaks).indexed_iter();
        // let indexed = peaks.iter().enumerate().collect::<(usize, &Peak)>();
        // let indexed: Vec<(usize, &Peak)> = peaks.iter().enumerate().collect();
        // let indexed = Array::from_iter(peaks).indexed_iter();
        // .map(|v| v.to_slice()));
        // console_log!("indexed: {:?}", indexed);
        // console_log!("indexed shape: {:?}", indexed.shape());
        // data_ids.sort_by_key(|&a| data[a as usize]);
        // let sorted = indexed.sort_axis_by(Axis(1), |a, b| indexed[[a, 1]] > indexed[[b, 1]]);

        // let peaks = Array1::from(peaks);

        // We then sort the peaks according to volume...
        // let mut peaks: Vec<(isize, i32)> = peaks
        //     .iter()
        //     .map(|(idx, val)| {
        //         let idx: isize = NumCast::from(*idx).unwrap();
        //         let val: i32 = NumCast::from(*val).unwrap();
        //         (idx, val)
        //     })
        //     .collect();
        // return peaks;
        peaks.sort_by(|a, b| {
            // let av: i64 = NumCast::from(a.1).unwrap();
            // let bv: i64 = NumCast::from(b.1).unwrap();
            b.1.cmp(&a.1)
        });
        // println!("peaks sorted by volume: {:?}", peaks);
        // let loudest = peaks.sort_axis_by(Axis(0), |a, b| peaks[a].1 > peaks[b].1);
        // ...take the loundest half of those...
        let center: f32 = NumCast::from(peaks.len()).unwrap();
        let center = center * 0.5;
        let center: usize = NumCast::from(center).unwrap();
        let loudest_peaks = &peaks[..center];
        let mut loudest_peaks = loudest_peaks.to_vec();

        // ...and re-sort it back based on position.
        loudest_peaks.sort_by(|a, b| a.0.cmp(&b.0));
        // println!("peaks sorted back by position: {:?}", loudest_peaks);
        loudest_peaks

        // peaks = peaks.slice_axis(Axis(0), Slice::from(...peaks.len() * 0.5));
        // loudest_half = Array1::from(loudest.slice_axis(Axis(0), Slice::from(...peaks.len() * 0.5));

        // console_log!("sorted: {:?}", sorted);
        // let test = samples.slice_axis(
        //     Axis(0),
        //     Slice::from(part * part_size..(part + 1) * part_size),
        // );
        // let test = test.sort_axis_by(Axis(0), |a, b| test[[a]] > test[[b]]);

        // let test = &samples[part * part_size..(part + 1) * part_size];
        // let test = peaks.push(0.0);
        // peaks

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
}

impl<T> Analyzer<Array2<T>> for BpmDetectionAnalyzer
where
    T: Float + FloatConst + One + Send + Sync + Default + std::fmt::Debug + ScalarOperand,
{
    fn window_size(&self) -> usize {
        self.buffer_window_size
    }

    fn descriptor(&self) -> proto::grpc::AudioAnalyzerDescriptor {
        proto::grpc::AudioAnalyzerDescriptor {
            name: "BpmDetectionAnalyzer".to_string(),
            input: None,
        }
    }

    fn analyze_samples(
        &mut self,
        samples: Array2<T>,
    ) -> Result<proto::audio::analysis::AudioAnalysisResult> {
        // todo: make this nicer with some chaining of processing
        // e.g. make mono, perform fft etc
        // println!("got {:?} samples", samples);
        let mut samples = samples.mapv(|v| {
            let v: f64 = NumCast::from(v.abs()).unwrap();
            v
        });
        // combine channels and choose the maximum
        let samples = samples.map_axis(Axis(1), |row| row.iter().fold(0f64, |acc, v| acc.max(*v)));
        // combine channels and choose the maximum
        // let mut samples: Array1<f64> =
        //     samples.map_axis(Axis(1), |row| row.iter().fold(0f64, |acc, v| acc.max(*v)));

        // if let Err(err) = self.buffer.append(Axis(0), samples.view()) {
        //     eprintln!("failed to extend buffer: {}", err);
        // }

        // limit the buffer size
        // let start_idx: i64 = NumCast::from(self.buffer_window_size).unwrap() * 20;
        // println!("got {} samples", samples.len());
        // let start_idx = self.buffer_window_size as i64 * 20;
        // let start_idx = self.buffer.len() as i64 - start_idx;
        // let start_idx: usize = NumCast::from(start_idx.max(0)).unwrap();
        // // let start_idx = (-(self.buffer_window_size as i64) * 20).max(0) as usize;
        // self.buffer = self
        //     .buffer
        //     .slice_axis(
        //         Axis(0),
        //         // Slice::from(((-self.buffer_window_size) * 20).max(0)..),
        //         Slice::from(start_idx..),
        //     )
        //     .to_owned();

        let start = Instant::now();
        // println!("input sample size: {}", samples.len());
        // compute the bpm from it
        let filter = bandpass_filter(
            cutoff_from_frequency(
                self.config.highpass_cutoff_freq_hz,
                NumCast::from(self.config.sample_rate).unwrap(),
            ),
            cutoff_from_frequency(
                self.config.lowpass_cutoff_freq_hz,
                NumCast::from(self.config.sample_rate).unwrap(),
            ),
            self.config.band,
        );
        let filtered_samples = Array::from_iter(quantize_samples::<Peak>(&convolve(
            &filter,
            samples.as_slice().unwrap(),
            // target_buffer.as_slice().unwrap(),
        )));
        // println!("filtered: {:?}", filtered_samples);

        let peaks = self.detect_peaks(filtered_samples);
        // println!("found {:?} peaks", peaks.len());
        // println!("found {:?} peaks: {:?}", peaks.len(), peaks);
        let mut intervals = self.find_intervals(peaks);
        // println!("found {:?} intervals", intervals.len());
        intervals.sort_by(|a, b| b.count.cmp(&a.count));
        let duration = start.elapsed();
        // println!("computing bpm took: {:?}", duration);
        if intervals.len() > 0 {
            let top_guess = &intervals[0];
            println!("guessed {} BPM", top_guess.tempo);
            let result = proto::audio::analysis::BpmDetectionAudioAnalysisResult {
                bpm: top_guess.tempo,
            };
            return Ok(proto::audio::analysis::AudioAnalysisResult {
                seq_num: 0,
                window_size: NumCast::from(self.buffer_window_size).unwrap(),
                result: Some(proto::audio::analysis::audio_analysis_result::Result::Bpm(
                    result,
                )),
            });
        }
        return Err(BpmDetectionError::NoMatches().into());
    }
}
