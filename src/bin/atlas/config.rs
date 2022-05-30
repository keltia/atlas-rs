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
//! use atlas_rs::config::Config;
//!
//! let cfg = Config::new();  // will contain the defaults values from here.
//!
//! println!("Default key is {}", cfg.api_key);
//! ```
//!
//! or
//!
//! ```
//! use atlas_rs::config::Config;
//!
//! let cfg = Config::load("./atlas.toml").unwrap();
//!
//! println!("Default key is {}", cfg.api_key);
//! ```
//!
//! There is also a `reload()` method to override an existing configuration.
//!
//! Example:
//!
//! ```
//! use atlas_rs::config::Config;
//!
//! let cfg = Config::load("./atlas.toml").unwrap();
//!
//! let cfg = cfg.reload("new.toml").unwrap();
//!
//! println!("Key is now {}", cfg.api_key);
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
#[cfg(unix)]
use home::home_dir;
use serde::Deserialize;

/// Default configuration filename
const CONFIG: &str = "config.toml";

/// Use the standard location `$HOME/.config`
#[cfg(unix)]
const BASEDIR: &str = ".config";

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
///
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Measurements {
    /// RIPE Account ID to be billed for subsequent queries
    pub bill_to: String,
}

/// `Config` struct with one mandatory argument and optional ones.
///
/// Most API calls need an API key.
///
#[derive(Clone, Debug, Deserialize)]
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
    /// let cfg = Config::load("./atlas.conf");
    /// ```
    ///
    pub fn load(fname: &PathBuf) -> Result<Self> {
        let content = fs::read_to_string(fname)?;
        //println!("{:?}", content);
        Ok(toml::from_str(&content)?)
    }
}

/// Returns the path of the default config file. On Unix systems we use the standard `$HOME/.config`
/// base directory.
///
#[cfg(unix)]
pub fn default_file() -> Result<PathBuf> {
    let homedir = home_dir().unwrap();

    let def: PathBuf = [
        homedir,
        PathBuf::from(BASEDIR),
        PathBuf::from(crate_name!()),
        PathBuf::from(CONFIG),
    ]
    .iter()
    .collect();
    Ok(def)
}

/// Returns the path of the default config file.  Here we use the standard %LOCALAPPDATA%
/// variable to base our directory into.
///
#[cfg(windows)]
pub fn default_file() -> Result<PathBuf> {
    let basedir = env::var("LOCALAPPDATA")?;

    let def: PathBuf = [
        PathBuf::from(basedir),
        PathBuf::from(crate_name!()),
        PathBuf::from(CONFIG),
    ]
    .iter()
    .collect();
    Ok(def)
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
        let c = Config::load(&PathBuf::from("src/bin/atlas/config.toml")).unwrap();

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
        let h = h + "/.config/atlas-rs/config.toml";
        let h = PathBuf::from(h);

        assert_eq!(h, default_file().unwrap());
        Ok(())
    }

    #[test]
    #[cfg(windows)]
    fn test_default_file() -> Result<()> {
        let h = env::var("LOCALAPPDATA")?;
        let h: PathBuf = [
            PathBuf::from(h),
            PathBuf::from(crate_name!()),
            PathBuf::from(CONFIG),
        ]
        .iter()
        .collect();

        assert_eq!(h, default_file().unwrap());
        Ok(())
    }
}
