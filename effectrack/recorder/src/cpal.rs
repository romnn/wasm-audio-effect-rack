use crate::{Recorder, Sample};
use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BufferSize, Stream, StreamConfig};
use ndarray::prelude::*;
use ndarray::Array;
use ndarray::{concatenate, RemoveAxis, Slice};
use num::traits::{Float, FloatConst, Num, NumCast, ToPrimitive, Zero};
use ringbuf::RingBuffer;
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::{BufReader, Read, Seek};
use std::path::PathBuf;

#[derive()]
pub struct CpalRecorder {
    pub host: cpal::Host,
    pub input_device: cpal::Device,
    pub output_device: cpal::Device,
    // latency to buffer in case the input and output devices aren't synced
    pub latency: f32,
}

// let config = StreamConfig {
//     channels: 2,
//     buffer_size: BufferSize::Fixed(2048),
//     sample_rate: cpal::SampleRate(44_100),
// };
// config.buffer_size = BufferSize::Fixed(1024);

// this is for writing single to multi channel
// for frame in data.chunks_mut(nchannels.into()) {
//     let value = source.next().unwrap_or(S::zero());
//     for sample in frame.iter_mut() {
//         *sample = value;
//     }
// }

impl CpalRecorder {
    fn get_source_from_file(
        path: PathBuf,
    ) -> Result<(Box<dyn rodio::Source<Item = f32> + Send>, u32, u16)> {
        let file = BufReader::new(File::open(path)?);
        let source = Decoder::new_looped(file)?.convert_samples();
        let sample_rate = source.sample_rate();
        let nchannels = source.channels();
        Ok((Box::new(source), sample_rate, nchannels))
    }

    fn build_source_stream<S, T, F>(
        mut source: Box<dyn rodio::Source<Item = S> + Send>,
        output_device: &cpal::Device,
        config: &cpal::StreamConfig,
        callback: F,
    ) -> Result<(Stream, u32, u16)>
    where
        F: Fn(Result<Array2<T>>, u32, u16) -> () + Send + 'static,
        T: NumCast + Zero + Send + 'static,
        S: Sample + 'static,
    {
        let sample_rate = source.sample_rate();
        let nchannels = source.channels();
        let stream = output_device.build_output_stream(
            config,
            move |data, _| {
                data.iter_mut()
                    .for_each(|d| *d = source.next().unwrap_or(S::zero()));
                let num_samples = data.len();
                let (r, c) = (num_samples / (nchannels as usize), nchannels as usize);
                let samples = Array::from_iter(data.iter())
                    .mapv(|v| T::from(*v).unwrap())
                    .into_shape([r, c])
                    .map_err(|e| e.into());

                callback(samples, sample_rate, nchannels);
            },
            |err| eprintln!("an error occurred on stream: {}", err),
        )?;
        Ok((stream, sample_rate, nchannels))
    }

    fn build_input_stream<S, T, F>(
        input_device: &cpal::Device,
        config: &cpal::StreamConfig,
        mut callback: F,
    ) -> Result<(Stream, u32, u16)>
    where
        F: FnMut(Result<Array2<T>>, u32, u16) -> () + Send + 'static,
        T: NumCast + 'static,
        S: Sample,
    {
        let nchannels = config.channels;
        let sample_rate = config.sample_rate;
        let stream = input_device.build_input_stream(
            config,
            move |data: &[S], _| {
                let num_samples = data.len();
                let (r, c) = (num_samples / (nchannels as usize), nchannels as usize);
                let samples = Array::from_iter(data.iter())
                    .mapv(|v| T::from(*v).unwrap())
                    .into_shape([r, c])
                    .map_err(|e| e.into());

                callback(samples, sample_rate.0, nchannels);
            },
            |err| eprintln!("an error occurred on stream: {}", err),
        )?;
        Ok((stream, sample_rate.0, nchannels))
    }
}

impl Recorder for CpalRecorder {
    fn get_file_info(path: PathBuf) -> Result<(u32, u16)> {
        let file = BufReader::new(File::open(path)?);
        let source = Decoder::new_looped(file)?.convert_samples::<f32>();
        let (sample_rate, nchannels) = (source.sample_rate(), source.channels());
        Ok((sample_rate, nchannels))
    }

    fn stream_input<T, F>(&self, playback: bool, callback: F) -> Result<()>
    where
        F: Fn(Result<Array2<T>>, u32, u16) -> () + Send + 'static,
        T: NumCast + Zero + Send + Clone + 'static,
    {
        println!("Using input device: \"{}\"", self.input_device.name()?);
        println!("Using output device: \"{}\"", self.output_device.name()?);

        let input_config = self.input_device.default_input_config()?;

        // delay in case the input and output devices aren't synced
        let latency_frames = (self.latency / 1_000.0) * input_config.sample_rate().0 as f32;
        let latency_samples = latency_frames as usize * input_config.channels() as usize;

        // ring buffer to keep samples
        let ring = RingBuffer::<T>::new(latency_samples * 2);
        let (mut producer, mut consumer) = ring.split();

        for _ in 0..latency_samples {
            let _ = producer.push(T::zero());
        }

        let input_callback = move |data: Result<Array2<T>>, sample_rate, nchannels| {
            let mut output_fell_behind = false;
            if let Ok(ref data) = data {
                for sample in data {
                    if producer.push(sample.clone()).is_err() {
                        output_fell_behind = true;
                    }
                }
            }
            if output_fell_behind {
                eprintln!("output stream fell behind: try increasing latency");
            }
            callback(data, sample_rate, nchannels);
        };

        let output_callback = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let mut input_fell_behind = false;
            for sample in data {
                *sample = match consumer.pop() {
                    Some(s) => NumCast::from(s).unwrap(),
                    None => {
                        input_fell_behind = true;
                        0.0
                    }
                };
            }
            if input_fell_behind {
                eprintln!("input stream fell behind: try increasing latency");
            }
        };

        let output_config = self.output_device.default_output_config()?;
        let (input_stream, sample_rate, nchannels) = match input_config.sample_format() {
            cpal::SampleFormat::F32 => Self::build_input_stream::<f32, T, _>(
                &self.input_device,
                &input_config.clone().into(),
                input_callback,
            ),
            cpal::SampleFormat::I16 => Self::build_input_stream::<i16, T, _>(
                &self.input_device,
                &input_config.clone().into(),
                input_callback,
            ),
            cpal::SampleFormat::U16 => Self::build_input_stream::<u16, T, _>(
                &self.input_device,
                &input_config.clone().into(),
                input_callback,
            ),
        }?;

        let output_stream = self.output_device.build_output_stream(
            &input_config.into(),
            output_callback,
            |err| eprintln!("an error occurred on output stream: {}", err),
        )?;

        println!(
            "Starting the input and output streams with {} msec of latency",
            self.latency
        );
        input_stream.play()?;
        output_stream.play()?;

        // todo: await and block here until we receive a stop signal
        // one hour for now
        std::thread::sleep(std::time::Duration::from_secs(60 * 60));
        drop(input_stream);
        drop(output_stream);
        Ok(())
    }

    fn stream_file<T, F>(&self, path: PathBuf, callback: F) -> Result<()>
    where
        F: Fn(Result<Array2<T>>, u32, u16) -> () + Send + 'static,
        T: NumCast + Zero + Send + 'static,
    {
        let config = self.output_device.default_output_config()?;
        let (mut source, _, _) = Self::get_source_from_file(path)?;
        let (stream, _, _) = Self::build_source_stream::<f32, T, _>(
            source,
            &self.output_device,
            &config.into(),
            callback,
        )?;

        stream.play()?;
        // todo: await and block here until we receive a stop signal
        // one hour for now
        std::thread::sleep(std::time::Duration::from_secs(60 * 60));

        Ok(())
    }
}
