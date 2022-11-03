//! # atlas-api
//!
//! The `atlas-api` crate provides with a high-level [Rust] API to the RIPE Atlas probes and
//! measurement network.
//!
//! `atlas-api` is a blocking HTTP client for now, it may evolve into a more proper
//! async client later although I am not sure it is worth the complexity.  It uses the
//! [reqwest] HTTP library for the API calls, supporting all the `reqwest` features
//! including proxy support, etc.
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
//! It was originally a port of my [Go] library called [ripe-atlas] in the process of learning
//! [Rust] but it evolved very fast into a more properly idiomatic Rust library.
//!
//! [cargo-features]: https://doc.rust-lang.org/stable/cargo/reference/manifest.html#the-features-section
//! [Go]: https://golang.org/
//! [reqwest]: https://docs.rs/reqwest/
//! [ripe-atlas]: https://github.com/keltia/ripe-atlas/
//! [ripe-docs]: https://beta-docs.atlas.ripe.net/apis/
//! [Rust]: https://rust-lang.org/
//!

use clap::{crate_name, crate_version};
use log::info;

pub mod client;
pub mod core;
pub mod errors;
pub mod option;
pub mod param;
pub mod request;

/// Basic version string for the API.
///
/// Examples:
/// ```rs
/// use atlas_api::version;
///
/// println!("{}", atlas_api::version());
/// ```
///
pub fn version() -> String {
    info!("in version");
    format!("{}/{}", crate_name!(), crate_version!())
}

/// Simple macro to generate PathBuf from a series of entries
///
/// Example:
/// ```rust
/// # use std::path::PathBuf;
/// use atlas_api::makepath;
///
/// let p = makepath!("testdata", "config.toml");
///
/// assert_eq!(PathBuf::from("testdata/config.toml"), p);
/// ```
///
#[macro_export]
macro_rules! makepath {
    ($($item:expr),+) => {
        [
        $(PathBuf::from($item),)+
        ]
        .iter()
        .collect()
    };
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
