//! data.rs
//!
//! Describes the various data-related commands, subcommands and options for `atlas`
//!

use clap::Parser;

// ------------------------------------------------------------

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

/// Info options
///
#[derive(Parser)]
pub(crate) struct InfoOpts {
    /// Print debug info
    #[clap(short)]
    pub(crate) debug: bool,
    /// Probe ID
    pub(crate) id: Option<u32>,
}

/// List options
///
#[derive(Parser)]
pub(crate) struct ListOpts {
    /// Print debug info
    #[clap(short)]
    pub(crate) debug: bool,
}

// ------------------------------------------------------------

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

// ------------------------------------------------------------

/// Measurements options
///
#[derive(Parser)]
pub(crate) struct MeasurementOpts {
    /// Print debug info
    #[clap(short)]
    pub(crate) debug: bool,
    /// Subcommands
    #[clap(subcommand)]
    pub(crate) subcmd: MeasurementSubCommand,
}

/// Measurement subcommands
///
#[derive(Parser)]
pub(crate) enum MeasurementSubCommand {
    Info(InfoOpts),
    List(ListOpts),
}

// ------------------------------------------------------------

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
pub(crate) struct  ExpOpts {
    /// Print debug info
    #[clap(short)]
    pub(crate) debug: bool,
}

#[derive(Parser)]
pub(crate) struct  TransOpts {
    /// Print debug info
    #[clap(short)]
    pub(crate) debug: bool,
}

#[derive(Parser)]
pub(crate) struct  TransfOpts {
    /// Print debug info
    #[clap(short)]
    pub(crate) debug: bool,
}

