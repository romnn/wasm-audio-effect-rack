use clap::Parser;

#[derive(Debug, Clone)]
pub struct Config {
    pub run: StartOpts,
    pub default: Opts,
}

#[derive(Parser, Debug, Clone)]
pub struct StartOpts {
    #[clap(short = 'p', long = "port", default_value = "9000")]
    pub port: u16,
    #[cfg(feature = "record")]
    #[clap(short = 'f', long = "play-file")]
    pub play_file: Option<String>,
    #[clap(long = "max-sessions")]
    pub max_sessions: Option<usize>,
    #[clap(long = "max-viewers")]
    pub max_viewers: Option<usize>,
    #[clap(long = "max-controllers")]
    pub max_controllers: Option<usize>,
    #[clap(long = "keepalive-sec", default_value = "30")]
    pub max_keepalive_sec: u64,
    #[cfg(feature = "record")]
    #[clap(long = "no-sound")]
    pub no_sound: bool,
}

#[cfg(feature = "record")]
#[derive(Parser, Debug, Clone)]
pub struct QueryOpts {
    #[clap(short = 'd', long = "device")]
    pub device: Option<String>,
}
#[derive(Parser, Debug, Clone)]
pub enum Commands {
    #[clap(name = "start", about = "start the server")]
    Start(StartOpts),
    #[cfg(feature = "record")]
    #[clap(name = "query", about = "query available audio devices on the host")]
    Query(QueryOpts),
}

// setting = AppSettings::ColoredHelp,
// setting = AppSettings::ArgRequiredElseHelp,

#[derive(Parser, Debug, Clone)]
#[clap(version = "1.0", author = "romnn <contact@romnn.com>")]
pub struct Opts {
    #[cfg(feature = "record")]
    #[clap(short = 'i', long = "input-device")]
    pub input_device: Option<String>,

    #[cfg(feature = "record")]
    #[clap(short = 'o', long = "output-device")]
    pub output_device: Option<String>,

    #[cfg(feature = "record")]
    #[clap(long = "latency", default_value = "5")]
    pub latency: u32,

    #[cfg(use_jack)]
    #[clap(long = "jack", about = "use jack audio backend")]
    pub use_jack: bool,

    #[cfg(feature = "portaudio")]
    #[clap(long = "portaudio", about = "use portaudio audio backend")]
    pub use_portaudio: bool,

    #[clap(subcommand)]
    pub commands: Option<Commands>,
}
