use clap::{crate_authors, AppSettings, Parser};

/// Binary name
pub(crate) const NAME: &str = "atlas";
/// Binary version, different from the API itself represented the crate.
pub(crate) const VERSION: &str = "0.2.0";

// Import our various data structures & enums
use crate::data::{CredOpts, KeyOpts, MeasurementOpts, ProbeOpts};
use crate::proto::{DnsOpts, HttpOpts, NtpOpts, PingOpts, TlsOpts, TrrOpts};
use crate::util::IpOpts;


/// Help message
#[derive(Parser)]
#[clap(name = NAME, about = "Rust CLI for RIPE Atlas.")]
#[clap(version = VERSION, author = crate_authors!())]
#[clap(setting = AppSettings::NoAutoVersion)]
pub(crate) struct Opts {
    /// configuration file
    #[clap(short = 'c', long)]
    pub(crate) config: Option<String>,
    /// debug mode
    #[clap(short = 'D', long = "debug")]
    pub(crate) debug: bool,
    /// Verbose mode
    #[clap(short = 'v', long)]
    pub(crate) verbose: bool,
    /// Display version and exit
    #[clap(short = 'V', long = "version")]
    pub(crate) version: bool,
    /// Get info on probe
    #[clap(short = 'p', long = "probe")]
    pub(crate) probe: Option<u32>,
    /// Subcommands
    #[clap(subcommand)]
    pub(crate) subcmd: SubCommand,
}

#[derive(Parser)]
pub(crate) enum SubCommand {
    /// Data-specific commands (see data.rs)
    Credits(CredOpts),
    Key(KeyOpts),
    Measurement(MeasurementOpts),
    Probe(ProbeOpts),
    /// Protocol-specific commands (see protocols.rs)
    Dns(DnsOpts),
    Http(HttpOpts),
    Ntp(NtpOpts),
    Ping(PingOpts),
    TlsCert(TlsOpts),
    Traceroute(TrrOpts),
    /// Useful shortcut (see util.rs)
    Ip(IpOpts),
}

