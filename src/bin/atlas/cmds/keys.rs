use clap::Parser;

use atlas_rs::core::keys::*;

use crate::cmds::common::ListOpts;
use crate::Context;

/// Key options
///
#[derive(Parser)]
pub(crate) struct KeyOpts {
    /// Print debug info
    #[clap(short)]
    pub(crate) debug: bool,
    /// Subcommands
    #[clap(subcommand)]
    pub(crate) subcmd: KeySubCommand,
}

/// Key subcommands
///
#[derive(Parser)]
pub(crate) enum KeySubCommand {
    Info(KInfoOpts),
    List(ListOpts),
}

/// Key info options
///
#[derive(Parser)]
pub(crate) struct KInfoOpts {
    /// Print debug info
    #[clap(short)]
    pub(crate) debug: bool,
    /// Probe ID
    pub(crate) uuid: Option<String>,
}

pub(crate) fn cmd_keys(ctx: &Context, opts: KeyOpts) {
    match opts.subcmd {
        KeySubCommand::Info(opts) => {
            let uuid = opts.uuid.unwrap_or_else(|| ctx.cfg.api_key.clone());

            let k: Key = ctx.c.keys().get(uuid.as_str()).unwrap();
            println!("Key {} is:\n{:?}", uuid, k);
        }
        KeySubCommand::List(_opts) => (),
    }
}
