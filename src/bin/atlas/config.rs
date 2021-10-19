use std::fs;

use anyhow::{anyhow, Result};
use serde::Deserialize;
use toml::from_str;

/// Default set of probes to be used for queries
#[derive(Debug, Deserialize)]
pub struct ProbeSet {
    // How many probes do we want
    pool_size: Option<usize>,
    // Probe type
    ptype: Option<String>,
    // Value
    value: Option<String>,
    // Include/exclude specific tags
    tags: Option<String>,
}

/// If we want to bill the queries to a specific account (i.e. different from the
/// one behind the API key).
#[derive(Debug, Deserialize)]
pub struct Measurements {
    // RIPE Account ID to be billed for subsequent queries
    bill_to: String,
}

/// `Config` struct with one mandatory argument and optional ones.
#[derive(Debug, Deserialize)]
pub struct Config {
    // API key
    api_key: String,

    // Default probe ID
    default_probe: Option<u32>,

    // Default set of probes
    probe_set: Option<ProbeSet>,

    // Stuff about billing to a specific account
    measurements: Option<Measurements>,
}

/// Fills in the default values
impl Default for Config {
    fn default() -> Self {
        Config {
            api_key: "<CHANGEME>".to_string(),
            default_probe: Some(0),
            probe_set: Some(ProbeSet {
                pool_size: Some(10),
                ptype: Some("area".to_string()),
                value: Some("WW".to_string()),
                tags: Some("".to_string()),
            }),
            measurements: None,
        }
    }
}

/// Methods for Config
impl Config {
    /// Create a `Config` struct with default values.
    pub fn new() -> Config {
        Config {
            ..Default::default()
        }
    }

    /// Loads the configuration from the named file.
    pub fn load(fname: &str) -> anyhow::Result<Self> {
        let content = fs::read_to_string(fname)?;
        println!("{:?}", content);
        Ok(toml::from_str(&content).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let c = Config::new();

        assert_eq!("<CHANGEME>", c.api_key);
        assert_eq!(Some(0), c.default_probe);
    }

    #[test]
    fn test_from_ok() {
        let c = Config::load("src/bin/atlas/config.toml").unwrap();

        assert_eq!("no-way-i-tell-you", c.api_key);
        assert_eq!(Some(666), c.default_probe);
    }

    #[test]
    #[should_panic]
    fn test_from_nok() {
        assert!(Config::load("/dev/null").is_err())
    }

}
