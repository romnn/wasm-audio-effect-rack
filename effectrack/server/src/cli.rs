use clap::Clap;

#[derive(Clap, Debug, Clone)]
pub struct StartOpts {
    #[clap(short = 'p', long = "port", default_value = "9000")]
    pub port: u16,
    #[cfg(feature = "record")]
    #[clap(short = 'f', long = "play-file")]
    pub play_file: Option<String>,
    #[cfg(feature = "record")]
    #[clap(long = "max-viewers")]
    pub max_viewers: Option<u16>,
    #[cfg(feature = "record")]
    #[clap(long = "no-sound")]
    pub no_sound: bool,
}

#[cfg(feature = "record")]
#[derive(Clap, Debug, Clone)]
pub struct QueryOpts {
    #[clap(short = 'd', long = "device")]
    pub device: Option<String>,
}
#[derive(Clap, Debug, Clone)]
pub enum Commands {
    #[clap(name = "start", about = "start the server")]
    Start(StartOpts),
    #[cfg(feature = "record")]
    #[clap(name = "query", about = "query available audio devices on the host")]
    Query(QueryOpts),
}

#[derive(Clap, Debug, Clone)]
#[clap(
    version = "1.0",
    author = "romnn <contact@romnn.com>",
    setting = clap::AppSettings::ColoredHelp,
    setting = clap::AppSettings::ArgRequiredElseHelp,
)]
pub struct Opts {
    #[cfg(feature = "record")]
    #[clap(short = 'i', long = "input-device")]
    pub input_device: Option<String>,

    #[cfg(feature = "record")]
    #[clap(short = 'o', long = "output-device")]
    pub output_device: Option<String>,

    #[cfg(feature = "record")]
    #[clap(long = "latency", default_value = "150")]
    pub latency: u32,

    #[cfg(use_jack)]
    #[clap(long = "jack", about = "use jack audio backend")]
    pub use_jack_backend: bool,

    #[cfg(feature = "portaudio")]
    #[clap(long = "portaudio", about = "use portaudio audio backend")]
    pub use_portaudio_backend: bool,

    #[clap(subcommand)]
    pub commands: Option<Commands>,
}

// audio devices can be selected using the web control interface and the cli arg is just the default
