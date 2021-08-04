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
    // fn get_source_from_file<R>(path: PathBuf) -> Result<(rodio::Decoder<R>, u32, u16)>
    // fn get_source_from_file<S>(path: PathBuf) -> Result<(S, u32, u16)>
    fn get_source_from_file<T>(
        path: PathBuf,
    ) -> Result<(
        // Box<dyn rodio::Source<Item: rodio::Sample>>,
        // Box<dyn rodio::Source<Item: S>>,
        Box<dyn rodio::Source<Item = T> + Send>,
        // rodio::source::SamplesConverter<LoopedDecoder<BufReader<File>>, D>,
        u32,
        u16,
    )>
    where
        // I: rodio::Source,
        // I::Item: rodio::Sample,
        // D: rodio::Sample,
        // T: rodio::Sample + Sync + Send + 'static,
        T: rodio::Sample + Send + 'static,
        // where
        // S: rodio::Source,
        // <S as Iterator>::Item: rodio::Sample,
        // R: Read + Seek + Send,
        //rodio::Source<Item: rodio::Sample>
    {
        let file = BufReader::new(File::open(path)?);
        let source = Decoder::new_looped(file)?.convert_samples();
        let (sample_rate, nchannels) = (source.sample_rate(), source.channels());
        Ok((Box::new(source), sample_rate, nchannels))
    }

    // fn stream_file<T, S, F>(
    fn stream_file<T, F>(
        path: PathBuf,
        device: &cpal::Device,
        config: &cpal::StreamConfig,
        callback: F,
    ) -> Result<(u32, u16)>
    where
        // F: Fn(&[f32], u32, u16) -> () + Send + 'static,
        // F: Fn(&[T], u32, u16) -> () + Send + 'static,
        // F: Fn(Result<Array2<T>>, u32, u16) -> () + Send + 'static, // Sync + 'static,
        F: Fn(Result<Array2<T>>, u32, u16) -> () + Send + 'static,
        // S: ToPrimitive + Zero + rodio::Sample + Send + Sync + 'static,
        // T: rodio::Sample + Float + Zero + Send + Sync + 'static,
        // T: Sample + Sync + 'static,
        T: Sample + 'static,
    {
        // let file = BufReader::new(File::open(path)?);
        // let mut source = Decoder::new_looped(file)?.convert_samples();
        // let sample_rate = source.sample_rate();
        // let nchannels = source.channels();
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
                    // .mapv(|v| T::from(*v).unwrap())
                    .into_shape([r, c])
                    .map_err(|e| e.into());

                callback(samples, sample_rate, nchannels);
            },
            |err| eprintln!("an error occurred on stream: {}", err),
        )?;
        stream.play()?;
        // todo: await and block here until we receive a stop signal
        std::thread::sleep(std::time::Duration::from_millis(1000 * 1000));
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

    // fn stream_callback<T>(&self, data: &[T], sample_rate: u32, nchannels: u16) {
    //     let num_samples = data.len();
    //     let samples = Array::from_iter(data)
    //         .into_shape([num_samples / (nchannels as usize), nchannels as usize])
    //         .unwrap();
    // }

    // fn stream_file<F, T>(&self, path: PathBuf, callback: F) -> Result<()>
    fn stream_file<T, F>(&self, path: PathBuf, callback: F) -> Result<(u32, u16)>
    where
        // F: Fn(Result<Array2<T>>, u32, u16) -> () + Send + 'static,
        F: Fn(Result<Array2<T>>, u32, u16) -> () + Send + 'static,
        // S: Zero + rodio::Sample,
        // T: rodio::Sample + Float + Zero + Send + Sync + 'static,
        T: Sample + 'static, // Sync + 'static,
                             // where
                             //     F: Fn(Array2<T>, u32, u16) -> (), // + Send + 'static,
                             //     T: Float,
    {
        // println!("Output device: {}", self.device.name()?);

        let config = self.device.default_output_config().unwrap();
        // let config = StreamConfig {
        //     channels: 2,
        //     buffer_size: BufferSize::Fixed(2048),
        //     sample_rate: cpal::SampleRate(44_100),
        // };
        // config.buffer_size = BufferSize::Fixed(1024);
        // println!("Default output config: {:?}", config);
        // let callback = |data, sample_rate, nchannels| {
        //     let num_samples = data.len();
        //     let samples = Array::from_iter(data)
        //         .into_shape([num_samples / (nchannels as usize), nchannels as usize])
        //         .unwrap();

        //     let min = samples.iter().fold(f32::INFINITY, |a, &b| a.min(*b));
        //     let max = samples.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(*b));
        //     // todo: this should be done by the analyzer
        //     // make mono
        //     let samples = samples.mapv(|v| v.abs() as f32);

        //     // combine channels and choose the maximum
        //     let mono = samples.map_axis(Axis(1), |row| row.iter().fold(0f32, |acc, v| acc.max(*v)));

        //     println!(
        //         "size: {} mono size: {} (min: {}, max: {})",
        //         samples.len(),
        //         mono.len(),
        //         min,
        //         max
        //     );
        // };

        // Self::stream_file(
        //     path, &self.device, &config.into(),
        //     |data: &[T], sample_rate: u32, nchannels: u16| {
        //     }
        //     unknown field
        //   &self.stream_callback::<f32>);

        Self::stream_file::<T, _>(path, &self.device, &config.into(), callback)
        // match config.sample_format() {
        //     cpal::SampleFormat::F32 => {
        //         Self::stream_file::<T, f32, _>(path, &self.device, &config.into(), callback)
        //     }
        //     cpal::SampleFormat::I16 => {
        //         Self::stream_file::<T, i16, _>(path, &self.device, &config.into(), callback)
        //     }
        //     cpal::SampleFormat::U16 => {
        //         Self::stream_file::<T, _>(path, &self.device, &config.into(), callback)
        //     }
        // }
    }
}
