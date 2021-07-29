extern crate anyhow;
extern crate clap;
extern crate cpal;
extern crate rodio;

use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::BufferSize;
use cpal::StreamConfig;

#[derive(Debug)]
struct Opt {
    #[cfg(all(
        any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd"),
        feature = "jack"
    ))]
    jack: bool,

    device: String,
}

impl Opt {
    fn from_args() -> Self {
        let app = clap::App::new("beep").arg_from_usage("[DEVICE] 'The audio device to use'");
        #[cfg(all(
            any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd"),
            feature = "jack"
        ))]
        let app = app.arg_from_usage("-j, --jack 'Use the JACK host");
        let matches = app.get_matches();
        let device = matches.value_of("DEVICE").unwrap_or("default").to_string();

        #[cfg(all(
            any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd"),
            feature = "jack"
        ))]
        return Opt {
            jack: matches.is_present("jack"),
            device,
        };

        #[cfg(any(
            not(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd")),
            not(feature = "jack")
        ))]
        Opt { device }
    }
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    // Conditionally compile with jack if the feature is specified.
    #[cfg(all(
        any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd"),
        feature = "jack"
    ))]
    // Manually check for flags. Can be passed through cargo with -- e.g.
    // cargo run --release --example beep --features jack -- --jack
    let host = if opt.jack {
        cpal::host_from_id(cpal::available_hosts()
            .into_iter()
            .find(|id| *id == cpal::HostId::Jack)
            .expect(
                "make sure --features jack is specified. only works on OSes where jack is available",
            )).expect("jack host unavailable")
    } else {
        cpal::default_host()
    };

    #[cfg(any(
        not(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd")),
        not(feature = "jack")
    ))]
    let host = cpal::default_host();

    let device = if opt.device == "default" {
        host.default_output_device()
    } else {
        host.output_devices()?
            .find(|x| x.name().map(|y| y == opt.device).unwrap_or(false))
    }
    .expect("failed to find output device");
    println!("Output device: {}", device.name()?);

    let config = device.default_output_config().unwrap();
    // let config = StreamConfig {
    //     channels: 2,
    //     buffer_size: BufferSize::Fixed(2048),
    //     sample_rate: cpal::SampleRate(44_100),
    // };
    // config.buffer_size = BufferSize::Fixed(1024);
    println!("Default output config: {:?}", config);

    match config.sample_format() {
    // match config.sample_format {
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into()),
        cpal::SampleFormat::I16 => run::<i16>(&device, &config.into()),
        cpal::SampleFormat::U16 => run::<u16>(&device, &config.into()),
    }
}

pub fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> Result<(), anyhow::Error>
where
    T: cpal::Sample + std::fmt::Display,
{
    // let sample_rate = config.sample_rate.0 as f32;
    // let channels = config.channels as usize;

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let file = BufReader::new(
        File::open("/home/roman/dev/wasm-audio-effect-rack/experimental/audio-samples/roddy.wav")
            .unwrap(),
    );
    // Decode that sound file into a source
    let mut source = Decoder::new(file).unwrap().convert_samples();
    let sample_rate = source.sample_rate() as f32;
    let channels = source.channels() as usize;
    println!("sample_rate: {}", sample_rate);
    println!("channels: {}", channels);

    // Produce a sinusoid of maximum amplitude.
    let mut sample_clock = 0f32;
    // let mut next_value = move || {
    // sample_clock = (sample_clock + 1.0) % sample_rate;
    // sample_clock = (sample_clock + 1.0) % sample_rate;
    // (sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate).sin()
    // if sample_clock == 0.0 { 1.0 } else { 0.0 }
    // sample_clock
    // sample_clock.sin()
    // source.next().unwrap()
    // };

    let stream = device.build_output_stream(
        config,
        // move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
        move |data, _| {
            // println!("{:?}", data);
            println!("{:?}", data.len());
            // println!("{}", data[0]);
            // println!("{}", data[0].to_f32());
            // write_data(data, channels, &mut next_value)
            data.iter_mut()
                .for_each(|d| *d = source.next().unwrap_or(0f32))
        },
        err_fn,
    )?;
    stream.play()?;

    std::thread::sleep(std::time::Duration::from_millis(20 * 1000));

    Ok(())
}

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
where
    T: cpal::Sample,
{
    for frame in output.chunks_mut(channels) {
        let value: T = cpal::Sample::from::<f32>(&next_sample());
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}
