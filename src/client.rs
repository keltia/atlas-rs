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
//! Errors are handled in two steps:
//! 1. if there is a Transport error (Unknown Host, Unreachable, etc.) call() will return an error
//! 2. if the API returns an error, we attempt to decode as an APIError. If not, everything is good.
//!
//! So every `get/info/list()` returns a `Result<something, APIError>`.
//!
//! We use [reqwest] as HTTP client.  It has support for everything we need incl. proxy.  We choose
//! to use the blocking client as most of the time this ought to be enough and it is easier.
//!
//! [reqwest]: https://crates.io/reqwest/
//!

// Standard library
use std::time::Duration;

// External crates
use anyhow::{anyhow, Result};
use clap::{crate_name, crate_version};
use reqwest::Url;

// Internal crates
use crate::option::Options;
use crate::request::RequestBuilder;

// ---------------------------------------------------------------------------

/// We target the v2 API (not sure if it needs to be public)
const ENDPOINT: &str = "https://atlas.ripe.net/api/v2";

// ---------------------------------------------------------------------------

/// Represents all possible INET Address Family values
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AF {
    /// Only IPv4 target
    V4,
    /// Only IPv6 target
    V6,
    /// Both IPv4 & v6
    V46,
}

/// Represents the different categories aka first level of requests (probes, credits, etc.
#[derive(Clone, Copy, Debug)]
pub enum Ctx {
    None = 0,
    Anchors,
    AnchorMeasurements,
    Credits,
    Keys,
    Measurements,
    ParticipationRequests,
    Probes,
}

impl Default for Ctx {
    fn default() -> Self {
        Ctx::None
    }
}

// ---------------------------------------------------------------------------

/// This is the main `Client` struct.  It holds all the parameters and the HTTP client handle.
/// When using `Client::new()`, you get all the defaults values, if you want to configure it,
/// please use `ClientBuilder` instead.
///
/// Examples:
/// ```no_run
/// # fn main() -> Result<(), atlas_rs::errors::APIError> {
/// use atlas_rs::client::Client;
/// use atlas_rs::core::probes::Probe;
///
/// let c = Client::new();
///
/// let p: Probe = c.probe().get(666)?;
/// # Ok(())
/// # }
/// ```
/// or
/// ```no_run
/// # fn main() -> Result<(), atlas_rs::errors::APIError> {
/// use atlas_rs::client::Client;
/// use atlas_rs::core::credits::Credits;
///
/// let c = Client::new();
///
/// let r: Credits = c.credits().info()?;
/// # Ok(())
/// # }
/// ```
///
#[derive(Clone, Debug)]
pub struct Client {
    /// API Key for most calls
    pub(crate) api_key: Option<String>,
    /// API endpoint, overriden during tests
    pub(crate) endpoint: Url,
    /// Area type for restricting the scope
    pub(crate) area_type: String,
    /// Value for the area type
    pub(crate) area_value: String,
    /// Recurring or not?
    pub(crate) is_oneoff: bool,
    /// How many probes do we want
    pub(crate) pool_size: usize,
    /// IPv4, IPv6, both?
    pub(crate) want_af: AF,
    /// Verbose mode
    pub(crate) verbose: bool,
    /// Do we want specific probes types?
    pub(crate) tags: String,
    /// Default options
    pub(crate) opts: Options,
    /// Internal state, http client
    pub(crate) agent: Option<reqwest::blocking::Client>,
}

impl Default for Client {
    /// Defines all the default values for Client
    fn default() -> Self {
        Client::new()
    }
}

impl Client {
    /// Default connect timeout
    const DEFAULT_CONNECT_TIMEOUT: u64 = 10;

    /// Default HTTP timeout
    const DEFAULT_HTTP_TIMEOUT: u64 = 5;

    /// Default poolsize
    const DEFAULT_POOLSIZE: usize = 10;

    // ---------------------------------------------------------------------
    // Public API

    /// Creates a bare client with defaults
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use atlas_rs::client::Client;
    ///
    /// let c = Client::new();
    /// ```
    ///
    pub fn new() -> Self {
        let endp = reqwest::Url::parse(ENDPOINT).unwrap();
        Client {
            api_key: None,
            endpoint: endp,
            area_type: "area".to_string(),
            area_value: "WW".to_string(),
            is_oneoff: true,
            pool_size: Client::DEFAULT_POOLSIZE,
            want_af: AF::V46,
            verbose: false,
            tags: "".to_string(),
            opts: Options::new(),
            agent: None,
        }
        .httpclient()
    }

    /// Create a ClientBuilder struct and returns it for chained calls
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use atlas_rs::client::Client;
    ///
    /// let c = Client::builder();
    /// ```
    ///
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    // ---------------------------------------------------------------------
    // Entities
    //
    #[inline]
    pub fn anchors(&self) -> RequestBuilder {
        self.route_to(Ctx::Anchors)
    }

    #[inline]
    pub fn anchor_measurement(&self) -> RequestBuilder {
        self.route_to(Ctx::AnchorMeasurements)
    }

    #[inline]
    pub fn credits(&self) -> RequestBuilder {
        self.route_to(Ctx::Credits)
    }

    #[inline]
    pub fn keys(&self) -> RequestBuilder {
        self.route_to(Ctx::Keys)
    }

    #[inline]
    pub fn measurement(&self) -> RequestBuilder {
        unimplemented!()
    }

    #[inline]
    pub fn probe(&self) -> RequestBuilder {
        self.route_to(Ctx::Probes)
    }

    // ---------------------------------------------------------------------
    // Protocols
    //
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
    /// `.build()`.
    ///
    fn httpclient(mut self) -> Self {
        let ag = format!("{}/{}", crate_name!(), crate_version!());
        let agent = reqwest::blocking::ClientBuilder::new()
            .connect_timeout(Duration::from_secs(Client::DEFAULT_CONNECT_TIMEOUT))
            .timeout(Duration::from_secs(Client::DEFAULT_HTTP_TIMEOUT))
            .user_agent(&ag)
            .build()
            .unwrap();
        self.agent = Some(agent);
        self
    }

    /// Private routing function
    ///
    /// It is called with Ctx which represent the first level (`probe()`, `keys()`, etc.), generate
    /// the RequestBuilder object which will get fed by subsequent calls.  It will also ensure
    /// the API Key is filled in prior to everything.
    ///
    fn route_to(&self, ctx: Ctx) -> RequestBuilder {
        let url = self.endpoint.to_owned();

        // Default HTTP operation is GET, some will be POST/DELETE but that is handled in the
        // next call in the chain.
        let r = reqwest::blocking::Request::new(reqwest::Method::GET, url);

        // Enforce API key usage
        if self.api_key.is_none() {
            panic!("No API key defined");
        }

        let mut c = self.clone();
        c.opts.merge(&self.opts);

        // Ensure api-Key is filled in prior to the calls.
        c.opts["key"] = self.api_key.as_ref().unwrap().clone();
        RequestBuilder::new(ctx, c, r)
    }
}

// ---------------------------------------------------------------------------

/// `ClientBuilder` is the main struct to create and configure a `Client`. You have to close
/// the chain by calling `build()`.
///
/// Examples:
/// ```no_run
/// # fn main() -> Result<(), atlas_rs::errors::APIError> {
/// use atlas_rs::param::Param;
/// use atlas_rs::core::probes::Probe;
/// use atlas_rs::client::{AF, ClientBuilder};
///
/// let c = ClientBuilder::new()
///             .api_key("FOO")
///             .onoff(true)
///             .want_af(AF::V4)
///             .build()?;
///
/// let p: Probe = c.probe().get(666)?;
/// # Ok(())
/// # }
/// ```
///

pub struct ClientBuilder {
    cl: Client,
}

/// Default values for `ClientBuilder`
///
impl Default for ClientBuilder {
    /// Defines all the default values
    fn default() -> Self {
        ClientBuilder::new()
    }
}

/// Methods for `ClientBuilder`
///
impl ClientBuilder {
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
    pub fn build(self) -> Result<Client> {
        match &self.cl.api_key {
            Some(_k) => Ok(self.cl.clone()),
            None => Err(anyhow!("You must change the default key")),
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
    pub fn api_key(mut self, key: &str) -> Self {
        self.cl.api_key = Some(key.to_owned());
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
    pub fn endpoint(mut self, v: &str) -> Self {
        let endp = Url::parse(v).unwrap();
        self.cl.endpoint = endp;
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
    pub fn area_type(mut self, v: &str) -> Self {
        self.cl.area_type = v.to_owned();
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
    pub fn area_value(mut self, v: &str) -> Self {
        self.cl.area_value = v.to_owned();
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
    /// # use atlas_rs::client::{AF, ClientBuilder};
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
    pub fn tags<S: Into<String>>(mut self, v: S) -> Self {
        self.cl.tags = v.into();
        self
    }

    /// Add options (one by one)
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use atlas_rs::option::Options;
    /// # use atlas_rs::client::ClientBuilder;
    ///
    /// let c = ClientBuilder::new()
    ///     .opt("is_anchor", "true")
    ///     .opt("country", "fr")
    /// # ;
    /// ```
    ///
    pub fn opt(&self, k: &str, v: &str) -> Self {
        let mut cl = self.cl.clone();

        cl.opts[k] = v.to_string();
        ClientBuilder { cl }
    }

    /// Add a set of options
    ///
    /// Example
    /// ```no_run
    /// # use atlas_rs::option::Options;
    /// # use atlas_rs::client::ClientBuilder;
    ///
    /// let c = ClientBuilder::new()
    ///     .opts([("is_anchor", "true"), ("country", "fr")])
    /// # ;
    /// ```
    ///
    pub fn opts<const N: usize>(&self, arr: [(&str, &str); N]) -> Self {
        let mut cl = self.cl.clone();
        let o = Options::from(arr);
        cl.opts.merge(&o);
        ClientBuilder { cl }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_new() {
        let c = Client::new();

        // Check all defaults
        assert!(c.api_key.is_none());
        assert_eq!(ENDPOINT.to_string(), c.endpoint.as_str());
        assert_eq!("area".to_string(), c.area_type);
        assert_eq!("WW".to_string(), c.area_value);
        assert!(c.is_oneoff);
        assert_eq!(10, c.pool_size);
        assert_eq!(AF::V46, c.want_af);
        assert!(!c.verbose);
        assert_eq!("".to_string(), c.tags);
        assert!(c.agent.is_some());
    }

    #[test]
    fn test_clientbuilder_new() {
        let cb = ClientBuilder::new().api_key("key").build();

        assert!(cb.is_ok());

        let cb = cb.unwrap();

        // Check all defaults
        assert_eq!("key".to_string(), cb.api_key.unwrap());
        assert_eq!(ENDPOINT, cb.endpoint.as_str());
        assert_eq!("area".to_string(), cb.area_type);
        assert_eq!("WW".to_string(), cb.area_value);
        assert!(cb.is_oneoff);
        assert_eq!(10, cb.pool_size);
        assert_eq!(AF::V46, cb.want_af);
        assert!(!cb.verbose);
        assert_eq!("".to_string(), cb.tags);
        assert!(!cb.opts.contains_key("key"));
        assert!(cb.agent.is_some());
    }

    #[test]
    fn test_opt() {
        let h = [("foo", "a"), ("bar", "b"), ("key", "FOO")];

        let c = ClientBuilder::new()
            .api_key("key")
            .opt(h[0].0, h[0].1)
            .opt(h[1].0, h[1].1)
            .opt(h[2].0, h[2].1)
            .build();
        assert!(c.is_ok());

        let c = c.unwrap();
        assert_eq!(Options::from(h), c.opts);
        assert_eq!("key", c.api_key.unwrap());
        assert_eq!(h.len(), c.opts.len());
    }

    #[test]
    fn test_opts() {
        let h = [("foo", "a"), ("bar", "b"), ("key", "FOO")];

        let c = ClientBuilder::new().api_key("key").opts(h).build();
        assert!(c.is_ok());

        let c = c.unwrap();
        assert_eq!(Options::from(h), c.opts);
        assert_eq!("key", c.api_key.unwrap());
        assert_eq!(h.len(), c.opts.len());
    }

    #[test]
    fn test_clientbuilder_error() {
        let c = ClientBuilder::new().build();

        assert!(c.is_err());
    }

    #[test]
    fn test_clientbuilder_api_key() {
        let c = ClientBuilder::new().api_key("FOO").build();

        assert!(c.is_ok());
        assert!(c.as_ref().unwrap().api_key.is_some());

        let c = c.unwrap();

        let key = c.api_key;
        assert!(key.is_some());
        assert_eq!("FOO", key.unwrap());
    }

    #[test]
    fn test_onoff() {
        let c = ClientBuilder::new().api_key("key").onoff(true).build();

        assert!(c.unwrap().is_oneoff);
    }
}
