//! # atlas-rs
//!
//! The `atlas-rs`provides with a high-level [Rust] API to the RIPE Atlas probes and
//! measurement network.
//!
//! `atlas-rs` is a blocking HTTP client for now, it may evolve into a more proper
//! async client later although I am not sure it is worth the complexity.  It uses the
//! [reqwest] HTTP library for the API calls, supporting all the `reqwest` features
//! including proxy support, etc.
//!
//! It includes the `atlas` binary, used both as a showcase of many calls of the API and
//! a test utility for me.
//!
//! ## Proxy features
//!
//! **NOTE**: System proxies are enabled by default.
//!
//! System proxies look in environment variables to set HTTP or HTTPS proxies.
//!
//! `HTTP_PROXY` or `http_proxy` provide http proxies for http connections while
//! `HTTPS_PROXY` or `https_proxy` provide HTTPS proxies for HTTPS connections.
//! `ALL_PROXY` or `all_proxy`is a "catch-all" setting for all protocols.
//! `NO_PROXY` or `no_proxy` can prevent using any of the proxies.
//!
//! ## Official Documentation
//!
//! Metadata API (probes, keys, credits) and Measurement Results API.
//!
//! - [Main RIPE Atlas site](https://atlas.ripe.net/)
//! - [REST API Documentation](https://atlas.ripe.net/docs/api/v2/manual/)
//! - [REST API Reference](https://atlas.ripe.net/docs/api/v2/reference/)

//! ## History
//!
//! It was originally a port of my [Go] library called [ripe-atlas] (see the `flat-api`
//! features below) in the process of learning [Rust] but it evolved very fast into a more
//! proper idiomatic Rust library.
//!
//! ## Optional Features
//!
//! The following are a list of [Cargo features][cargo-features] that can be
//! enabled or disabled:
//!
//! - **flat-api**: Provides the flatter API calls (aka `c.get_probe(n)`, etc.)
//! - **alt-api**: Provides an alternate set of API calls (`Probe::get(cl, n)`, etc.)
//!
//! [cargo-features]: https://doc.rust-lang.org/stable/cargo/reference/manifest.html#the-features-section
//! [Go]: https://golang.org/
//! [reqwest]: https://docs.rs/reqwest/
//! [ripe-atlas]: https://github.com/keltia/ripe-atlas/
//! [ripe-docs]: https://beta-docs.atlas.ripe.net/apis/
//! [Rust]: https://rust-lang.org/
//!

use clap::{crate_name, crate_version};

// main modules
pub mod anchor_measurements;
pub mod anchors;
pub mod client;
pub mod common;
pub mod credits;
pub mod errors;
pub mod keys;
pub mod measurement;
pub mod option;
pub mod probes;
pub mod request;

/// Basic version string for the API.
///
/// Examples:
/// ```rs
/// use atlas_rs::version;
///
/// println!("{}", atlas_rs::version());
/// ```
///
pub fn version() -> String {
    format!("{}/{}", crate_name!(), crate_version!())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let v = version();

        assert_eq!(format!("{}/{}", crate_name!(), crate_version!()), v);
    }
}
