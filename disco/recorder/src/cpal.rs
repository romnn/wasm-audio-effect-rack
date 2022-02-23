use crate::{
    AudioBackendConfig, AudioInput, AudioInputCallback, AudioInputConfig, AudioOutput,
    AudioOutputCallback, AudioOutputConfig, AudioStreamInfo, Sample,
};
use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Stream;
use ndarray::Array;
use num::traits::NumCast;
use rodio::{source::Source, Decoder};
use std::fs::File;
use std::io::BufReader;
use std::marker::PhantomData;
use std::path::PathBuf;

impl From<cpal::SupportedStreamConfig> for AudioStreamInfo {
    fn from(config: cpal::SupportedStreamConfig) -> AudioStreamInfo {
        AudioStreamInfo {
            sample_rate: config.sample_rate().0,
            nchannels: config.channels(),
        }
    }
}

#[derive()]
pub struct CpalAudioInput<T> {
    /// audio backend configuration
    pub config: AudioInputConfig,
    /// audio host backend used
    pub host: cpal::Host,
    /// audio input device to stream from
    // pub input_device: Option<cpal::Device>,
    pub input_device: cpal::Device,
    /// input configuration
    pub input_config: cpal::SupportedStreamConfig,
    /// output device that is used to stream audio files
    // pub output_device: Option<cpal::Device>,
    pub output_device: cpal::Device,
    /// output configuration
    pub output_config: cpal::SupportedStreamConfig,
    phantom: PhantomData<T>,
}

#[derive()]
pub struct CpalAudioFile<T> {
    /// audio backend configuration
    // pub config: AudioInputConfig,
    /// audio host backend used
    pub host: cpal::Host,
    pub path: PathBuf,
    pub looped: bool,
    /// audio input device to stream from
    // pub input_device: Option<cpal::Device>,
    // pub input_device: cpal::Device,
    /// input configuration
    // pub input_config: cpal::SupportedStreamConfig,
    /// output device that is used to stream audio files
    // pub output_device: Option<cpal::Device>,
    pub output_device: cpal::Device,
    /// output configuration
    pub output_config: cpal::SupportedStreamConfig,
    phantom: PhantomData<T>,
}

#[derive()]
pub struct CpalAudioOutput<T> {
    /// audio backend configuration
    pub config: AudioOutputConfig,
    /// audio host backend used
    pub host: cpal::Host,
    /// audio input device to stream from
    pub input_device: cpal::Device,
    /// input configuration
    pub input_config: cpal::SupportedStreamConfig,
    /// output device that is used to stream audio files
    pub output_device: cpal::Device,
    /// output configuration
    pub output_config: cpal::SupportedStreamConfig,
    phantom: PhantomData<T>,
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
        path: &PathBuf,
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

    fn get_host(_config: &AudioBackendConfig) -> Result<cpal::Host> {
        cfg_if::cfg_if! {
            if #[cfg(use_jack)] {
                let host = if _config.jack {
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

impl<T> AudioOutput<T> for CpalAudioOutput<T>
where
    T: Sample,
{
    fn new(config: AudioOutputConfig) -> Result<Self> {
        let host = Self::get_host(&config.backend_config)?;
        let input_device = Self::get_input_device(&host, &config.input_device)?;
        let output_device = Self::get_output_device(&host, &config.output_device)?;
        let input_config = input_device.default_input_config()?;
        let output_config = output_device.default_output_config()?;
        Ok(Self {
            config,
            host,
            input_device,
            input_config,
            // input_descriptor,
            output_device,
            output_config,
            phantom: PhantomData,
        })
    }

    fn descriptor(&self) -> Result<proto::grpc::AudioOutputDescriptor> {
        Ok(proto::grpc::AudioOutputDescriptor {
            backend: "cpal".to_string(),
            device: self.output_device.name()?,
            host: self.host.id().name().to_string(),
            input: None,
        })
    }

    fn input_stream_params(&self) -> AudioStreamInfo {
        AudioStreamInfo::from(self.input_config.clone())
    }

    fn output_stream_params(&self) -> AudioStreamInfo {
        AudioStreamInfo::from(self.output_config.clone())
    }

    fn stream_to_output(&mut self, mut callback: AudioOutputCallback<T>) -> Result<()> {
        let output_callback = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let mut input_fell_behind = false;
            for sample in data {
                *sample = match callback() {
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
        let output_stream = self.output_device.build_output_stream(
            &self.input_config.clone().into(),
            output_callback,
            |err| eprintln!("an error occurred on output stream: {}", err),
        )?;

        output_stream.play()?;

        // todo: await and block here until we receive a stop signal
        // 12 hours for now
        std::thread::sleep(std::time::Duration::from_secs(12 * 60 * 60));
        drop(output_stream);
        Ok(())
    }
}

impl<T> CpalAudioInput<T>
where
    T: Sample,
{
    // fn from_input(config: AudioInputConfig) -> Result<&'static dyn AudioInput<T>> {
    // pub fn new(config: AudioInputConfig) -> Result<Box<dyn AudioInput<T>>> {
    pub fn new(config: AudioInputConfig) -> Result<Self> {
        let host = Self::get_host(&config.backend_config)?;
        let input_device = Self::get_input_device(&host, &config.input_device)?;
        let input_config = input_device.default_input_config()?;
        let output_device = Self::get_output_device(&host, &config.output_device)?;
        let output_config = output_device.default_output_config()?;
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
        // Ok(Box::new(CpalAudioInput {
        Ok(CpalAudioInput {
            config,
            host,
            input_device,
            input_config,
            output_device,
            output_config,
            phantom: PhantomData,
            // }))
        })
    }

    fn build_input_stream<S>(
        input_device: &cpal::Device,
        config: &cpal::StreamConfig,
        mut callback: AudioInputCallback<T>,
    ) -> Result<(Stream, u32, u16)>
    where
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
                    .mapv(|v| NumCast::from(*v).unwrap())
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
    T: Sample,
{
    fn descriptor(&self) -> Result<proto::grpc::AudioInputDescriptor> {
        Ok(proto::grpc::AudioInputDescriptor {
            backend: "cpal".to_string(),
            device: self.input_device.name()?,
            host: self.host.id().name().to_string(),
            file: "".to_string(),
        })
    }

    fn input_stream_params(&self) -> AudioStreamInfo {
        AudioStreamInfo::from(self.input_config.clone())
    }

    fn stream(&self, callback: AudioInputCallback<T>) -> Result<()> {
        let (input_stream, _sample_rate, _nchannels) = match self.input_config.sample_format() {
            cpal::SampleFormat::F32 => Self::build_input_stream::<f32>(
                &self.input_device,
                &self.input_config.clone().into(),
                callback,
            ),
            cpal::SampleFormat::I16 => Self::build_input_stream::<i16>(
                &self.input_device,
                &self.input_config.clone().into(),
                callback,
            ),
            cpal::SampleFormat::U16 => Self::build_input_stream::<u16>(
                &self.input_device,
                &self.input_config.clone().into(),
                callback,
            ),
        }?;

        input_stream.play()?;

        // todo: await and block here until we receive a stop signal
        // 12 hours for now
        std::thread::sleep(std::time::Duration::from_secs(12 * 60 * 60));
        drop(input_stream);
        Ok(())
    }
}

impl<T> CpalAudioFile<T>
where
    T: Sample,
{
    // pub fn new(path: PathBuf, looped: bool) -> Result<&'static dyn AudioInput<T>> {
    pub fn new(path: PathBuf, looped: bool) -> Result<Self> {
        let config = AudioOutputConfig::default();
        let host = Self::get_host(&config.backend_config)?;
        let output_device = Self::get_output_device(&host, &config.output_device)?;
        let output_config = output_device.default_output_config()?;
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
            // config,
            host,
            path,
            looped,
            // input_device,
            // input_config,
            output_device,
            output_config,
            phantom: PhantomData,
        })
    }
}

impl<T> AudioInput<T> for CpalAudioFile<T>
where
    T: Sample,
{
    fn descriptor(&self) -> Result<proto::grpc::AudioInputDescriptor> {
        Ok(proto::grpc::AudioInputDescriptor {
            backend: "cpal".to_string(),
            device: self.output_device.name()?,
            host: self.host.id().name().to_string(),
            file: self
                .path
                .to_str().unwrap().to_string()
                // .into_os_string()
                // .into_string()
                // .map(|f| f.to_owned())
                // .unwrap()
                // .map_err(|err| err.into())?,
        })
    }

    fn input_stream_params(&self) -> AudioStreamInfo {
        AudioStreamInfo::from(self.output_config.clone())
    }

    fn stream(&self, callback: AudioInputCallback<T>) -> Result<()> {
        let (mut source, _sample_rate, _nchannels) = Self::get_source_from_file(&self.path)?;

        let output_callback = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            // let mut input_fell_behind = false;
            for sample in data {
                *sample = match source.next() {
                    Some(s) => NumCast::from(s).unwrap(),
                    None => {
                        // input_fell_behind = true;
                        0.0
                    }
                };
            }
            // if input_fell_behind {
            //     eprintln!("input stream fell behind: try increasing latency");
            // }
        };
        let output_stream = self.output_device.build_output_stream(
            &self.output_config.clone().into(),
            output_callback,
            |err| eprintln!("an error occurred on output stream: {}", err),
        )?;

        // let (input_stream, _sample_rate, _nchannels) = match self.input_config.sample_format() {
        //     cpal::SampleFormat::F32 => Self::build_input_stream::<f32>(
        //         &self.input_device,
        //         &self.input_config.clone().into(),
        //         callback,
        //     ),
        //     cpal::SampleFormat::I16 => Self::build_input_stream::<i16>(
        //         &self.input_device,
        //         &self.input_config.clone().into(),
        //         callback,
        //     ),
        //     cpal::SampleFormat::U16 => Self::build_input_stream::<u16>(
        //         &self.input_device,
        //         &self.input_config.clone().into(),
        //         callback,
        //     ),
        // }?;

        output_stream.play()?;

        // todo: await and block here until we receive a stop signal
        // 12 hours for now
        std::thread::sleep(std::time::Duration::from_secs(12 * 60 * 60));
        drop(output_stream);
        Ok(())
    }
}

impl<T> CpalAudioBackend for CpalAudioInput<T> {}
impl<T> CpalAudioBackend for CpalAudioFile<T> {}
impl<T> CpalAudioBackend for CpalAudioOutput<T> {}
