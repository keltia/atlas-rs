use clap::Parser;

#[derive(Parser)]
pub(crate) struct IpOpts {
    /// Print debug info
    #[clap(short)]
    pub(crate) debug: bool,
}

