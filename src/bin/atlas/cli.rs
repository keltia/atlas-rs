//! Everything related to CLI option and argument handling.
//!


// Std library
//
use std::path::PathBuf;

// External crates
//
use clap::{crate_authors, Parser};

// Import our various data structures & enums
use crate::cmds::credits::CredOpts;
use crate::cmds::ip::IpOpts;
use crate::cmds::keys::KeyOpts;
use crate::cmds::measurements::MeasurementOpts;
use crate::cmds::probes::ProbeOpts;
use crate::proto::{DnsOpts, HttpOpts, NtpOpts, PingOpts, TlsOpts, TrrOpts};

/// Binary name
pub(crate) const NAME: &str = "atlas";
/// Binary version, different from the API itself represented the crate.
pub(crate) const VERSION: &str = "0.3.0";

/// Help message
#[derive(Parser)]
#[clap(name = NAME, about = "Rust CLI for RIPE Atlas.")]
#[clap(version = VERSION, author = crate_authors!())]
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

    /// Display the full version stuff
    Version,

    /// Displays the default probe IPs
    Ip(IpOpts),
}
