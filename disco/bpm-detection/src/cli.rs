use clap::Clap;
use std::path::{Path, PathBuf};

#[derive(Clap, Debug, Clone)]
struct ApplyOpts {
    #[clap(short = 'i', long = "image")]
    image_path: String,

    #[clap(short = 'o', long = "output")]
    output_path: Option<String>,
    // #[clap(short = 'p', long = "port", default_value = "3000")]
    // port: u16,

    // #[clap(short = 'n', long = "pages")]
    // max_pages: Option<u32>,
}

#[derive(Clap, Debug, Clone)]
#[clap(
    name = "hipster",
    version = "1.0",
    about = "todo",
    author = "romnn <contact@romnn.com>"
)]

enum Hipster {
    #[clap(name = "apply")]
    Apply(ApplyOpts),
}

#[derive(Clap, Debug, Clone)]
#[clap(
    name = "hipster",
    about = "TODO",
    version = "1.0",
    author = "romnn <contact@romnn.com>"
)]
pub struct Opts {
    #[clap(short = 'v', parse(from_occurrences))]
    verbosity: u8,

    #[clap(subcommand)]
    commands: Option<Hipster>,
}

fn main() {
    let opts: Opts = Opts::parse();

    if let Some(subcommand) = opts.commands {
        match subcommand {
            Hipster::Apply(cfg) => {
                println!("apply:  {:?}", cfg);
            }
        }
    }
}
