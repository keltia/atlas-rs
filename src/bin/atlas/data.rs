use clap::Parser;


/// Probe subcommands
#[derive(Parser)]
pub(crate) struct ProbeOpts {
    /// Print debug info
    #[clap(short)]
    pub(crate) debug: bool,
    /// Sub-commands
    #[clap(subcommand)]
    pub(crate) subcmd: ProbeSubCommand,
}

/// Probe sub-commands
#[derive(Parser)]
pub(crate) enum ProbeSubCommand {
    Info(InfoOpts),
    List(ListOpts),
}

#[derive(Parser)]
pub(crate) struct InfoOpts {
    /// Print debug info
    #[clap(short)]
    pub(crate) debug: bool,
    /// Probe ID
    pub(crate) id: Option<u32>,
}

#[derive(Parser)]
pub(crate) struct ListOpts {
    /// Print debug info
    #[clap(short)]
    pub(crate) debug: bool,
}

/// Key sub-commands
#[derive(Parser)]
pub(crate) struct KeyOpts {
    /// Print debug info
    #[clap(short)]
    pub(crate) debug: bool,
    /// Sub-commands
    #[clap(subcommand)]
    pub(crate) subcmd: KeySubCommand,
}

#[derive(Parser)]
pub(crate) enum KeySubCommand {
    Info(InfoOpts),
    List(ListOpts),
}

/// Measurement sub-commands
#[derive(Parser)]
pub(crate) struct MeasurementOpts {
    /// Print debug info
    #[clap(short)]
    pub(crate) debug: bool,
    /// Sub-commands
    #[clap(subcommand)]
    pub(crate) subcmd: MeasurementSubCommand,
}

#[derive(Parser)]
pub(crate) enum MeasurementSubCommand {
    Info(InfoOpts),
    List(ListOpts),
}

/// Credit sub-commands
#[derive(Parser)]
pub(crate) struct CredOpts {
    /// Sub-commands
    #[clap(subcommand)]
    pub(crate) subcmd: CreditSubCommand,
}

#[derive(Parser)]
pub(crate) enum CreditSubCommand {
    Info(InfoOpts),
    List(ListOpts),
}
