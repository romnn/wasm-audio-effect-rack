// use crate::{cpal::CpalRecorder, AudioBackend, AudioBackendConfig};
use crate::{cpal::CpalRecorder, RecorderConfig};
use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::BufferSize;
use cpal::StreamConfig;
use ndarray::prelude::*;
use ndarray::Array;
use ndarray::{concatenate, RemoveAxis, Slice};
use num::traits::{Float, FloatConst, Zero};
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Clone)]
pub struct CpalAudioBackend {
    config: AudioBackendConfig,
}

impl AudioBackend for CpalAudioBackend {
    type Rec = CpalRecorder;

    fn new(config: AudioBackendConfig) -> Self {
        Self { config }
    }

    fn new_recorder(&self) -> Result<Self::Rec> {
        cfg_if::cfg_if! {
            if #[cfg(use_jack)] {
                let host = if self.config.jack {
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
        let input_device = if let Some(device) = &self.config.input_device {
            host.input_devices()?
                .find(|x| x.name().map(|y| y == *device).unwrap_or(false))
        } else {
            host.default_input_device()
        }
        .ok_or(cpal::BackendSpecificError {
            description: format!(
                "unable to find or use selected input device: \"{}\"",
                self.config
                    .input_device
                    .as_ref()
                    .unwrap_or(&"default".to_string())
            )
            .to_string(),
        })?;
        let output_device = if let Some(device) = &self.config.output_device {
            host.output_devices()?
                .find(|x| x.name().map(|y| y == *device).unwrap_or(false))
        } else {
            host.default_output_device()
        }
        .ok_or(cpal::BackendSpecificError {
            description: format!(
                "unable to find or use selected output device: \"{}\"",
                self.config
                    .output_device
                    .as_ref()
                    .unwrap_or(&"default".to_string())
            )
            .to_string(),
        })?;
        Ok(Self::Rec {
            host,
            input_device,
            output_device,
            latency: self.config.latency,
        })
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
            }
        }
        Ok(())
    }
}