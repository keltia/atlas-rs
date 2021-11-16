use clap::Parser;

#[derive(Parser)]
pub(crate) struct DnsOpts {
    /// Print debug info
    #[clap(short)]
    pub(crate) debug: bool,
}


#[derive(Parser)]
pub(crate) struct HttpOpts {
    /// Print debug info
    #[clap(short)]
    pub(crate) debug: bool,
}

#[derive(Parser)]
pub(crate) struct NtpOpts {
    /// Print debug info
    #[clap(short)]
    pub(crate) debug: bool,
}

#[derive(Parser)]
pub(crate) struct PingOpts {
    /// Print debug info
    #[clap(short)]
    pub(crate) debug: bool,
}

#[derive(Parser)]
pub(crate) struct TlsOpts {
    /// Print debug info
    #[clap(short)]
    pub(crate) debug: bool,
}

#[derive(Parser)]
pub(crate) struct TrrOpts {
    /// Print debug info
    #[clap(short)]
    pub(crate) debug: bool,
}

