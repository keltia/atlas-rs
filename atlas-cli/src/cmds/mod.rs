use clap::Parser;

use crate::cli::{NAME, VERSION};

pub(crate) mod credits;
pub(crate) mod ip;
pub(crate) mod keys;
pub(crate) mod measurements;
pub(crate) mod probes;

// These two struct are shared amongst the different commands/subcommands
//

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
    /// Query parameters
    #[clap(short)]
    pub(crate) q: Vec<String>,
}

/// Display version
///
pub(crate) fn cmd_version() -> String {
    format!("Running API {} CLI {}/{}", atlas_api::version(), NAME, VERSION)
}
