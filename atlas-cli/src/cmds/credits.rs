use clap::Parser;

use atlas_api::core::credits::*;
use atlas_api::errors::APIError;
use atlas_api::request::{Callable, Return};

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
            println!("Credits incomes are:\n{:?}", c);
        },
        CreditSubCommand::Transactions(opts) => {
            let c: Result<Return<Transaction>, APIError> = ctx.c.credits().list(opts.q).with(("type", "transactions")).call();

            let c = match c {
                Ok(c) => match c {
                    Return::Paged(c) => c,
                    _ => panic!("bad call"),
                },
                Err(e) => {
                    println!("Error: {:?}", e);
                    return;
                }
            };
            println!("Credits transactions are:\n{:?}", c);
        },
        CreditSubCommand::Transfer(_opts) => {
            let c: Result<Return<Transfer>, APIError> = ctx.c.credits().info().with(("type", "transfer")).call();

            let c = match c {
                Ok(c) => match c {
                    Return::Single(c) => c,
                    _ => panic!("bad call"),
                },
                Err(e) => {
                    println!("Error: {:?}", e);
                    return;
                }
            };
            println!("Credits transfert are:\n{:?}", c);
        },
        CreditSubCommand::Expense(_opts) => {
            let c: Result<Return<ExpenseItems>, APIError> = ctx.c.credits().info().with(("type", "expense-items")).call();

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
        CreditSubCommand::Members(_opts) => {
            let c: Result<Return<MemberListing>, APIError> = ctx.c.credits().info().with(("type", "members")).call();
            let c = match c {
                Ok(c) => match c {
                    Return::Single(c) => c,
                    _ => panic!("bad call"),
                },
                Err(e) => {
                    println!("Error: {:?}", e);
                    return;
                }
            };
            println!("Credits are:\n{:?}", c);
        },
    }
}
