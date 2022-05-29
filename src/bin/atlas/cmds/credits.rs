use clap::Parser;

use atlas_rs::core::credits::*;

use crate::cmds::common::{InfoOpts, ListOpts};
use crate::Context;

/// Credits options
///
#[derive(Parser)]
pub(crate) struct CredOpts {
    /// Subcommands
    #[clap(subcommand)]
    pub(crate) subcmd: CreditSubCommand,
}

/// Credit subcommands
///
#[derive(Parser)]
pub(crate) enum CreditSubCommand {
    Info(InfoOpts),
    Income(ListOpts),
    Expense(ExpOpts),
    Transactions(TransOpts),
    Transfer(TransfOpts),
}

#[derive(Parser)]
pub(crate) struct ExpOpts {
    /// Print debug info
    #[clap(short)]
    pub(crate) debug: bool,
}

#[derive(Parser)]
pub(crate) struct TransOpts {
    /// Print debug info
    #[clap(short)]
    pub(crate) debug: bool,
}

#[derive(Parser)]
pub(crate) struct TransfOpts {
    /// Print debug info
    #[clap(short)]
    pub(crate) debug: bool,
}

pub(crate) fn cmd_credits(ctx: &Context, opts: CredOpts) {
    match opts.subcmd {
        CreditSubCommand::Info(_opts) => {
            let cred: Credits = ctx.c.credits().info().unwrap();
            println!("Credits are {:?}", &cred);
        }
        CreditSubCommand::Income(_opts) => (),
        CreditSubCommand::Transactions(_opts) => (),
        CreditSubCommand::Transfer(_opts) => (),
        CreditSubCommand::Expense(_opts) => (),
    }
}
