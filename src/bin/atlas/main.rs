//! This is the main binary shipped with the library.
//!
//! It is a way to both demonstrate the use of the API and a testing tool.
//!

// External crates
//
use anyhow::Result;
use clap::Parser;

// API-related ones.
//
use atlas_rs::client::{Client, ClientBuilder};
use cli::{NAME, Opts, SubCommand, VERSION};
use config::{Config, default_file};

// Import all subcommands
use crate::cmds::credits::cmd_credits;
use crate::cmds::ip::cmd_ip;
use crate::cmds::keys::cmd_keys;
use crate::cmds::probes::cmd_probes;

// Link with other modules.
mod cli;
mod cmds;
mod config;
mod proto;

/// Wrapper to load configuration
///
fn load_config(opts: &Opts) -> Config {
    // Handle configuration loading & defaults
    match &opts.config {
        Some(fname) => Config::load(fname).unwrap_or_else(|e| {
            println!("No config file, using defaults: {}", e);
            Config::new()
        }),
        None => {
            let cnf = default_file().unwrap();
            Config::load(&cnf).unwrap_or_default()
        },
    }
}

/// This contains our common objects we need into commands & subcommands
///
pub struct Context {
    c: Client,
    cfg: Config,
}

/// Main entry point
///
/// It returns an empty `Result` which enable use this type with `?`.
///
fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    if opts.debug {
        println!("DEBUG MODE");
    }

    // Handle configuration loading & defaults
    let cfg = load_config(&opts);

    let c = ClientBuilder::new()
        .api_key(&*cfg.api_key)
        .verbose(opts.verbose)
        .build()?;

    // create the context of every operation
    let ctx = Context { c, cfg };

    match opts.subcmd {
        // data related commands
        SubCommand::Probe(opts) => cmd_probes(&ctx, opts),
        SubCommand::Key(opts) => cmd_keys(&ctx, opts),
        SubCommand::Credits(opts) => cmd_credits(&ctx, opts),
        SubCommand::Measurement(_opts) => (),
        // protocols-related commands
        SubCommand::Dns(_opts) => (),
        SubCommand::Http(_opts) => (),
        SubCommand::Ntp(_opts) => (),
        SubCommand::Ping(_opts) => (),
        SubCommand::TlsCert(_opts) => (),
        SubCommand::Traceroute(_opts) => (),
        // extra utility command
        SubCommand::Ip(opts) => cmd_ip(&ctx, opts),
        SubCommand::Version => {
            let v = atlas_rs::version();

            println!("Running API {} CLI {}/{}\n", v, NAME, VERSION);
            std::process::exit(0);
        }
    }
    Ok(())
}
