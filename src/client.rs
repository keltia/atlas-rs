//! This is the main client struct and the different configuration methods.
//!
//! The way to configure it is different from the Go way and more in line
//! current Rust practices.
//!
//! The only mandatory argument is the API key so it is given to `new()` and
//! all the other methods are there for configuration everything you want to
//! change from the default.
//!
//! Examples:
//!
//! ```
//! use atlas_rs::client::{AF,Client};
//!
//! let c = Client::new("FOOBAR")
//!     .onoff(true)
//!     .default_probe(666)
//!     .want_af(AF::V4);
//!
//!

use std::collections::HashMap;

/// We target the v2 API
const ENDPOINT: &str = "https://atlas.ripe.net/api/v2";

#[derive(Debug, PartialEq)]
pub enum AF {
    V4,
    V6,
    V46,
}

/// Hold the client data
#[derive(Debug)]
pub struct Client<'cl> {
    /// Mandatory
    pub(crate) api_key: &'cl str,

    /// Optional
    pub(crate) endpoint: &'cl str,
    pub(crate) default_probe: u32,
    pub(crate) area_type: &'cl str,
    pub(crate) area_value: &'cl str,
    pub(crate) is_oneoff: bool,
    pub(crate) pool_size: usize,
    pub(crate) want_af: AF,
    pub(crate) verbose: bool,
    pub(crate) tags: &'cl str,

    /// Default options
    pub(crate) opts: HashMap<&'cl str, &'cl str>,
}

/// Default values
impl<'cl> Default for Client<'cl> {
    /// Defines all the default values
    fn default() -> Self {
        Client {
            api_key: "<CHANGEME>",
            endpoint: ENDPOINT,
            default_probe: 0,
            area_type: "area",
            area_value: "WW",
            is_oneoff: true,
            pool_size: 10,
            want_af: AF::V46,
            verbose: false,
            tags: "",
            opts: HashMap::new(),
        }
    }
}

/// All methods for`Client` for configuration
impl<'cl> Client<'cl> {
    /// Create a new `Client` instance wit hthe specified key
    ///
    /// Examples
    ///
    /// ```no_run
    /// # use atlas_rs::client::Client;
    /// let c = Client::new("FOO");
    /// ```
    pub fn new<S: Into<&'cl str>>(key: S) -> Self {
        Client {
            api_key: key.into(),
            ..Default::default()
        }
    }

    /// Sets the API endpoint
    ///
    /// Examples
    ///
    /// ```no_run
    /// # use atlas_rs::client::Client;
    /// Client::new("FOO")
    ///     .endpoint("https://example.com/v1")
    /// # ;
    /// ```
    pub fn endpoint<S: Into<&'cl str>>(mut self, v: S) ->  Self {
        self.endpoint = v.into();
        self
    }

    /// Sets the API endpoint
    ///
    /// Examples
    ///
    /// ```no_run
    /// # use atlas_rs::client::Client;
    /// Client::new("FOO")
    ///     .endpoint("https://example.com/v1")
    /// # ;
    /// ```
    pub fn default_probe(mut self, v: u32) -> Self {
        self.default_probe = v;
        self
    }

    /// Limits the scope by specifying an area type
    ///
    /// Examples
    ///
    /// ```no_run
    /// # use atlas_rs::client::Client;
    /// Client::new("FOO")
    ///     .area_type("area")
    /// # ;
    /// ```
    pub fn area_type<S: Into<&'cl str>>(mut self, v: S) -> Self {
        self.area_type = v.into();
        self
    }

    /// Limits the scope to this particular area value
    ///
    /// Examples
    ///
    /// ```no_run
    /// # use atlas_rs::client::Client;
    /// Client::new("FOO")
    ///     .area_value("WW")
    /// # ;
    /// ```
    pub fn area_value<S: Into<&'cl str>>(mut self, v: S) -> Self {
        self.area_value = v.into();
        self
    }

    /// Sets the one-shot flag
    ///
    /// Examples
    ///
    /// ```no_run
    /// # use atlas_rs::client::Client;
    /// Client::new("FOO")
    ///     .onoff(true)
    /// # ;
    /// ```
    pub fn onoff(mut self, v: bool) -> Self {
        self.is_oneoff = v;
        self
    }

    /// Sets the pool size
    ///
    /// Examples
    ///
    /// ```no_run
    /// # use atlas_rs::client::Client;
    /// Client::new("FOO")
    ///     .pool_size(20)
    /// # ;
    /// ```
    pub fn pool_size(mut self, v: usize) -> Self {
        self.pool_size = v;
        self
    }

    /// Sets the verbose flag
    ///
    /// Examples
    ///
    /// ```no_run
    /// # use atlas_rs::client::Client;
    /// Client::new("FOO")
    ///     .verbose(true)
    /// # ;
    /// ```
    pub fn verbose(mut self, v: bool) -> Self {
        self.verbose = v;
        self
    }

    /// Sets the inet family, either v4 or v6 or both.
    ///
    /// Examples
    ///
    /// ```no_run
    /// # use atlas_rs::client::{AF,Client};
    /// Client::new("FOO")
    ///     .want_af(AF::V6)
    /// # ;
    /// ```
    pub fn want_af(mut self, v: AF) -> Self {
        self.want_af = v;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let c = Client::new("FOO");

        assert_eq!("FOO", c.api_key);
        assert_eq!(ENDPOINT, c.endpoint);
        assert!(c.is_oneoff);
        assert_eq!(AF::V46, c.want_af)
    }

    #[test]
    fn test_api_key() {
        let c= Client::new("FOO")
            .onoff(true);
        assert_eq!("FOO", c.api_key);
        assert_eq!(ENDPOINT, c.endpoint);
        assert!(c.is_oneoff);
        println!("{:#?}", c);
    }
}