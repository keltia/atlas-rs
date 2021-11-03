//! This is the module to handle configuration for the `atlas_rs` client.
//!
//! It takes a [TOML] configuration file of the following format:
//!
//! ```toml
//! # Default configuration file
//!
//! api_key = "no-way-i-tell-you"
//! default_probe = 666
//!
//! [probe_set]
//!
//! pool_size = 42
//! type = "area"
//! value = "WW"
//! tags = "+ipv4"
//! ```
//!
//! We can share the configuration file with the Go `ripe-atlas` of course
//! and this is the default.
//!
//! Examples:
//! ```rs
//! use crate::config::Config;
//!
//! let cfg = Config::new();  // will contain the defaults values from here.
//!
//! println!("Default key is {}", cfg.api_key);
//! ```
//!
//! or
//!
//! ```rs
//!  use atlas_rs::config::Config;
//!
//!  let cfg = Config::load("./atlas.toml").unwrap();
//!
//!  println!("Default key is {}", cfg.api_key);
//! ```
//!
//! [TOML]: https://crates.io/crates/toml

use std::fs;

use serde::Deserialize;

/// Default set of probes to be used for queries
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ProbeSet {
    /// How many probes do we want
    pub pool_size: Option<usize>,
    /// Probe type
    pub ptype: Option<String>,
    /// Value for probe type
    pub value: Option<String>,
    /// Include/exclude specific tags
    pub tags: Option<String>,
}

/// If we want to bill the queries to a specific account (i.e. different from the
/// one behind the API key).
///
/// NOTE: I never used it but it is part of the API.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Measurements {
    /// RIPE Account ID to be billed for subsequent queries
    pub bill_to: String,
}

/// `Config` struct with one mandatory argument and optional ones.
#[derive(Debug, Deserialize)]
pub struct Config {
    /// API key
    pub api_key: String,
    /// Default probe ID
    pub default_probe: Option<u32>,
    /// Default set of probes
    pub probe_set: Option<ProbeSet>,
    /// Stuff about billing to a specific account
    pub measurements: Option<Measurements>,
}

impl Default for Config {
    /// Fills in the default values
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

    /// Loads the configuration from the named file.  Creates a new `Config` object.
    ///
    /// Example:
    ///
    /// ```no_run
    ///
    ///   let cfg = Config::load("./atlas.conf");
    /// ```
    ///
    pub fn load(fname: &str) -> anyhow::Result<Self> {
        let content = fs::read_to_string(fname)?;
        println!("{:?}", content);
        Ok(toml::from_str(&content).unwrap())
    }

    /// Reloads the configuration from the named file.
    ///
    /// Example:
    ///
    /// ```no_run
    ///
    ///   let cfg = Config::load("./atlas.conf");
    ///  #...
    ///   let n = cfg.reload("./new.toml").unwrap();
    /// ```
    ///
    pub fn reload(&mut self, fname: &str) -> anyhow::Result<&mut Self> {
        let content = fs::read_to_string(fname)?;
        let val: Config = toml::from_str(&content).unwrap();

        // copy non-null values
        if val.api_key != "".to_string() {
            self.api_key = val.api_key;
        }

        if val.default_probe != None {
            self.default_probe = val.default_probe;
        }

        if val.probe_set != None {
            self.probe_set = val.probe_set.clone();
        }

        if val.measurements != None {
            self.probe_set = val.probe_set.clone();
        }

        Ok(self)
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
    fn test_load_ok() {
        let c = Config::load("src/bin/atlas/config.toml").unwrap();

        assert_eq!("no-way-i-tell-you", c.api_key);
        assert_eq!(Some(666), c.default_probe);
    }

    #[test]
    fn test_load_nok() {
        let c = Config::load("/nonexistent");

        assert!(c.is_err());
    }

    #[test]
    fn test_reload_ok() {
        let mut c = Config::new();

        assert_eq!(Some(0), c.default_probe);

        let d = c.reload("src/bin/atlas/config.toml").unwrap();
        assert_eq!(Some(666), d.default_probe);
    }

    #[test]
    fn test_reload_nok() {
        let mut c = Config::new();

        assert_eq!(Some(0), c.default_probe);

        let d = c.reload("/nonexistent");
        assert!(d.is_err());
    }
}
