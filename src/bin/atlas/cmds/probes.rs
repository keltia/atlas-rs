use clap::Parser;

use atlas_rs::core::probes::*;

use crate::cmds::common::{InfoOpts, ListOpts};
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

            let p: Probe = ctx.c.probe().get(pn).unwrap();
            println!("Probe {} is:\n{:?}", pn, p);
        }
        ProbeSubCommand::List(opts) => {
            let p: Vec<Probe> = ctx.c.probe().list(opts.q).unwrap();
            dbg!(&p);
        }
    }
}
