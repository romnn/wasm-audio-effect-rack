use crate::{
    AudioBackend, AudioBackendConfig, AudioInput, AudioInputCallback, AudioInputConfig,
    AudioOutput, AudioOutputCallback, AudioOutputConfig, AudioStreamDescriptor, AudioStreamKind,
    Sample,
};
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
use std::marker::PhantomData;
use std::path::PathBuf;
use std::thread;

// #[derive()]
// pub struct CpalAudioBackend<T> {
//     /// audio backend configuration
//     pub config: AudioBackendConfig,
//     /// audio host backend used
//     pub host: cpal::Host,
//     /// audio input device to stream from
//     pub input_device: Option<cpal::Device>,
//     /// output device that is used to stream audio files
//     pub output_device: Option<cpal::Device>,
//     phantom: PhantomData<T>,
//     // latency to buffer in case the input and output devices aren't synced
//     // pub latency: f32,
// }

#[derive()]
pub struct CpalAudioInput<T> {
    /// audio backend configuration
    pub config: AudioInputConfig,
    /// audio host backend used
    pub host: cpal::Host,
    /// audio input device to stream from
    // pub input_device: Option<cpal::Device>,
    pub input_device: cpal::Device,
    /// output device that is used to stream audio files
    // pub output_device: Option<cpal::Device>,
    pub output_device: cpal::Device,
    phantom: PhantomData<T>,
    // latency to buffer in case the input and output devices aren't synced
    // pub latency: f32,
}

#[derive()]
pub struct CpalAudioOutput<T> {
    /// audio backend configuration
    pub config: AudioOutputConfig,
    /// audio host backend used
    pub host: cpal::Host,
    /// audio input device to stream from
    pub input_device: cpal::Device,
    /// output device that is used to stream audio files
    pub output_device: cpal::Device,
    // pub input_device: Option<cpal::Device>,
    // pub output_device: Option<cpal::Device>,
    phantom: PhantomData<T>,
    // /// latency to buffer in case the input and output devices aren't synced
    // pub latency: f32,
}

pub trait CpalAudioBackend {
    fn query(&self) -> Result<()> {
        let available_hosts = cpal::available_hosts();
        println!("Available hosts:\n  {:?}", available_hosts);

        for host_id in available_hosts {
            println!("{}", host_id.name());
            let host = cpal::host_from_id(host_id)?;

            let default_in = host.default_input_device().map(|e| e.name().unwrap());
            let default_out = host.default_output_device().map(|e| e.name().unwrap());
            println!("  Default Input Device:\n    {:?}", default_in);
            println!("  Default Output Device:\n    {:?}", default_out);

            let devices = host.devices()?;
            println!("  Devices: ");
            for (device_index, device) in devices.enumerate() {
                println!("  {}. \"{}\"", device_index + 1, device.name()?);
            }
        }
        Ok(())
    }

    fn get_file_info(&self, path: PathBuf) -> Result<(u32, u16)> {
        let file = BufReader::new(File::open(path)?);
        let source = Decoder::new_looped(file)?.convert_samples::<f32>();
        let (sample_rate, nchannels) = (source.sample_rate(), source.channels());
        Ok((sample_rate, nchannels))
    }

    fn get_source_from_file(
        path: PathBuf,
    ) -> Result<(Box<dyn rodio::Source<Item = f32> + Send>, u32, u16)> {
        let file = BufReader::new(File::open(path)?);
        let source = Decoder::new_looped(file)?.convert_samples();
        let sample_rate = source.sample_rate();
        let nchannels = source.channels();
        Ok((Box::new(source), sample_rate, nchannels))
    }

    fn get_output_device(
        host: &cpal::Host,
        output_device_name: &Option<String>,
    ) -> Result<cpal::Device> {
        if let Some(device) = &output_device_name {
            host.output_devices()?
                .find(|x| x.name().map(|y| y == *device).unwrap_or(false))
        } else {
            host.default_output_device()
        }
        .ok_or(
            cpal::BackendSpecificError {
                description: format!(
                    "unable to find selected output device: \"{}\"",
                    output_device_name
                        .as_ref()
                        .unwrap_or(&"default".to_string())
                )
                .to_string(),
            }
            .into(),
        )
    }

    fn get_input_device(
        host: &cpal::Host,
        input_device_name: &Option<String>,
    ) -> Result<cpal::Device> {
        if let Some(device) = &input_device_name {
            host.input_devices()?
                .find(|x| x.name().map(|y| y == *device).unwrap_or(false))
        } else {
            host.default_input_device()
        }
        .ok_or(
            cpal::BackendSpecificError {
                description: format!(
                    "unable to find selected input device: \"{}\"",
                    input_device_name.as_ref().unwrap_or(&"default".to_string())
                )
                .to_string(),
            }
            .into(),
        )
    }

    fn get_host(
        config: &AudioBackendConfig,
        // ) -> Result<(Box<dyn rodio::Source<Item = f32> + Send>, u32, u16)> {
    ) -> Result<cpal::Host> {
        cfg_if::cfg_if! {
            if #[cfg(use_jack)] {
                let host = if config.jack {
                    cpal::host_from_id(cpal::available_hosts()
                        .into_iter()
                        .find(|id| *id == cpal::HostId::Jack)
                        .ok_or(cpal::BackendSpecificError{ description: "failed to find jack audio backend host".to_string() })?)?
                } else {
                    cpal::default_host()
                };
            } else {
                let host = cpal::default_host();
            }
        }
        Ok(host)
    }
}

impl<T> CpalAudioOutput<T>
where
    T: NumCast + Zero + Send + 'static,
{
    // todo: general functions
}

impl<T> AudioOutput<T> for CpalAudioOutput<T>
where
    // T: NumCast + Clone + Zero + Send + 'static,
    T: NumCast + Clone + Zero + Send + Sync + 'static,
    // T: NumCast + Clone + Zero + Send + Sync + 'static,
{
    fn new(config: AudioOutputConfig) -> Result<Self> {
        let host = Self::get_host(&config.backend_config)?;
        let input_device = Self::get_input_device(&host, &config.input_device)?;
        let output_device = Self::get_output_device(&host, &config.output_device)?;
        // let output_device = if let Some(device) = &config.output_device {
        //     host.output_devices()?
        //         .find(|x| x.name().map(|y| y == *device).unwrap_or(false))
        // } else {
        //     host.default_output_device()
        // }
        // .ok_or(cpal::BackendSpecificError {
        //     description: format!(
        //         "unable to find or use selected output device: \"{}\"",
        //         config
        //             .output_device
        //             .as_ref()
        //             .unwrap_or(&"default".to_string())
        //     )
        //     .to_string(),
        // })?;
        Ok(Self {
            config,
            host,
            input_device,
            output_device,
            phantom: PhantomData,
        })
    }

    fn descriptor(&self) -> Result<AudioStreamDescriptor> {
        Ok(AudioStreamDescriptor {
            kind: AudioStreamKind::OUTPUT,
            device: self.output_device.name()?,
            host: self.host.id().name().to_string(),
        })
    }

    // we really only ever want exactly one to run
    fn stream_to_output(
        &mut self,
        // input_config: Option<StreamConfig>,
        // input_config: AudioInputConfig,
        // playback: bool,
        mut callback: AudioOutputCallback<T>,
        // callback: &'static (dyn Fn(Result<Array2<T>>, u32, u16) -> () + Send + Sync),
        // callback: Callback<T>,
    ) -> Result<()> {
        // println!("Using input device: \"{}\"", self.input_device.name()?);
        // println!("Using output device: \"{}\"", self.output_device.name()?);
        let input_config = self.input_device.default_input_config()?.config();
        // let input_config = match input_config {
        //     Some(input_config) => input_config,
        //     None => self.input_device.default_input_config()?.into(),
        // };
        let output_config = self.output_device.default_output_config()?;

        // delay in case the input and output devices aren't synced
        let latency_frames = (self.config.latency / 1_000.0) * input_config.sample_rate.0 as f32;
        let latency_samples = latency_frames as usize * input_config.channels as usize;

        // ring buffer to keep samples
        let ring = RingBuffer::<T>::new(latency_samples * 2);
        let (mut producer, mut consumer) = ring.split();

        for _ in 0..latency_samples {
            let _ = producer.push(T::zero());
        }

        let builder = thread::Builder::new();
        let audio_receiver_thread = builder
            .name("audio output stream thread".to_string())
            .spawn(move || {
                println!(
                    "streaming audio to output \"{}\"",
                    "default" // recorder.input_name().unwrap_or("unknown".to_string())
                );
                // call the callback for new data and push to the producer
                let mut output_fell_behind = false;
                loop {
                    if let Ok(ref data) = callback() {
                        println!("got data");
                        for sample in data {
                            if producer.push(sample.clone()).is_err() {
                                output_fell_behind = true;
                            }
                        }
                    }
                    // println!("got data: {:?}", data);
                    // println!("got data: {}", data);
                }
                // if let Err(err) = input.stream_from_input(
                // config.monitor_input.unwrap_or(false),
                // callback,
                // Box::new(move |samples: Result<Array2<T>>, sample_rate, nchannels| {
                // todo: send to all subscribed analyzers
                // if let Err(err) = rec_tx.send((samples, sample_rate, nchannels)) {
                //     panic!("{}", err);
                // }
                // }),
                // ) {
                //     eprintln!("failed to stream input: {}", err);
                // }
                println!("input stream is over");
            })?;
        // self.threads.push(audio_receiver_thread);
        // let input_callback = Box::new(move |data: Result<Array2<T>>, sample_rate, nchannels| {
        //     let mut output_fell_behind = false;
        //     if let Ok(ref data) = data {
        //         for sample in data {
        //             if producer.push(sample.clone()).is_err() {
        //                 output_fell_behind = true;
        //             }
        //         }
        //     }
        //     // todo: actually do some real time optimization here next to printing a warning
        //     if output_fell_behind {
        //         eprintln!("output stream fell behind: try increasing latency");
        //     }
        //     callback(data, sample_rate, nchannels);
        // });

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

        // let (input_stream, sample_rate, nchannels) = match input_config.sample_format() {
        //     // cpal::SampleFormat::F32 => Self::build_input_stream::<f32, T>(
        //     cpal::SampleFormat::F32 => Self::build_input_stream::<f32>(
        //         &self.input_device,
        //         &input_config.clone().into(),
        //         // input_callback,
        //         callback,
        //     ),
        //     // cpal::SampleFormat::I16 => Self::build_input_stream::<i16, T>(
        //     cpal::SampleFormat::I16 => Self::build_input_stream::<i16>(
        //         &self.input_device,
        //         &input_config.clone().into(),
        //         // input_callback,
        //         callback,
        //     ),
        //     // cpal::SampleFormat::U16 => Self::build_input_stream::<u16, T>(
        //     cpal::SampleFormat::U16 => Self::build_input_stream::<u16>(
        //         &self.input_device,
        //         &input_config.clone().into(),
        //         // input_callback,
        //         callback,
        //     ),
        // }?;

        let output_stream = self.output_device.build_output_stream(
            &input_config.into(),
            output_callback,
            |err| eprintln!("an error occurred on output stream: {}", err),
        )?;

        // println!(
        //     "Starting the input and output streams with {} msec of latency",
        //     self.config.latency
        // );
        // input_stream.play()?;
        output_stream.play()?;

        // todo: await and block here until we receive a stop signal
        // one hour for now
        std::thread::sleep(std::time::Duration::from_secs(60 * 60));
        // drop(input_stream);
        drop(output_stream);
        Ok(())
    }
}

impl<T> CpalAudioInput<T>
where
    // T: NumCast + Clone + Zero + Send + 'static,
    T: NumCast + Zero + Send + 'static,
{
    // fn build_input_stream<S, T, F>(
    // fn build_input_stream<S, T>(
    fn build_input_stream<S>(
        // fn build_input_stream<S>(
        input_device: &cpal::Device,
        config: &cpal::StreamConfig,
        // mut callback: &dyn FnMut(Result<Array2<T>>, u32, u16) -> (),
        mut callback: AudioInputCallback<T>,
        // callback: Callback<T>,
        // callback: &'static (dyn FnMut(Result<Array2<T>>, u32, u16) -> () + Sync + Send + 'static),
    ) -> Result<(Stream, u32, u16)>
    where
        // F: FnMut(Result<Array2<T>>, u32, u16) -> () + Send + 'static,
        // T: NumCast + 'static,
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

impl<T> AudioInput<T> for CpalAudioInput<T>
where
    T: NumCast + Clone + Zero + Send + Sync + 'static,
{
    fn new(config: AudioInputConfig) -> Result<Self> {
        // cfg_if::cfg_if! {
        //     if #[cfg(use_jack)] {
        //         let host = if config.jack {
        //             cpal::host_from_id(cpal::available_hosts()
        //                 .into_iter()
        //                 .find(|id| *id == cpal::HostId::Jack)
        //                 .ok_or(cpal::BackendSpecificError{ description: "test".to_string() })?)?
        //         } else {
        //             cpal::default_host()
        //         };
        //     } else {
        //         let host = cpal::default_host();
        //     }
        // }
        let host = Self::get_host(&config.backend_config)?;
        let input_device = Self::get_input_device(&host, &config.input_device)?;
        let output_device = Self::get_output_device(&host, &config.output_device)?;
        // {
        // let input_device = if let Some(device) = &config.input_device {
        //     host.input_devices()?
        //         .find(|x| x.name().map(|y| y == *device).unwrap_or(false))
        // } else {
        //     host.default_input_device()
        // }
        // .ok_or(cpal::BackendSpecificError {
        //     description: format!(
        //         "unable to find or use selected input device: \"{}\"",
        //         config
        //             .input_device
        //             .as_ref()
        //             .unwrap_or(&"default".to_string())
        //     )
        //     .to_string(),
        // })?;
        let output_device = if let Some(device) = &config.output_device {
            host.output_devices()?
                .find(|x| x.name().map(|y| y == *device).unwrap_or(false))
        } else {
            host.default_output_device()
        }
        .ok_or(cpal::BackendSpecificError {
            description: format!(
                "unable to find or use selected output device: \"{}\"",
                config
                    .output_device
                    .as_ref()
                    .unwrap_or(&"default".to_string())
            )
            .to_string(),
        })?;
        Ok(Self {
            config,
            host,
            input_device,
            output_device,
            phantom: PhantomData,
        })
    }

    fn descriptor(&self) -> Result<AudioStreamDescriptor> {
        Ok(AudioStreamDescriptor {
            kind: AudioStreamKind::INPUT,
            device: self.input_device.name()?,
            host: self.host.id().name().to_string(),
        })
    }

    fn stream_from_input(
        &mut self,
        // playback: bool,
        mut callback: AudioInputCallback<T>,
        // callback: &'static (dyn Fn(Result<Array2<T>>, u32, u16) -> () + Sync + Send),
    ) -> Result<()>
// where
        // Self: Sized, //     F: Fn(Result<Array2<T>>, u32, u16) -> () + Send + 'static,
                     //     T: NumCast + Zero + Send + Clone + 'static
    {
        println!("Using input device: \"{}\"", self.input_device.name()?);
        // println!("Using output device: \"{}\"", self.output_device.name()?);

        let input_config = self.input_device.default_input_config()?;

        // delay in case the input and output devices aren't synced
        // let latency_frames = (self.config.latency / 1_000.0) * input_config.sample_rate().0 as f32;
        // let latency_samples = latency_frames as usize * input_config.channels() as usize;

        // // ring buffer to keep samples
        // let ring = RingBuffer::<T>::new(latency_samples * 2);
        // let (mut producer, mut consumer) = ring.split();

        // for _ in 0..latency_samples {
        //     let _ = producer.push(T::zero());
        // }

        // let input_callback = Box::new(move |data: Result<Array2<T>>, sample_rate, nchannels| {
        //     let mut output_fell_behind = false;
        //     if let Ok(ref data) = data {
        //         for sample in data {
        //             if producer.push(sample.clone()).is_err() {
        //                 output_fell_behind = true;
        //             }
        //         }
        //     }
        //     // todo: actually do some real time optimization here next to printing a warning
        //     if output_fell_behind {
        //         eprintln!("output stream fell behind: try increasing latency");
        //     }
        //     callback(data, sample_rate, nchannels);
        // });

        // let output_callback = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        //     let mut input_fell_behind = false;
        //     for sample in data {
        //         *sample = match consumer.pop() {
        //             Some(s) => NumCast::from(s).unwrap(),
        //             None => {
        //                 input_fell_behind = true;
        //                 0.0
        //             }
        //         };
        //     }
        //     if input_fell_behind {
        //         eprintln!("input stream fell behind: try increasing latency");
        //     }
        // };

        let (input_stream, sample_rate, nchannels) = match input_config.sample_format() {
            // cpal::SampleFormat::F32 => Self::build_input_stream::<f32, T>(
            cpal::SampleFormat::F32 => Self::build_input_stream::<f32>(
                &self.input_device,
                &input_config.clone().into(),
                // input_callback,
                callback,
            ),
            // cpal::SampleFormat::I16 => Self::build_input_stream::<i16, T>(
            cpal::SampleFormat::I16 => Self::build_input_stream::<i16>(
                &self.input_device,
                &input_config.clone().into(),
                // input_callback,
                callback,
            ),
            // cpal::SampleFormat::U16 => Self::build_input_stream::<u16, T>(
            cpal::SampleFormat::U16 => Self::build_input_stream::<u16>(
                &self.input_device,
                &input_config.clone().into(),
                // input_callback,
                callback,
            ),
        }?;

        // let output_config = self.output_device.default_output_config()?;
        // let output_stream = self.output_device.build_output_stream(
        //     &input_config.into(),
        //     output_callback,
        //     |err| eprintln!("an error occurred on output stream: {}", err),
        // )?;

        // println!(
        //     "Starting the input and output streams with {} msec of latency",
        //     self.config.latency
        // );
        input_stream.play()?;
        // output_stream.play()?;

        // todo: await and block here until we receive a stop signal
        // one hour for now
        std::thread::sleep(std::time::Duration::from_secs(60 * 60));
        drop(input_stream);
        // drop(output_stream);
        Ok(())
    }
}

// impl<T> CpalRecorder<T>
// where
//     T: NumCast + Zero + Send + 'static,
// {
//     // fn build_source_stream<S, T, F>(
//     // fn build_source_stream<S, T>(
//     fn build_source_stream<S>(
//         // fn build_source_stream<S>(
//         mut source: Box<dyn rodio::Source<Item = S> + Send>,
//         output_device: &cpal::Device,
//         config: &cpal::StreamConfig,
//         mut callback: Callback<T>,
//         // callback: &'static (dyn Fn(Result<Array2<T>>, u32, u16) -> () + Sync + Send),
//     ) -> Result<(Stream, u32, u16)>
//     where
//         // F: Fn(Result<Array2<T>>, u32, u16) -> () + Send + 'static,
//         // T: NumCast + Zero + Send + 'static,
//         S: Sample + 'static,
//     {
//         let sample_rate = source.sample_rate();
//         let nchannels = source.channels();
//         let stream = output_device.build_output_stream(
//             config,
//             move |data, _| {
//                 data.iter_mut()
//                     .for_each(|d| *d = source.next().unwrap_or(S::zero()));
//                 let num_samples = data.len();
//                 let (r, c) = (num_samples / (nchannels as usize), nchannels as usize);
//                 let samples = Array::from_iter(data.iter())
//                     .mapv(|v| T::from(*v).unwrap())
//                     .into_shape([r, c])
//                     .map_err(|e| e.into());

//                 callback(samples, sample_rate, nchannels);
//             },
//             |err| eprintln!("an error occurred on stream: {}", err),
//         )?;
//         Ok((stream, sample_rate, nchannels))
//     }

//     // fn build_input_stream<S, T, F>(
//     // fn build_input_stream<S, T>(
//     fn build_input_stream<S>(
//         // fn build_input_stream<S>(
//         input_device: &cpal::Device,
//         config: &cpal::StreamConfig,
//         // mut callback: &dyn FnMut(Result<Array2<T>>, u32, u16) -> (),
//         mut callback: Box<dyn FnMut(Result<Array2<T>>, u32, u16) -> () + Send + Sync + 'static>,
//         // callback: Callback<T>,
//         // callback: &'static (dyn FnMut(Result<Array2<T>>, u32, u16) -> () + Sync + Send + 'static),
//     ) -> Result<(Stream, u32, u16)>
//     where
//         // F: FnMut(Result<Array2<T>>, u32, u16) -> () + Send + 'static,
//         // T: NumCast + 'static,
//         S: Sample,
//     {
//         let nchannels = config.channels;
//         let sample_rate = config.sample_rate;
//         let stream = input_device.build_input_stream(
//             config,
//             move |data: &[S], _| {
//                 let num_samples = data.len();
//                 let (r, c) = (num_samples / (nchannels as usize), nchannels as usize);
//                 let samples = Array::from_iter(data.iter())
//                     .mapv(|v| T::from(*v).unwrap())
//                     .into_shape([r, c])
//                     .map_err(|e| e.into());

//                 callback(samples, sample_rate.0, nchannels);
//             },
//             |err| eprintln!("an error occurred on stream: {}", err),
//         )?;
//         Ok((stream, sample_rate.0, nchannels))
//     }
// }

// impl<T, F> Recorder<T, F> for CpalRecorder
// impl<T> AudioInputStream<T> for CpalRecorder<T>
// where
//     // F: Fn(Result<Array2<T>>, u32, u16) -> () + Send + 'static,
//     T: NumCast + Zero + Send + Clone + 'static,
// {
//     // fn new(config: RecorderConfig) -> Result<Self> {
//     //     cfg_if::cfg_if! {
//     //         if #[cfg(use_jack)] {
//     //             let host = if config.jack {
//     //                 cpal::host_from_id(cpal::available_hosts()
//     //                     .into_iter()
//     //                     .find(|id| *id == cpal::HostId::Jack)
//     //                     .ok_or(cpal::BackendSpecificError{ description: "test".to_string() })?)?
//     //             } else {
//     //                 cpal::default_host()
//     //             };
//     //         } else {
//     //             let host = cpal::default_host();
//     //         }
//     //     }
//     //     let input_device = if let Some(device) = &config.input_device {
//     //         host.input_devices()?
//     //             .find(|x| x.name().map(|y| y == *device).unwrap_or(false))
//     //     } else {
//     //         host.default_input_device()
//     //     }
//     //     .ok_or(cpal::BackendSpecificError {
//     //         description: format!(
//     //             "unable to find or use selected input device: \"{}\"",
//     //             config
//     //                 .input_device
//     //                 .as_ref()
//     //                 .unwrap_or(&"default".to_string())
//     //         )
//     //         .to_string(),
//     //     })?;
//     //     let output_device = if let Some(device) = &config.output_device {
//     //         host.output_devices()?
//     //             .find(|x| x.name().map(|y| y == *device).unwrap_or(false))
//     //     } else {
//     //         host.default_output_device()
//     //     }
//     //     .ok_or(cpal::BackendSpecificError {
//     //         description: format!(
//     //             "unable to find or use selected output device: \"{}\"",
//     //             config
//     //                 .output_device
//     //                 .as_ref()
//     //                 .unwrap_or(&"default".to_string())
//     //         )
//     //         .to_string(),
//     //     })?;
//     //     Ok(Self {
//     //         config,
//     //         host,
//     //         input_device,
//     //         output_device,
//     //         phantom: PhantomData,
//     //     })
//     // }

//     // fn input_name(&self) -> Result<String> {
//     //     self.input_device.name().map_err(|err| err.into())
//     // }

//     // fn output_name(&self) -> Result<String> {
//     //     self.output_device.name().map_err(|err| err.into())
//     // }

//     // fn descriptor(&self) -> Result<AudioStreamDescriptor> {
//     //     Ok(AudioStreamDescriptor {
//     //         kind: AudioStreamKind::INPUT,
//     //         device: self.input_device.name()?,
//     //         host: self.host.id().name().to_string(),
//     //     })
//     // }

//     fn stream_output(
//         &self,
//         // playback: bool,
//         // mut callback: Callback<T>,
//         // callback: &'static (dyn Fn(Result<Array2<T>>, u32, u16) -> () + Sync + Send),
//     ) -> Result<()> {
//         Ok(())
//     }

//     // fn stream_file<T, F>(&self, path: PathBuf, callback: F) -> Result<()>
//     fn stream_file(
//         &self,
//         path: PathBuf,
//         mut callback: Callback<T>,
//         // callback: &'static (dyn Fn(Result<Array2<T>>, u32, u16) -> () + Sync + Send),
//     ) -> Result<()>
//     where
//         Self: Sized, //     F: Fn(Result<Array2<T>>, u32, u16) -> () + Send + 'static,
//                      //     T: NumCast + Zero + Send + 'static
//     {
//         let config = self.output_device.default_output_config()?;
//         // let (mut source, _, _) = Self::get_source_from_file(path)?;
//         let (mut source, _, _) = get_source_from_file(path)?;
//         // let (stream, _, _) = Self::build_source_stream::<f32, T>(
//         let (stream, _, _) = Self::build_source_stream::<f32>(
//             source,
//             &self.output_device,
//             &config.into(),
//             callback,
//         )?;

//         stream.play()?;
//         // todo: await and block here until we receive a stop signal
//         // one hour for now
//         std::thread::sleep(std::time::Duration::from_secs(60 * 60));

//         Ok(())
//     }
// }

impl<T> CpalAudioBackend for CpalAudioInput<T> {}
impl<T> CpalAudioBackend for CpalAudioOutput<T> {}

// impl<T> AudioBackend for CpalRecorder<T>
// where
//     T: NumCast + Zero + Send + 'static,
// {
//     fn new(config: AudioBackendConfig) -> Result<Self> {
//         cfg_if::cfg_if! {
//             if #[cfg(use_jack)] {
//                 let host = if config.jack {
//                     cpal::host_from_id(cpal::available_hosts()
//                         .into_iter()
//                         .find(|id| *id == cpal::HostId::Jack)
//                         .ok_or(cpal::BackendSpecificError{ description: "failed to find jack audio backend host".to_string() })?)?
//                 } else {
//                     cpal::default_host()
//                 };
//             } else {
//                 let host = cpal::default_host();
//             }
//         }
//         // let input_device = if let Some(device) = &config.input_device {
//         //     host.input_devices()?
//         //         .find(|x| x.name().map(|y| y == *device).unwrap_or(false))
//         // } else {
//         //     host.default_input_device()
//         // }
//         // .ok_or(cpal::BackendSpecificError {
//         //     description: format!(
//         //         "unable to find or use selected input device: \"{}\"",
//         //         config
//         //             .input_device
//         //             .as_ref()
//         //             .unwrap_or(&"default".to_string())
//         //     )
//         //     .to_string(),
//         // })?;
//         // let output_device = if let Some(device) = &config.output_device {
//         //     host.output_devices()?
//         //         .find(|x| x.name().map(|y| y == *device).unwrap_or(false))
//         // } else {
//         //     host.default_output_device()
//         // }
//         // .ok_or(cpal::BackendSpecificError {
//         //     description: format!(
//         //         "unable to find or use selected output device: \"{}\"",
//         //         config
//         //             .output_device
//         //             .as_ref()
//         //             .unwrap_or(&"default".to_string())
//         //     )
//         //     .to_string(),
//         // })?;
//         Ok(Self {
//             config,
//             host,
//             input_device,
//             output_device,
//             phantom: PhantomData,
//         })
//     }
// }
//
//
//
//
//
// // Input configs
// if let Ok(conf) = device.default_input_config() {
//     println!("    Default input stream config:\n      {:?}", conf);
// }
// let input_configs = match device.supported_input_configs() {
//     Ok(f) => f.collect(),
//     Err(e) => {
//         println!("    Error getting supported input configs: {:?}", e);
//         Vec::new()
//     }
// };
// if !input_configs.is_empty() {
//     println!("    All supported input stream configs:");
//     for (config_index, config) in input_configs.into_iter().enumerate() {
//         println!(
//             "      {}.{}. {:?}",
//             device_index + 1,
//             config_index + 1,
//             config
//         );
//     }
// }

// // Output configs
// if let Ok(conf) = device.default_output_config() {
//     println!("    Default output stream config:\n      {:?}", conf);
// }
// let output_configs = match device.supported_output_configs() {
//     Ok(f) => f.collect(),
//     Err(e) => {
//         println!("    Error getting supported output configs: {:?}", e);
//         Vec::new()
//     }
// };
// if !output_configs.is_empty() {
//     println!("    All supported output stream configs:");
//     for (config_index, config) in output_configs.into_iter().enumerate() {
//         println!(
//             "      {}.{}. {:?}",
//             device_index + 1,
//             config_index + 1,
//             config
//         );
//     }
// }

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
