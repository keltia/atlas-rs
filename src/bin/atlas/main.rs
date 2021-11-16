//! This is the main binary shipped with the library.
//!
//! It is a way to both demonstrate the use of the API and a testing tool.
//!

/// External crates
use anyhow::Result;
use clap::Parser;

use atlas_rs::client::ClientBuilder;

mod cli;
mod config;
mod data;
mod util;

use config::{Config, default_file};
use cli::{Opts, SubCommand, NAME, VERSION};
use data::ProbeSubCommand;

/// Wrapper to load configuration
fn load_config(opts: &Opts) -> Config {
    // Handle configuration loading & defaults
    return match &opts.config {
        Some(fname) => Config::load(&fname).unwrap_or_else(|e| {
            println!("No config file, using defaults: {}", e);
            Config::new()
        }),
        None => Config::load(&default_file().unwrap()).unwrap_or_default(),
    }
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
    let cfg = load_config(&opts);

    let c = ClientBuilder::new()
        .api_key(&*cfg.api_key)
        .verbose(opts.verbose)
        .build()?;

    match opts.subcmd {
        // data related commands
        SubCommand::Probe(opts) => {
            match opts.subcmd {
                ProbeSubCommand::Info(opts) => {
                    let pn = opts.id.unwrap_or_else(|| cfg.default_probe.unwrap());
                    let p = c.get_probe(pn);

                    match p {
                        Ok(p) => println!("Probe {} is:\n{:?}", pn, p),
                        Err(e) => {
                            println!("Err: {:?}", e);
                        }
                    };
                },
                ProbeSubCommand::List(_opts) => (),
            }
        },
        SubCommand::Key(_opts) => (),
        SubCommand::Credits(_opts) => (),
        SubCommand::Measurement(_opts) => (),
        // protocols-related commands
        SubCommand::Dns(_opts) => (),
        SubCommand::Http(_opts) => (),
        SubCommand::Ntp(_opts) => (),
        SubCommand::Ping(_opts) => (),
        SubCommand::TlsCert(_opts) => (),
        SubCommand::Traceroute(_opts) => (),
        // extra utility command
        SubCommand::Ip(_opts) => (),
    }
    Ok(())
}
