//! This is the main binary shipped with the library.
//!
//! It is a way to both demonstrate the use of the API and a testing tool.
//!

/// External crates
use clap::{crate_version, crate_authors, Parser, AppSettings};

use atlas_rs::client::Client;
use config::Config;

/// Binary name
pub(crate) const NAME: &str = "atlas";
/// Binary version, different from the API itself represented the crate.
pub(crate) const VERSION: &str = "0.1.1";

mod config;

/// Help message
#[derive(Debug, Parser)]
#[clap(name = NAME, about = "Rust CLI for RIPE Atlas.")]
#[clap(version = VERSION, author = crate_authors!())]
#[clap(setting = AppSettings::NoAutoVersion)]
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
    /// Get info on probe
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
