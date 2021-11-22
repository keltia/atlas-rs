//! Module to create a `Client` instance which is the main container for the API
//! This is the main client struct and the different configuration methods.
//!
//! The way to configure it is different from the Go way and more in line
//! with current Rust practices.
//!
//! The only mandatory argument is the API key so it is given to `new()` and
//! all the other methods are there for configuration everything you want to
//! change from the default.
//!
//! `NOTE` none of the fields are public except within the crate
//!
//! There are several classes of calls in the API:
//!
//! - entities
//! - protocols
//! - utility functions
//!
//! Each class is selected by a method on the `Client` struct such as `probe()` or `measurement()`.
//! Calling one of these methods sets up the context for further calls with `RequestBuilder`
//! (or plain `Request`).
//!
//! Examples:
//! ```no_run
//! use atlas_rs::client::{AF, ClientBuilder};
//!
//! let c = ClientBuilder::new()
//!             .api_key("FOO")
//!             .onoff(true)
//!             .default_probe(666)
//!             .want_af(AF::V4).build();
//!
//! let r = c.probe().get(666).call()?;
//! ```
//!
//! ```no_run
//! use atlas_rs::client::{AF, ClientBuilder};
//!
//! let c = ClientBuilder::new()
//!             .api_key("FOO")
//!             .onoff(true)
//!             .default_probe(666)
//!             .want_af(AF::V4)
//!             .build();
//!
//! let r = c.traceroute().to("next.example.net").call()?;
//! ```
//!
//! or
//!
//! ```no_run
//! use atlas_rs::client::Client;
//!
//! let c = Client::new();
//!
//! let r = c.traceroute().to("next.example.net").call()?;
//! ```
//!
//! like
//!
//! ```no_run
//! use atlas_rs::client::Client;
//!
//! let c = Client::new();
//!
//! let r = c.ntp().from("next.example.net").call()?;
//! ```
//!
//! (do we want a Client::NTP(next.example.net") or not?)
//!
//! Errors are handled in two steps:
//! 1. if there is a Transport error (Unknown Host, Unreachable, etc.) call() will return an error
//! 2. if the API returns an error, we attempt to decode as an APIError. If not, everything is good.
//!
//! So every `call()` returns a `Result<something, APIError>`.
//!
//! We use [reqwest] as HTTP client.  It has support for everything we need incl. proxy.  We choose
//! to use the blocking client as most of the time this ought to be enough and it is easier.
//!
//! [reqwest]: https://crates.io/reqwest/
//!

// Standard library
use std::collections::HashMap;
use std::time::Duration;

// External crates
use anyhow::{anyhow, Result};
use clap::{crate_name, crate_version};

// Internal crates
use crate::request::RequestBuilder;

// ---------------------------------------------------------------------------

/// We target the v2 API (not sure if it needs to be public)
pub(crate) const ENDPOINT: &str = "https://atlas.ripe.net/api/v2";

// ---------------------------------------------------------------------------

/// Represents all possible INET Address Family values
#[derive(Debug, PartialEq)]
pub enum AF {
    /// Only IPv4 target
    V4,
    /// Only IPv6 target
    V6,
    /// Both IPv4 & v6
    V46,
}

/// Represents the different categories aka first level of requests (probes, credits, etc.
#[derive(Debug)]
pub enum Cmd {
    Anchors,
    AnchorMeasurements,
    Credits,
    Keys,
    Measurements,
    ParticipationRequests,
    Probes,
}

// ---------------------------------------------------------------------------

/// This is the main `Client` struct.  It holds all the parameters and the HTTP client handle.
/// When using `Client::new()`, you get all the defaults values, if you want to configure it,
/// please use `ClientBuilder` instead.
///
/// Example:
/// ```no_run
/// # use atlas_rs::client::Client;
///
/// let c = Client::new();
/// ```
#[derive(Debug)]
pub struct Client<'cl> {
    /// Mandatory
    pub(crate) api_key: Option<&'cl str>,

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

    /// Internal state, http client
    pub(crate) agent: Option<reqwest::blocking::Client>,
}

/// Default values for Client
///
impl<'cl> Default for Client<'cl> {
    /// Defines all the default values
    fn default() -> Self {
        Client::new()
    }
}

/// All methods for `Client` for configuration
///
impl<'cl> Client<'cl> {
    // ---------------------------------------------------------------------
    // Public API

    /// Creates a bare client with defaults except for the API key which limits to certain
    /// RIPE Atlas calls.
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use atlas_rs::client::Client;
    ///
    /// let c = Client::new();
    /// ```
    ///
    pub fn new() -> Client<'cl> {
        Client {
            api_key: None,
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
            agent: None,
        }
        .httpclient()
    }

    /// Create a ClientBuilder struct and returns it for chained calls
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use atlas_rs::client::ClientBuilder;
    ///
    /// let c = ClientBuilder::builder();
    /// ```
    ///
    pub fn builder() -> ClientBuilder<'cl> {
        ClientBuilder::new()
    }

    // ---------------------------------------------------------------------
    // Entities
    pub fn anchors(&self) -> RequestBuilder { unimplemented!() }

    pub fn credits(&self) -> RequestBuilder {
        unimplemented!()
    }

    pub fn keys(&self) -> RequestBuilder {
        unimplemented!()
    }

    pub fn measurement(&self) -> RequestBuilder {
        unimplemented!()
    }

    pub fn probe(self) -> RequestBuilder<'cl> {
        let url = reqwest::Url::parse(self.endpoint).unwrap();
        let r = reqwest::blocking::Request::new(
            reqwest::Method::GET,
            url,
        );
        RequestBuilder {ctx: Cmd::Probes, c: self, r: r}
    }

    // ---------------------------------------------------------------------
    // Protocols

    pub fn dns(&self) -> RequestBuilder {
        unimplemented!()
    }

    pub fn http(&self) -> RequestBuilder {
        unimplemented!()
    }

    pub fn ntp(&self) -> RequestBuilder {
        unimplemented!()
    }

    pub fn ping(&self) -> RequestBuilder {
        unimplemented!()
    }

    pub fn tlscert(&self) -> RequestBuilder {
        unimplemented!()
    }

    pub fn traceroute(&self) -> RequestBuilder {
        unimplemented!()
    }

    // ---------------------------------------------------------------------
    // Helpers/shortcuts

    // *placeholder*

    // ---------------------------------------------------------------------
    // Private functions

    /// Create an instance of the HTTP client and attach it there.  It is called as part of
    /// .build().
    ///
    fn httpclient(mut self) -> Self {
        let ag = format!("{}/{}", crate_name!(), crate_version!());
        let agent = reqwest::blocking::ClientBuilder::new()
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(5))
            .user_agent(&ag)
            .build()
            .unwrap();
        self.agent = Some(agent);
        self
    }
}

// ---------------------------------------------------------------------------

/// `ClientBuilder` is the main struct to create and configure a `Client`
///
pub struct ClientBuilder<'cl> {
    cl: Client<'cl>,
}

/// Default values for `ClientBuilder`
///
impl<'cl> Default for ClientBuilder<'cl> {
    /// Defines all the default values
    fn default() -> Self {
        ClientBuilder::new()
    }
}

/// Methods for `ClientBuilder`
///
impl<'cl> ClientBuilder<'cl> {
    // ---------------------------------------------------------------------
    // Public API

    /// Create a new `ClientBuilder` instance
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use atlas_rs::client::ClientBuilder;
    /// let c = ClientBuilder::new();
    /// ```
    ///
    pub fn new() -> Self {
        ClientBuilder { cl: Client::new() }
    }

    /// Create the final Client after checking the API key has been changed
    ///
    pub fn build(self) -> Result<Client<'cl>> {
        match self.cl.api_key {
            None => Err(anyhow!("You must change the default key")),
            Some(_k) => Ok(self.cl),
        }
    }

    /// Set the API key
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use atlas_rs::client::ClientBuilder;
    ///
    /// let c = ClientBuilder::new()
    ///         .api_key("FOO");
    /// ```
    ///
    pub fn api_key<S: Into<&'cl str>>(mut self, key: S) -> Self {
        self.cl.api_key = Some(key.into());
        self
    }

    /// Sets the API endpoint
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use atlas_rs::client::ClientBuilder;
    ///
    /// let c = ClientBuilder::new()
    ///     .endpoint("https://example.com/v1")
    /// # ;
    /// ```
    ///
    pub fn endpoint<S: Into<&'cl str>>(mut self, v: S) -> Self {
        self.cl.endpoint = v.into();
        self
    }

    /// Sets the default probe ID
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use atlas_rs::client::ClientBuilder;
    ///
    /// let c = ClientBuilder::new()
    ///     .default_probe(666)
    /// # ;
    /// ```
    ///
    pub fn default_probe(mut self, v: u32) -> Self {
        self.cl.default_probe = v;
        self
    }

    /// Limits the scope by specifying an area type
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use atlas_rs::client::ClientBuilder;
    ///
    /// let c = ClientBuilder::new()
    ///     .area_type("area")
    /// # ;
    /// ```
    ///
    pub fn area_type<S: Into<&'cl str>>(mut self, v: S) -> Self {
        self.cl.area_type = v.into();
        self
    }

    /// Limits the scope to this particular area value
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use atlas_rs::client::ClientBuilder;
    ///
    /// let c = ClientBuilder::new()
    ///     .area_value("WW")
    /// # ;
    /// ```
    ///
    pub fn area_value<S: Into<&'cl str>>(mut self, v: S) -> Self {
        self.cl.area_value = v.into();
        self
    }

    /// Sets the one-shot flag
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use atlas_rs::client::ClientBuilder;
    ///
    /// let c = ClientBuilder::new()
    ///     .onoff(true)
    /// # ;
    /// ```
    ///
    pub fn onoff(mut self, v: bool) -> Self {
        self.cl.is_oneoff = v;
        self
    }

    /// Sets the pool size
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use atlas_rs::client::ClientBuilder;
    ///
    /// let c = ClientBuilder::new()
    ///     .pool_size(20)
    /// # ;
    /// ```
    ///
    pub fn pool_size(mut self, v: usize) -> Self {
        self.cl.pool_size = v;
        self
    }

    /// Sets the verbose flag
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use atlas_rs::client::ClientBuilder;
    ///
    /// let c = ClientBuilder::new()
    ///     .verbose(true)
    /// # ;
    /// ```
    ///
    pub fn verbose(mut self, v: bool) -> Self {
        self.cl.verbose = v;
        self
    }

    /// Sets the inet family, either v4 or v6 or both.
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use atlas_rs::client::ClientBuilder;
    ///
    /// let c = ClientBuilder::new()
    ///     .want_af(AF::V6)
    /// # ;
    /// ```
    ///
    pub fn want_af(mut self, v: AF) -> Self {
        self.cl.want_af = v;
        self
    }

    /// Sets the tags to be sent with the requests
    ///    +tag / tag  ==> tags_include
    ///    -tag / !tag ==> tags_exclude
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use atlas_rs::client::ClientBuilder;
    ///
    /// let c = ClientBuilder::new()
    ///     .tags("ftth !cable")
    /// # ;
    /// ```
    ///
    pub fn tags<S: Into<&'cl str>>(mut self, v: S) -> Self {
        self.cl.tags = v.into();
        self
    }

    /// Add options
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use std::collections::HashMap;
    /// # use atlas_rs::client::ClientBuilder;
    ///
    /// let c = ClientBuilder::new()
    ///     .opts(&HashMap::from([
    ///                        ("is_anchor", "true")
    ///                    ]))
    /// # ;
    /// ```
    ///
    pub fn opts(mut self, opts: &HashMap<&'cl str, &'cl str>) -> Self {
        for (k, v) in opts.iter() {
            self.cl.opts.insert(k, v);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_new() {
        let c = Client::new("FOO");

        // Check all defaults
        assert_eq!("FOO", c.api_key);
        assert_eq!(ENDPOINT, c.endpoint);
        assert_eq!(0, c.default_probe);
        assert_eq!("area", c.area_type);
        assert_eq!("WW", c.area_value);
        assert!(c.is_oneoff);
        assert_eq!(10, c.pool_size);
        assert_eq!(AF::V46, c.want_af);
        assert!(!c.verbose);
        assert_eq!("", c.tags);
        assert!(c.opts.contains_key("key"));
        assert!(c.agent.is_some());
    }

    #[test]
    fn test_clientbuilder_new() {
        let cb = ClientBuilder::new().api_key("key").build();

        assert!(cb.is_ok());

        let c = c.unwrap();
        // Check all defaults
        assert_eq!("key", c.cl.api_key);
        assert_eq!(ENDPOINT, c.cl.endpoint);
        assert_eq!(0, c.cl.default_probe);
        assert_eq!("area", c.cl.area_type);
        assert_eq!("WW", c.cl.area_value);
        assert!(c.cl.is_oneoff);
        assert_eq!(10, c.cl.pool_size);
        assert_eq!(AF::V46, c.cl.want_af);
        assert!(!c.cl.verbose);
        assert_eq!("", c.cl.tags);
        assert!(c.cl.opts.contains_key("key"));
        assert!(c.cl.agent.is_some());
    }

    #[test]
    fn test_opts() {
        let h = HashMap::from([("foo", "a"), ("bar", "b"), ("key", "FOO")]);
        let c = ClientBuilder::new().opts(&h).build().unwrap();
        assert_eq!(h, c.opts);
    }

    #[test]
    fn test_onoff() {
        let c = ClientBuilder::new().onoff(true).build();

        assert!(c.is_oneoff);
    }
}
