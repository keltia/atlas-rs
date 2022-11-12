//! This is the module to handle configuration for the `atlas` client.  Upon calling
//! `Config::new()` the struct contain reasonable defaults (and a bad API key that
//! you must change).
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
//! On Unix systems (FreeBSD, macOS, Linux, etc.) the default configuration
//! directory is `$HOME/.config/atlas-rs/` whereas on Windows, it is located
//! in `%LOCALAPPDATA%\atlas-rs\`.
//!
//! Examples:
//! ```
//! use crate::config::Config;
//!
//! let cfg = Config::new();  // will contain the defaults values from here.
//!
//! println!("Default key is {}", cfg.api_key);
//! ```
//!
//! or
//!
//! ```
//! use crate::config::Config;
//!
//! let cfg = Config::load("./atlas.toml").unwrap();
//!
//! println!("Default key is {}", cfg.api_key);
//! ```
//!
//! [TOML]: https://crates.io/crates/toml

// Standard library
use std::env;
use std::fs;
use std::path::PathBuf;

// External crates
use anyhow::Result;
use clap::crate_name;
use serde::Deserialize;

use atlas_api::makepath;

#[cfg(unix)]
use home::home_dir;

/// Default configuration filename
const CONFIG: &str = "config.toml";

/// Use the standard location `$HOME/.config`
#[cfg(unix)]
const BASEDIR: &str = ".config";

/// Default set of probes to be used for queries
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub(crate) struct ProbeSet {
    /// How many probes do we want
    pub(crate) pool_size: Option<usize>,
    /// Probe type
    pub(crate) ptype: Option<String>,
    /// Value for probe type
    pub(crate) value: Option<String>,
    /// Include/exclude specific tags
    pub(crate) tags: Option<String>,
}

/// If we want to bill the queries to a specific account (i.e. different from the
/// one behind the API key).
///
/// NOTE: I never used it but it is part of the API.
///
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub(crate) struct Measurements {
    /// RIPE Account ID to be billed for subsequent queries
    pub(crate) bill_to: String,
}

/// `Config` struct with one mandatory argument and optional ones.
///
/// Most API calls need an API key.
///
#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    /// API key
    pub(crate) api_key: String,
    /// Default probe ID
    pub(crate) default_probe: Option<u32>,
    /// Default set of probes
    pub(crate) probe_set: Option<ProbeSet>,
    /// Stuff about billing to a specific account
    pub(crate) measurements: Option<Measurements>,
}

/// Here are the "reasonable" defaults.
///
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
///
impl Config {
    /// Create a `Config` struct with default values.
    ///
    /// Example:
    /// ```
    /// # use atlas_rs::config::Config;
    ///
    /// let cfg = Config::new();
    /// ```
    ///
    pub(crate) fn new() -> Config {
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
    /// let cfg = Config::load("./atlas.conf");
    /// ```
    ///
    pub(crate) fn load(fname: &PathBuf) -> Result<Self> {
        let content = fs::read_to_string(fname)?;
        dbg!(&content);
        Ok(toml::from_str(&content)?)
    }
}

/// Returns the path of the default config file. On Unix systems we use the standard `$HOME/.config`
/// base directory.
///
#[cfg(unix)]
pub(crate) fn default_file() -> Result<PathBuf> {
    let homedir = home_dir()?;

    Ok(makepath!(homedir,, BASEDIR, crate_name!(), CONFIG))
}

/// Returns the path of the default config file.  Here we use the standard %LOCALAPPDATA%
/// variable to base our directory into.
///
#[cfg(windows)]
pub(crate) fn default_file() -> Result<PathBuf> {
    let basedir = env::var("LOCALAPPDATA")?;

    Ok(makepath!(basedir, crate_name!(), CONFIG))
}

#[cfg(test)]
mod tests {
    use super::*;
    use atlas_api::makepath;

    #[test]
    fn test_new() {
        let c = Config::new();

        assert_eq!("<CHANGEME>", c.api_key);
        assert_eq!(Some(0), c.default_probe);
    }

    #[test]
    fn test_load_ok() {
        let h: PathBuf = makepath!("src", CONFIG);
        let c = Config::load(&h).unwrap();

        assert_eq!("no-way-i-tell-you", c.api_key);
        assert_eq!(Some(666), c.default_probe);
    }

    #[test]
    fn test_load_nok() {
        let c = Config::load(&PathBuf::from("/nonexistent"));

        assert!(c.is_err());
    }

    #[test]
    #[cfg(unix)]
    fn test_default_file() -> Result<()> {
        let h = env::var("HOME")?;
        let h: PathBuf = makepath!(h, BASEDIR, crate_name!(), CONFIG);

        assert_eq!(h, default_file().unwrap());
        Ok(())
    }

    #[test]
    #[cfg(windows)]
    fn test_default_file() -> Result<()> {
        let h = env::var("LOCALAPPDATA")?;
        let h: PathBuf = makepath!(h, crate_name!(), CONFIG);

        assert_eq!(h, default_file().unwrap());
        Ok(())
    }
}
