//! Everything related to CLI option and argument handling.
//!

use std::path::PathBuf;
use clap::{crate_authors, AppSettings, Parser};

/// Binary name
pub(crate) const NAME: &str = "atlas";
/// Binary version, different from the API itself represented the crate.
pub(crate) const VERSION: &str = "0.3.0";

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
    pub(crate) config: Option<PathBuf>,
    /// debug mode
    #[clap(short = 'D', long = "debug")]
    pub(crate) debug: bool,
    /// Verbose mode
    #[clap(short = 'v', long)]
    pub(crate) verbose: bool,
    /// Display version and exit
    #[clap(short = 'V', long = "version")]
    pub(crate) version: bool,
    /// Subcommands
    #[clap(subcommand)]
    pub(crate) subcmd: SubCommand,
}

#[derive(Parser)]
pub(crate) enum SubCommand {
    // Data-specific commands (see data.rs)
    /// Dislays informations about credits
    #[clap(visible_alias = "c")]
    Credits(CredOpts),
    /// Key management
    #[clap(visible_alias = "keys", visible_alias = "k")]
    Key(KeyOpts),
    /// Create, starts, displays measurements
    #[clap(visible_alias = "m")]
    Measurement(MeasurementOpts),
    /// Get informations about probes
    #[clap(visible_alias = "probes", visible_alias = "p")]
    Probe(ProbeOpts),

    // Protocol-specific commands (see protocols.rs)
    /// DNS-related measurements
    Dns(DnsOpts),
    /// HTTP-related measurements
    Http(HttpOpts),
    /// NTP-related measurements
    Ntp(NtpOpts),
    /// ICMP-related measurements
    Ping(PingOpts),
    /// Certificate info management
    #[clap(visible_alias = "cert")]
    TlsCert(TlsOpts),
    /// Traceroute from probes
    #[clap(visible_alias = "tracert")]
    Traceroute(TrrOpts),

    // Useful shortcut (see util.rs)
    /// Displays the default probe IPs
    Ip(IpOpts),
}
