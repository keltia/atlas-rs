//! This is the module to handle configuration for the `atlas` client.  Upon calling
//! `Config::new()` the struct contains reasonable defaults (and a bad API key that
//! you *must* change).
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
//! directory is `$HOME/.config/ripe-atlas/` whereas on Windows, it is located
//! in `%LOCALAPPDATA%\ripe-atlas\`.  We share the directory with the [Go version]
//! of this API.
//!
//! Examples:
//! ```
//! use atlas_api::config::Config;
//!
//! let cfg = Config::new();  // will contain the defaults values from here.
//!
//! println!("Default key is {}", cfg.api_key);
//! ```
//!
//! or
//!
//! ```
//! use atlas_api::config::Config;
//!
//! let cfg = Config::load("./atlas.toml").unwrap();
//!
//! println!("Default key is {}", cfg.api_key);
//! ```
//!
//! [TOML]: https://crates.io/crates/toml
//! [Go Version]: https://github.com/keltia/ripe-atlas/

// Standard library
use std::fs;
use std::path::{Path, PathBuf};

// External crates
use directories::BaseDirs;
use eyre::Result;
use log::{debug, error};
use serde::Deserialize;

/// Default configuration filename
const CONFIG: &str = "config.toml";

/// Package name for the configuration
const CONF_NAME: &str = "ripe-atlas";

/// Use the standard location `$HOME/.config`
#[cfg(unix)]
const BASEDIR: &str = ".config";

/// Represents the configuration for a set of probes used in the `atlas` client.
///
/// ### Fields:
///
/// - `pool_size`: An optional `usize` specifying how many probes to include.
/// - `ptype`: An optional `String` representing the type of the probe (e.g., "area").
/// - `value`: An optional `String` specifying the value for the probe type (e.g., "WW").
/// - `tags`: An optional `String` containing tags to include or exclude specific probe features.
///
/// ### Example:
///
/// ```toml
/// [probe_set]
///
/// pool_size = 42
/// type = "area"
/// value = "WW"
/// tags = "+ipv4"
/// ```
///
/// This struct is part of the [`Config`] and can be used to customize the probe configuration
/// for the `atlas` client.
///
/// ### See also:
/// - [`Config`] for the main configuration structure.
/// - [RIPE Atlas API Documentation](https://atlas.ripe.net/) for more details on probe configurations.
///
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
/// NOTE: I never used it, but it is part of the API.
///
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub(crate) struct Measurements {
    /// RIPE Account ID to be billed for subsequent queries
    pub(crate) bill_to: String,
}

/// The `Config` struct contains configuration options for the `atlas` client,
/// which includes options like API keys, default probes, and billing settings.
///
/// ### Fields:
///
/// - `api_key`: A `String` containing the API key required for most API calls.
/// - `default_probe`: An optional `u32` representing the default probe ID.
/// - `probe_set`: An optional `ProbeSet` struct containing details about the probe configuration.
/// - `measurements`: An optional `Measurements` struct for specifying billing-related settings.
///
/// ### Example:
///
/// ```no_run
/// use atlas_api::config::Config;
///
/// let cfg = Config::new();  // Initializes with default values.
/// println!("Default API key is {}", cfg.api_key);
///
/// let loaded_cfg = Config::load(&PathBuf::from("./config.toml")).unwrap();
/// println!("Loaded API key is {}", loaded_cfg.api_key);
/// ```
///
/// This struct also provides default values via the `Default` trait and utility methods
/// such as `new()` and `load()` for creating and loading configuration.
///
/// It supports the following TOML format for configurations:
///
/// ```toml
/// api_key = "your_api_key"
/// default_probe = 123
///
/// [probe_set]
///
/// pool_size = 10
/// type = "area"
/// value = "WW"
/// tags = "+ipv4"
/// ```
///
/// ### See also:
/// - [`ProbeSet`] for details on configuring probe pools.
/// - [`Measurements`] for billing-related options.
///
#[allow(dead_code)]
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
    /// # use atlas_api::config::Config;
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
        Ok(toml::from_str(&content)?)
    }
}

/// Returns the path of the default config file. On Unix systems we use the standard `$HOME/.config`
/// base directory.
///
pub(crate) fn default_file() -> Result<PathBuf> {
    let base = BaseDirs::new();
    let basedir = match base {
        Some(base) => {
            #[cfg(unix)]
            let base = base.home_dir().join(".config");

            #[cfg(windows)]
            let base = base.data_local_dir();

            debug!("base = {base:?}");
            base.join(Path::new(CONF_NAME))
        }
        None => {
            #[cfg(unix)]
            let homedir = std::env::var("HOME")
                .map_err(|_| error!("No HOME variable defined, can not continue"))
                .unwrap();

            #[cfg(windows)]
            let homedir = std::env::var("LOCALAPPDATA")
                .map_err(|_| error!("No LOCALAPPDATA variable defined, can't continue"))
                .unwrap();

            debug!("base = {homedir}");

            #[cfg(unix)]
            let base = Path::new(&homedir)
                .join(Path::new(".config"))
                .join(Path::new(CONF_NAME));

            #[cfg(windows)]
            let base = PathBuf::from(homedir).join(CONF_NAME);

            base
        }
    };

    let fname = basedir.join(CONFIG);
    Ok(fname)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_config_new() {
        let c = Config::new();

        assert_eq!("<CHANGEME>", c.api_key);
        assert_eq!(Some(0), c.default_probe);
    }

    #[test]
    fn test_config_load_ok() {
        let c = Config::load(&PathBuf::from("src/config.toml")).unwrap();

        assert_eq!("no-way-i-tell-you", c.api_key);
        assert_eq!(Some(666), c.default_probe);
    }

    #[test]
    fn test_config_load_nok() {
        let c = Config::load(&PathBuf::from("/nonexistent"));

        assert!(c.is_err());
    }

    #[test]
    #[cfg(unix)]
    fn test_config_default_file() -> Result<()> {
        let h = env::var("HOME")?;
        let h = Path::new(&h).join(".config").join(CONF_NAME).join(CONFIG);

        assert_eq!(h, default_file()?);
        Ok(())
    }

    #[test]
    #[cfg(windows)]
    fn test_default_file() -> Result<()> {
        let h = env::var("LOCALAPPDATA")?;
        let h: PathBuf = Path::new(&h).join(CONF_NAME).join(CONFIG);

        assert_eq!(h, default_file().unwrap());
        Ok(())
    }
}
