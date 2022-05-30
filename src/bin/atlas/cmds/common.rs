use clap::Parser;

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
