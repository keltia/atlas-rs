use clap::Parser;

use atlas_api::core::probes::*;
use atlas_api::errors::APIError;
use atlas_api::request::{Callable, Return};

use crate::cmds::{InfoOpts, ListOpts};
use crate::Context;

/// Probe options
///
#[derive(Parser)]
pub(crate) struct ProbeOpts {
    /// Print debug info
    #[clap(short)]
    pub(crate) debug: bool,
    /// Subcommands
    #[clap(subcommand)]
    pub(crate) subcmd: ProbeSubCommand,
}

/// Probe subcommands
///
#[derive(Parser)]
pub(crate) enum ProbeSubCommand {
    Info(InfoOpts),
    List(ListOpts),
}

pub(crate) fn cmd_probes(ctx: &Context, opts: ProbeOpts) {
    match opts.subcmd {
        ProbeSubCommand::Info(opts) => {
            let pn = opts.id.unwrap_or_else(|| ctx.cfg.default_probe.unwrap());

            let p: Result<Return<Probe>, APIError> = ctx.c.probe().get(pn).call();
            let p = match p {
                Ok(p) => match p {
                    Return::Single(p) => p,
                    _ => panic!("bad call"),
                },
                Err(e) => {
                    println!("Probe {} not found!", pn);
                    println!("Error: {:#?}", e);
                    return;
                }
            };
            println!("Probe {} is:\n{:?}", pn, p);
        }
        ProbeSubCommand::List(opts) => {
            let p: Result<Return<Probe>, APIError> = ctx.c.probe().list(opts.q).call();

            let p = match p {
                Ok(p) => match p {
                    Return::Paged(vp) => vp,
                    _ => panic!("bad call"),
                },
                Err(e) => {
                    println!("Error: {:#?}", e);
                    vec![]
                }
            };
            println!("{} probes found!", p.len());
        }
    }
}
