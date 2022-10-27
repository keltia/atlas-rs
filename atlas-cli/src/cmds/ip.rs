use clap::Parser;

use atlas_api::core::probes::Probe;
use atlas_api::errors::APIError;
use atlas_api::request::{Callable, Return};

use crate::Context;

#[derive(Parser)]
pub(crate) struct IpOpts {
    /// Print debug info
    #[clap(short)]
    pub(crate) debug: bool,
    /// Probe ID
    pub(crate) id: Option<u32>,
}

pub(crate) fn cmd_ip(ctx: &Context, opts: IpOpts) {
    let pn = opts.id.unwrap_or_else(|| ctx.cfg.default_probe.unwrap());
    let p: Result<Return<Probe>, APIError> = ctx.c.probe().get(pn).call();

    match p {
        Ok(p) => {
            // Decapsulate the result
            let p: Probe = match p {
                Return::Single(p) => p,
                _ => panic!("bad call"),
            };
            let ip4 = p.address_v4.unwrap_or_else(|| "None".to_string());
            let ip6 = p.address_v6.unwrap_or_else(|| "None".to_string());

            let ip = format!("IPv4: {} IPv6: {}", ip4, ip6);
            println!("Probe {} has the following IP:\n{}", pn, ip)
        }
        Err(e) => {
            println!("Err: {:?}", e);
        }
    };
}
