//! This is the main binary shipped with the library.
//!
//! It is a way to both demonstrate the use of the API and a testing tool.
//!

/// External crates
use anyhow::Result;
use clap::{crate_authors, AppSettings, Parser};

use atlas_rs::client::Client;
use config::Config;
use crate::config::default_file;

/// Binary name
pub(crate) const NAME: &str = "atlas";
/// Binary version, different from the API itself represented the crate.
pub(crate) const VERSION: &str = "0.2.0";

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
    #[clap(short = 'p', long = "probe")]
    probe: Option<u32>,
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    // Do not forget to set NoAutoVersion otherwise this is ignored
    if opts.version {
        let v = atlas_rs::version();

        println!("Running API {} CLI {}/{}\n", v, NAME, VERSION);
        std::process::exit(0);
    }

    // Handle configuration loading & defaults
    let cfg = match opts.config {
        Some(fname) => Config::load(&fname).unwrap_or_else(|e| {
            println!("No config file, using defaults: {}", e);
            Config::new()
        }),
        None => Config::load(&default_file().unwrap()).unwrap_or_default(),
    };

    let c = Client::new(&*cfg.api_key).verbose(opts.verbose);

    let pn = opts.probe.unwrap_or_else(|| cfg.default_probe.unwrap());
    let p = c.get_probe(pn);

    match p {
        Ok(p) => println!("Probe {} is:\n{:?}", pn, p),
        Err(e) => {
            println!("Err: {:?}", e);
        }
    };
    Ok(())
}
