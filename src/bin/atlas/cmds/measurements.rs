//use atlas_rs::core::measurements::*;

use clap::Parser;

use crate::cmds::common::{InfoOpts, ListOpts};

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

