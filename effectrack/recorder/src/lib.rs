use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait};
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct RecorderOptions {
    #[cfg(use_jack)]
    jack: bool,
}

// todo: add another struct for portaudio and use a trait for the recorder that is implemented by
// cpalrecorder and by portaudio recorder

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

pub trait Recorder {
    fn new(options: RecorderOptions) -> Result<Self>
    where
        Self: Sized;
    fn stream_file(&self, path: PathBuf) -> Result<()>;
    fn query(&self) -> Result<()>;
    // pub fn for_file(&self, path: PathBuf) -> Result<Self>
    // pub fn for_input(&self) -> Result<Self>
}

#[derive()]
pub struct CpalRecorder {
    host: cpal::Host,
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
        Ok(Self { host })
    }

    fn stream_file(&self, path: PathBuf) -> Result<()> {
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
