use clap::{crate_name, crate_version};

pub mod client;
pub mod common;
pub mod probes;
pub mod errors;

/// Basic version string
///
/// Examples:
/// ```
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
