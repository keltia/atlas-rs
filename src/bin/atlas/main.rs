use clap::{Parser, crate_version};

use atlas_rs::client::Client;
use config::Config;

mod config;

/// Help message
#[derive(Debug, Parser)]
#[clap(name = "atlas", about = "Rust CLI for RIPE Atlas.")]
#[clap(version = crate_version!())]
struct Opts {
    /// configuration file
    #[clap(short = 'c', long)]
    config: Option<String>,
    /// debug mode
    #[clap(short = 'D', long = "debug")]
    debug: bool,
    /// Verbose mode
    #[clap(short = 'v', long)]
    verbose: bool,
    /// Display version and exit
    #[clap(short = 'V', long = "version")]
    version: bool,
    /// Search for workstation
    #[clap(short, long = "probe")]
    probe: Option<u32>,
}

fn main() {
    let opts: Opts = Opts::parse();

    let cfg = Config::load("src/bin/atlas/config.toml").unwrap();
    let c = Client::new(&*cfg.api_key)
        .verbose(opts.verbose);

    if opts.version {
        let v = atlas_rs::version();
        println!("Running {}\n{:#?}", v, c);
    } else {
        let pn = opts.probe.unwrap();
        let p = c.get_probe(pn);

        match p {
            Ok(p) => println!("Probe {} is:\n{:?}", pn, p),
            Err(e) => {
                println!("Err: {}", e);
            }
        };

    }
}
