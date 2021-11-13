//! This is the module to handle configuration for the `atlas` client.
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
//! There is also a `reload()` method to override an existing configuration.
//!
//! Example:
//!
//! ```rs
//!  use atlas_rs::config::Config;
//!
//!  let cfg = Config::load("./atlas.toml").unwrap();
//!
//!  let cfg = cfg.reload("new.toml").unwrap();
//!
//!  println!("Key is now {}", cfg.api_key);
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

#[cfg(unix)]
use home::home_dir;

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
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Measurements {
    /// RIPE Account ID to be billed for subsequent queries
    pub bill_to: String,
}

/// `Config` struct with one mandatory argument and optional ones.
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
    ///
    /// Example:
    /// ```rs
    ///  # use atlas_rs::config::Config;
    ///
    ///  let cfg = Config::new();
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
    ///   let cfg = Config::load("./atlas.conf");
    /// ```
    ///
    pub fn load(fname: &str) -> Result<Self> {
        let content = fs::read_to_string(fname)?;
        println!("{:?}", content);
        Ok(toml::from_str(&content)?)
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
    pub fn reload(&mut self, fname: &str) -> Result<&mut Self> {
        let val = Config::load(fname)?;

        // copy non-null values
        if val.api_key != *"" {
            self.api_key = val.api_key;
        }

        if val.default_probe != None {
            self.default_probe = val.default_probe;
        }

        if val.probe_set != None {
            self.probe_set = val.probe_set;
        }

        if val.measurements != None {
            self.measurements = val.measurements;
        }

        Ok(self)
    }
}

/// Returns the path of the default config file. On Unix systems we use the standard `$HOME/.config`
/// base directory.
#[cfg(unix)]
pub fn default_file() -> Result<String> {
    let homedir = home_dir().unwrap();

    let def: PathBuf = [
        homedir,
        PathBuf::from(BASEDIR),
        PathBuf::from(crate_name!()),
        PathBuf::from(CONFIG),
    ]
    .iter()
    .collect();
    Ok(def.to_str().unwrap().to_string())
}

/// Returns the path of the default config file.  Here we use the standard %LOCALAPPDATA%
/// variable to base our directory into.
#[cfg(windows)]
pub fn default_file() -> Result<String> {
    let basedir = env::var("LOCALAPPDATA")?;

    let def: PathBuf = [
        PathBuf::from(basedir),
        PathBuf::from(crate_name!()),
        PathBuf::from(CONFIG),
    ]
    .iter()
    .collect();
    let def = def.to_str().unwrap().to_string();
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

    #[test]
    #[cfg(unix)]
    fn test_default_file() -> Result<()> {
        let h = env::var("HOME")?;
        let h = h + "/.config/atlas-rs/config.toml";
        let h = PathBuf::from(h);

        let sh = h.to_str().unwrap().to_string();

        assert_eq!(sh, default_file().unwrap());
        Ok(())
    }

    #[test]
    #[cfg(windows)]
    fn test_default_file() -> Result<()> {
        let h = env::var("LOCALAPPDATA")?;
        let h: PathBuf = [
            PathBuf::from(h),
            PathBuf::from("atlas-rs"),
            PathBuf::from(CONFIG),
        ]
        .iter()
        .collect();

        let sh = h.to_str().unwrap().to_string();

        assert_eq!(sh, default_file().unwrap());
        Ok(())
    }
}
