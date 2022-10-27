use clap::Parser;

use atlas_rs::core::keys::*;
use atlas_rs::errors::APIError;
use atlas_rs::request::{Callable, Return};

use crate::cmds::ListOpts;
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

            let k: Result<Return<Key>, APIError> = ctx.c.keys().get(uuid.as_str()).call();

            let k = match k {
                Ok(k) => match k {
                    Return::Single(k) => k,
                    _ => panic!("bad call"),
                },
                Err(e) => {
                    println!("Key {} not found!", uuid);
                    println!("Error: {:#?}", e);
                    return;
                }
            };
            println!("Key {} is:\n{:?}", uuid, k);
        }
        KeySubCommand::List(opts) => {
            let vk: Result<Return<Key>, APIError> = ctx.c.keys().list(opts.q).call();

            let vk = match vk {
                Ok(vk) => match vk {
                    Return::Paged(vk) => vk,
                    _ => panic!("bad call"),
                },
                Err(e) => {
                    println!("Error: {:#?}", e);
                    vec![]
                }
            };
            println!("{} keys found!", vk.len());
        }
    }
}
