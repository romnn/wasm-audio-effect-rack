use crate::{Recorder, Sample};
use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::BufferSize;
use cpal::StreamConfig;
use ndarray::prelude::*;
use ndarray::Array;
use ndarray::{concatenate, RemoveAxis, Slice};
use num::traits::{Float, FloatConst, Num, NumCast, ToPrimitive, Zero};
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::{BufReader, Read, Seek};
use std::path::PathBuf;

#[derive()]
pub struct CpalRecorder {
    pub host: cpal::Host,
    pub device: cpal::Device,
}

impl CpalRecorder {
    fn get_source_from_file<T>(
        path: PathBuf,
    ) -> Result<(Box<dyn rodio::Source<Item = T> + Send>, u32, u16)>
    where
        T: rodio::Sample + Send + 'static,
    {
        let file = BufReader::new(File::open(path)?);
        let source = Decoder::new_looped(file)?.convert_samples();
        let (sample_rate, nchannels) = (source.sample_rate(), source.channels());
        Ok((Box::new(source), sample_rate, nchannels))
    }

    fn stream_file<T, F>(
        path: PathBuf,
        device: &cpal::Device,
        config: &cpal::StreamConfig,
        callback: F,
    ) -> Result<(u32, u16)>
    where
        F: Fn(Result<Array2<T>>, u32, u16) -> () + Send + 'static,
        T: Sample + 'static,
    {
        let (mut source, sample_rate, nchannels) = Self::get_source_from_file::<T>(path)?;

        let stream = device.build_output_stream(
            config,
            move |data, _| {
                data.iter_mut()
                    .for_each(|d| *d = source.next().unwrap_or(T::zero()));
                let num_samples = data.len();
                let (r, c) = (num_samples / (nchannels as usize), nchannels as usize);
                let samples = Array::from_iter(data.iter())
                    .mapv(|v| *v)
                    .into_shape([r, c])
                    .map_err(|e| e.into());

                callback(samples, sample_rate, nchannels);
            },
            |err| eprintln!("an error occurred on stream: {}", err),
        )?;
        stream.play()?;
        // todo: await and block here until we receive a stop signal
        // one hour for now
        std::thread::sleep(std::time::Duration::from_millis(60 * 60 * 1000 * 1000));
        Ok((sample_rate, nchannels))
    }
}

impl Recorder for CpalRecorder {
    fn stream_input(&self) -> Result<()> {
        Ok(())
    }

    fn get_file_info(path: PathBuf) -> Result<(u32, u16)> {
        let file = BufReader::new(File::open(path)?);
        let source = Decoder::new_looped(file)?.convert_samples::<f32>();
        let (sample_rate, nchannels) = (source.sample_rate(), source.channels());
        Ok((sample_rate, nchannels))
    }

    fn stream_file<T, F>(&self, path: PathBuf, callback: F) -> Result<(u32, u16)>
    where
        F: Fn(Result<Array2<T>>, u32, u16) -> () + Send + 'static,
        T: Sample + 'static,
    {
        let config = self.device.default_output_config()?;
        // let config = StreamConfig {
        //     channels: 2,
        //     buffer_size: BufferSize::Fixed(2048),
        //     sample_rate: cpal::SampleRate(44_100),
        // };
        // config.buffer_size = BufferSize::Fixed(1024);
        // println!("Default output config: {:?}", config);
        Self::stream_file::<T, _>(path, &self.device, &config.into(), callback)
    }
}
