use std::collections::HashMap;
use std::time::Duration;

use clap::{crate_name,crate_version};

/// We target the v2 API (not sure if it needs to be public)
pub(crate) const ENDPOINT: &str = "https://atlas.ripe.net/api/v2";

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

/// This is the main client struct and the different configuration methods.
///
/// The way to configure it is different from the Go way and more in line
/// with current Rust practices.
///
/// The only mandatory argument is the API key so it is given to `new()` and
/// all the other methods are there for configuration everything you want to
/// change from the default.
///
/// `NOTE` none of the fields are public except within the crate
///
/// `XXX` There is no `api_key()` method to enable changing the API key between
/// calls.  Not sure it would be useful.
///
/// Examples:
/// ```
/// use atlas_rs::client::{AF,Client};
///
/// let c = Client::new("FOOBAR")
///             .onoff(true)
///             .default_probe(666)
///             .want_af(AF::V4);
/// ```
///
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

    /// Internal state, http client
    pub(crate) agent: Option<ureq::Agent>,
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
            agent: None,
        }
    }
}

/// All methods for `Client` for configuration
impl<'cl> Client<'cl> {
    /// Create a new `Client` instance with the specified key
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
        .httpclient()
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
    pub fn endpoint<S: Into<&'cl str>>(mut self, v: S) -> Self {
        self.endpoint = v.into();
        self
    }

    /// Sets the default probe ID
    ///
    /// Examples
    ///
    /// ```no_run
    /// # use atlas_rs::client::Client;
    /// Client::new("FOO")
    ///     .default_probe(666)
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

    /// Sets the tags to be sent with the requests
    ///    +tag / tag  ==> tags_include
    ///    -tag / !tag ==> tags_exclude
    ///
    /// Examples
    ///
    /// ```no_run
    /// # use atlas_rs::client::{AF,Client};
    /// Client::new("FOO")
    ///     .tags("ftth !cable")
    /// # ;
    /// ```
    pub fn tags<S: Into<&'cl str>>(mut self, v: S) -> Self {
        self.tags = v.into();
        self
    }

    /// Create an instance of the HTTP client and attach it there
    ///
    /// Examples
    /// ```no_run
    /// # use atlas_rs::client::{AF,Client};
    /// Client::new("FOO")
    ///     .httpclient()
    /// # ;
    /// ```
    pub fn httpclient(mut self) -> Self {
        let ps = std::env::var("all_proxy");
        let ps = match ps {
            Ok(p) => p,
            Err(_e) => match std::env::var("https_proxy") {
                Ok(p) => p,
                Err(_e) => match std::env::var("http_proxy") {
                    Ok(p) => p,
                    Err(e) => e.to_string(),
                },
            },
        };

        let ag = format!("{}/{}", crate_name!(), crate_version!());
        let proxy = ureq::Proxy::new(ps).unwrap();
        let agent = ureq::AgentBuilder::new()
            .timeout_connect(Duration::from_secs(10))
            .timeout_read(Duration::from_secs(5))
            .timeout_write(Duration::from_secs(5))
            .proxy(proxy)
            .user_agent(&ag)
            .build();
        self.agent = Some(agent);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
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
        assert_eq!(HashMap::new(), c.opts);
        assert!(c.agent.is_some());
    }

    #[test]
    fn test_onoff() {
        let c = Client::new("FOO").onoff(true);

        assert!(c.is_oneoff);
    }
}
