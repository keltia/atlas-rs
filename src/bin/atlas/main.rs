//! This is the main binary shipped with the library.
//!
//! It is a way to both demonstrate the use of the API and a testing tool.
//!

/// External crates
///
use anyhow::Result;
use clap::Parser;

/// API-related ones.
///
use atlas_rs::client::ClientBuilder;
use atlas_rs::errors::APIError;
use atlas_rs::keys::Key;
use atlas_rs::probes::Probe;

// Link with other modules.

mod cli;
mod config;
mod data;
mod proto;
mod util;

use cli::{Opts, SubCommand, NAME, VERSION};
use config::{default_file, Config};
use data::{KeySubCommand, ProbeSubCommand};

/// Wrapper to load configuration
///
fn load_config(opts: &Opts) -> Config {
    // Handle configuration loading & defaults
    match &opts.config {
        Some(fname) => Config::load(fname).unwrap_or_else(|e| {
            println!("No config file, using defaults: {}", e);
            Config::new()
        }),
        None => Config::load(&default_file().unwrap()).unwrap_or_default(),
    }
}

/// Main entry point
///
/// It returns an empty `Result` which enable use this type with `?`.
///
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
        SubCommand::Probe(opts) => match opts.subcmd {
            ProbeSubCommand::Info(opts) => {
                let pn = opts.id.unwrap_or_else(|| cfg.default_probe.unwrap());

                let p: Probe = c.probe().get(pn).call()?;
                println!("Probe {} is:\n{:?}", pn, p);
            }
            ProbeSubCommand::List(_opts) => (),
        },
        SubCommand::Key(opts) => match opts.subcmd {
            KeySubCommand::Info(opts) => {
                let uuid = opts.uuid.unwrap_or_else(|| cfg.api_key.clone());

                let k: Key = c.keys().get(uuid.as_str()).call()?;
                println!("Key {} is:\n{:?}", uuid, k);
            }
            KeySubCommand::List(_opts) => (),
        },
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
        SubCommand::Ip(opts) => {
            let pn = opts.id.unwrap_or_else(|| cfg.default_probe.unwrap());
            let p: Result<Probe, APIError> = c.probe().get(pn).call();

            match p {
                Ok(p) => {
                    let ip4 = p.address_v4.unwrap_or_else(|| "None".to_string());
                    let ip6 = p.address_v6.unwrap_or_else(|| "None".to_string());

                    let ip = format!("IPv4: {} IPv6: {}", ip4, ip6);
                    println!("Probe {} has the following IP:\n{}", pn, ip)
                }
                Err(e) => {
                    println!("Err: {:?}", e);
                }
            };
        }
    }
    Ok(())
}
