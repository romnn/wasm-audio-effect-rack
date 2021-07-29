use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::BufferSize;
use cpal::StreamConfig;
use ndarray::prelude::*;
use ndarray::Array;
use ndarray::{concatenate, RemoveAxis, Slice};
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct RecorderOptions {
    #[cfg(use_jack)]
    pub jack: bool,
    pub device: Option<String>,
}

pub trait Recorder {
    fn new(options: RecorderOptions) -> Result<Self>
    where
        Self: Sized;
    fn stream_file(&self, path: PathBuf) -> Result<()>;
    fn query(&self) -> Result<()>;
}

#[derive()]
pub struct CpalRecorder {
    host: cpal::Host,
    device: cpal::Device,
}

impl CpalRecorder {
    // fn do_something_with_it<T>(data: &[T]) -> ()
    // // fn do_something_with_it<T>(data: &mut [T]) -> ()
    // where
    //     T: cpal::Sample + From<f32>,
    // {
    //     println!("{:?}", data.len());
    //     // data.iter_mut().for_each(|d| *d = 0.5f32.into());
    //     // data.iter().for_each(|d| 0.5f32.into());
    //     ()
    // }

    // fn stream_file<T, F>(
    fn stream_file<F>(
        path: PathBuf,
        device: &cpal::Device,
        config: &cpal::StreamConfig,
        handler: F,
    ) -> Result<()>
    where
        // T: cpal::Sample + std::fmt::Display,
        F: Fn(&[f32], u32, u16) -> () + Send + 'static,
    {
        let file = BufReader::new(File::open(path)?);
        let mut source = Decoder::new_looped(file)?.convert_samples();
        let sample_rate = source.sample_rate(); //  as f32;
        let channels = source.channels(); // as usize;
                                          // println!("sample_rate: {}", sample_rate);
        println!("channels: {}", channels);
        let stream = device.build_output_stream(
            config,
            move |data, _| {
                data.iter_mut()
                    .for_each(|d| *d = source.next().unwrap_or(0f32));
                handler(data, sample_rate, channels);
            },
            |err| eprintln!("an error occurred on stream: {}", err),
        )?;
        stream.play()?;

        std::thread::sleep(std::time::Duration::from_millis(1000 * 1000));

        Ok(())
    }
}

impl Recorder for CpalRecorder {
    fn new(options: RecorderOptions) -> Result<Self> {
        cfg_if::cfg_if! {
            if #[cfg(use_jack)] {
                let host = if options.jack {
                    cpal::host_from_id(cpal::available_hosts()
                        .into_iter()
                        .find(|id| *id == cpal::HostId::Jack)
                        .ok_or(cpal::BackendSpecificError{ description: "test".to_string() })?)?
                } else {
                    cpal::default_host()
                };
            } else {
                let host = cpal::default_host();
            }
        }
        let device = if let Some(device) = options.device {
            host.output_devices()?
                .find(|x| x.name().map(|y| y == device).unwrap_or(false))
        } else {
            host.default_output_device()
        }
        .ok_or(cpal::BackendSpecificError {
            description: "test".to_string(),
        })?;
        Ok(Self { host, device })
    }

    fn stream_file(&self, path: PathBuf) -> Result<()> {
        println!("Output device: {}", self.device.name()?);

        let config = self.device.default_output_config().unwrap();
        // let config = StreamConfig {
        //     channels: 2,
        //     buffer_size: BufferSize::Fixed(2048),
        //     sample_rate: cpal::SampleRate(44_100),
        // };
        // config.buffer_size = BufferSize::Fixed(1024);
        println!("Default output config: {:?}", config);
        Self::stream_file(
            path,
            &self.device,
            &config.into(),
            |data, sample_rate, nchannels| {
                let num_samples = data.len();
                let samples = Array::from_iter(data)
                    .into_shape([num_samples / (nchannels as usize), nchannels as usize])
                    .unwrap();

                let min = samples.iter().fold(f32::INFINITY, |a, &b| a.min(*b));
                let max = samples.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(*b));
                // todo: this should be done by the analyzer
                // make mono
                let samples = samples.mapv(|v| v.abs() as f32);

                // combine channels and choose the maximum
                let mono =
                    samples.map_axis(Axis(1), |row| row.iter().fold(0f32, |acc, v| acc.max(*v)));

                println!(
                    "size: {} mono size: {} (min: {}, max: {})",
                    samples.len(),
                    mono.len(),
                    min,
                    max
                );
            },
        );
        // match config.sample_format() {
        //     cpal::SampleFormat::F32 => Self::stream_file::<f32>(path, &self.device, &config.into(), stream_hander:),
        //     cpal::SampleFormat::I16 => Self::stream_file::<i16>(path, &self.device, &config.into()),
        //     cpal::SampleFormat::U16 => Self::stream_file::<u16>(path, &self.device, &config.into()),
        // };
        Ok(())
    }

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

                continue;
                // Input configs
                if let Ok(conf) = device.default_input_config() {
                    println!("    Default input stream config:\n      {:?}", conf);
                }
                let input_configs = match device.supported_input_configs() {
                    Ok(f) => f.collect(),
                    Err(e) => {
                        println!("    Error getting supported input configs: {:?}", e);
                        Vec::new()
                    }
                };
                if !input_configs.is_empty() {
                    println!("    All supported input stream configs:");
                    for (config_index, config) in input_configs.into_iter().enumerate() {
                        println!(
                            "      {}.{}. {:?}",
                            device_index + 1,
                            config_index + 1,
                            config
                        );
                    }
                }

                // Output configs
                if let Ok(conf) = device.default_output_config() {
                    println!("    Default output stream config:\n      {:?}", conf);
                }
                let output_configs = match device.supported_output_configs() {
                    Ok(f) => f.collect(),
                    Err(e) => {
                        println!("    Error getting supported output configs: {:?}", e);
                        Vec::new()
                    }
                };
                if !output_configs.is_empty() {
                    println!("    All supported output stream configs:");
                    for (config_index, config) in output_configs.into_iter().enumerate() {
                        println!(
                            "      {}.{}. {:?}",
                            device_index + 1,
                            config_index + 1,
                            config
                        );
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(feature = "portaudio")]
#[derive()]
pub struct PortaudioRecorder {}

#[cfg(feature = "portaudio")]
impl Recorder for PortaudioRecorder {
    fn new(options: RecorderOptions) -> Result<Self> {
        Ok(Self {})
    }
    fn stream_file(&self, path: PathBuf) -> Result<()> {
        Ok(())
    }
    fn query(&self) -> Result<()> {
        Ok(())
    }
}
//
// Set up the input device and stream with the default input config.
// let device = if opt.device == "default" {
//     host.default_input_device()
// } else {
//     host.input_devices()?
//         .find(|x| x.name().map(|y| y == opt.device).unwrap_or(false))
// }
// .expect("failed to find input device");

// println!("Input device: {}", device.name()?);
//

// use std::fs::File;
// use std::io::BufReader;
// use rodio::{Decoder, OutputStream, source::Source};

// // Get a output stream handle to the default physical sound device
// let (_stream, stream_handle) = OutputStream::try_default().unwrap();
// // Load a sound from a file, using a path relative to Cargo.toml
// let file = BufReader::new(File::open("examples/music.ogg").unwrap());
// // Decode that sound file into a source
// let source = Decoder::new(file).unwrap();
// // Play the sound directly on the device
// stream_handle.play_raw(source.convert_samples());

// // The sound plays in a separate audio thread,
// // so we need to keep the main thread alive while it's playing.
// std::thread::sleep(std::time::Duration::from_secs(5));
