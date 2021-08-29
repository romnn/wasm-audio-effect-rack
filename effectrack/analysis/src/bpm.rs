use crate::Analyzer;
use anyhow::Result;
use common::filters::{bandpass_filter, convolve, cutoff_from_frequency};
use common::synth::quantize_samples;
use ndarray::prelude::*;
use ndarray::{ScalarOperand, Slice};
use num::{traits::FloatConst, Float, NumCast, One};
use std::error;
use std::fmt;

type Peak = i16;

#[derive(Debug)]
enum BpmDetectionError {
    NoMatches(),
}

impl fmt::Display for BpmDetectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
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
pub struct BpmDetectionAnalyzer {
    pub config: BpmDetectionAnalyzerConfig,
    pub chunk_size: usize,
    pub buffer_window_size: usize,
    buffer: Array1<f64>,
}

struct PeakIntervalGroup {
    pub tempo: f32,
    pub count: usize,
}

impl BpmDetectionAnalyzer {
    pub fn new(config: BpmDetectionAnalyzerConfig) -> Result<Self> {
        let sample_rate: f32 = NumCast::from(config.sample_rate).unwrap();
        // the larger the total window the more precise the bpm
        let chunk_size: usize = NumCast::from(sample_rate * 0.5).unwrap();
        let buffer_window_size: usize = NumCast::from(sample_rate * 10.0).unwrap();
        Ok(Self {
            config,
            chunk_size,
            buffer_window_size,
            buffer: Array1::zeros(0),
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

        for (idx, _) in peaks.iter().enumerate() {
            for i in (idx + 1)..(peaks.len().min(idx + 10)) {
                let minute: f32 = NumCast::from(60 * self.config.sample_rate).unwrap();
                let distance: f32 = NumCast::from(peaks[i].0 - peaks[idx].0).unwrap();
                let tempo = minute / distance;
                let mut group = PeakIntervalGroup { tempo, count: 1 };

                while group.tempo < 90.0 {
                    group.tempo *= 2.0;
                }

                while group.tempo > 180.0 {
                    group.tempo /= 2.0;
                }

                group.tempo = group.tempo.round();

                for other_group in groups.iter_mut() {
                    if (other_group.tempo - group.tempo).abs() < f32::EPSILON {
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
        let part_size = self.chunk_size;
        let parts: f32 = samples.len() as f32 / part_size as f32;
        let parts: usize = NumCast::from(parts).unwrap();
        let mut peaks: Vec<(usize, Peak)> = Vec::new();
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
        }
        peaks.sort_by(|a, b| b.1.cmp(&a.1));
        // println!("peaks sorted by volume: {:?}", peaks);
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
        let samples = samples.mapv(|v| {
            let v: f64 = NumCast::from(v.abs()).unwrap();
            v
        });
        // combine channels and choose the maximum
        let samples = samples.map_axis(Axis(1), |row| row.iter().fold(0f64, |acc, v| acc.max(*v)));

        // let start = Instant::now();
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
        )));
        // println!("filtered: {:?}", filtered_samples);

        let peaks = self.detect_peaks(filtered_samples);
        // println!("found {:?} peaks", peaks.len());
        // println!("found {:?} peaks: {:?}", peaks.len(), peaks);
        let mut intervals = self.find_intervals(peaks);
        // println!("found {:?} intervals", intervals.len());
        intervals.sort_by(|a, b| b.count.cmp(&a.count));
        // let duration = start.elapsed();
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
