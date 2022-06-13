use clap::Parser;

use atlas_rs::core::credits::*;
use atlas_rs::errors::APIError;
use atlas_rs::request::{Callable, Return};

use crate::cmds::{InfoOpts, ListOpts};
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
    Members(MembOpts),
    Transactions(ListOpts),
    Transfer(TransfOpts),
}

#[derive(Parser)]
pub(crate) struct ExpOpts {
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

#[derive(Parser)]
pub(crate) struct MembOpts {
    /// Print debug info
    #[clap(short)]
    pub(crate) debug: bool,
}

pub(crate) fn cmd_credits(ctx: &Context, opts: CredOpts) {
    match opts.subcmd {
        CreditSubCommand::Info(_opts) => {
            let c: Result<Return<Credits>, APIError> = ctx.c.credits().info().call();

            let c = match c {
                Ok(c) => match c {
                    Return::Single(c) => c,
                    _ => panic!("bad call"),
                },
                Err(e) => {
                    println!("Error: {:#?}", e);
                    return;
                }
            };
            println!("Credits are:\n{:?}", c);
        }
        CreditSubCommand::Income(_opts) => {
            let c: Result<Return<IncomeItems>, APIError> = ctx.c.credits().info().with(("type", "income-items")).call();

            let c = match c {
                Ok(c) => match c {
                    Return::Single(c) => c,
                    _ => panic!("bad call"),
                },
                Err(e) => {
                    println!("Error: {:#?}", e);
                    return;
                }
            };
            println!("Credits are:\n{:?}", c);
        },
        CreditSubCommand::Transactions(opts) => {
            let c: Vec<Transaction> = match ctx.c.credits().with(("type", "transactions")).list(opts.q) {
                Ok(c) => c,
                Err(e) => {
                    println!("Error: {:?}", e);
                    return;
                }
            };
            println!("Credits transactions are:\n{:?}", c);
        },
        CreditSubCommand::Transfer(_opts) => {
            let c: Transfer = match ctx.c.credits().with(("type", "transfer")).info() {
                Ok(c) => c,
                Err(e) => {
                    println!("Error: {:?}", e);
                    return;
                }
            };
            println!("Credits are:\n{:?}", c);
        },
        CreditSubCommand::Expense(_opts) => {
            let c: ExpenseItems = match ctx.c.credits().with(("type", "expense-items")).info() {
                Ok(c) => c,
                Err(e) => {
                    println!("Error: {:?}", e);
                    return;
                }
            };
            println!("Credits are:\n{:?}", c);
        },
        CreditSubCommand::Members(_opts) => {
            let c: MemberListing = match ctx.c.credits().with(("type", "members")).info() {
                Ok(c) => c,
                Err(e) => {
                    println!("Error: {:?}", e);
                    return;
                }
            };
            println!("Credits are:\n{:?}", c);
        },
    }
}
