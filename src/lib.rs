//! Root crate for the API library
//!
use clap::{crate_name, crate_version};

// main modules
pub mod anchors;
pub mod anchor_measurements;
pub mod client;
pub mod common;
pub mod credits;
pub mod errors;
pub mod keys;
pub mod ops;
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
