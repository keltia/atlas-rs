//! This is the main set of type and methods implementing the main routing and dispatching
//! involved in method chaining to setup and run an HTTP request through `reqwest`.
//!
//! There is the ` Request` struct and its builder counterpart `RequestBuilder`.
//! See `APIDESIGN.md` for the details list of calls and the contex/category on
//! each of them.
//!
//! The process is always creating a `Client` instance either with `new()` or through
//! the `ClientBuilder` chain.  Requests are then initiated by calling one of the categories
//! methods (like `probes()` and `keys()`) followed by the keyword of the action itself (like
//! `get()` or `list()` to five parameters) with `call()`to finish and do the actual API call.
//!
//!

// Std library
//
use std::fmt::Display;

// External crates
//
use anyhow::Result;
use itertools::Itertools;
use serde::de;

// Our internal crates.
//
use crate::client::{Client, Ctx};
use crate::common::Routing;
use crate::core::{
    anchor_measurements::AnchorMeasurement, anchors::Anchor, credits::Credits, keys::Key,
    measurements::Measurement, participation_requests::ParticipationRequests, probes::Probe,
};
use crate::errors::APIError;
use crate::option::Options;
use crate::param::Param;

// ------------------------------------------------------------

/// All operations available
///
#[derive(Debug)]
pub enum Op {
    Archive,
    Claim,
    Create,
    Delete,
    Expenses,
    Get,
    Incomes,
    Info,
    List,
    Measurement,
    Members,
    Permissions,
    Rankings,
    Set,
    Slugs,
    Tags,
    Targets,
    Transactions,
    Transfers,
    Update,
}

// Dispatch table for the various operations in the different contexts.
//
fn get_ops_url<T: Display>(ctx: &Ctx, op: Op, p: T) -> String {
    match ctx {
        Ctx::AnchorMeasurements => AnchorMeasurement::set_url(op, p),
        Ctx::Anchors => Anchor::set_url(op, p),
        Ctx::Credits => Credits::set_url(op, p),
        Ctx::Keys => Key::set_url(op, p),
        Ctx::Measurements => Measurement::set_url(op, p),
        Ctx::ParticipationRequests => ParticipationRequests::set_url(op, p),
        Ctx::Probes => Probe::set_url(op, p),
        Ctx::None => panic!("should not happen"),
    }
}

// ------------------------------------------------------------

// RequestBuilder itself

/// This is the chaining struct, containing all the state we are interesting in passing around.
/// We do not need a special `Request` singleton (like for `Client` as most of what we need to
/// pass around will be stored in either `cl` (the `Client`) or `r` (the `reqwest::Request` struct).
///
#[derive(Debug)]
pub struct RequestBuilder {
    /// Context is which part of the API we are targetting (`/probe/`, etc.)
    pub ctx: Ctx,
    /// Do we return paginated results?
    pub paged: bool,
    /// Client for API calls
    pub c: Client,
    /// Build our request here
    pub r: reqwest::blocking::Request,
}

/// Add methods for chaining and keeping state.
///
impl RequestBuilder {
    /// Create an empty struct RequestBuilder
    ///
    pub fn new(ctx: Ctx, c: Client, r: reqwest::blocking::Request) -> Self {
        RequestBuilder {
            ctx,
            paged: false,
            c,
            r,
        }
    }

    // ------------------------------------------------------------------------------------
    /// Establish the final URL before call()
    ///
    /// These methods expect to be called by one of the main "categories" methods like
    /// `probes()` or `keys()`.  That way, context is established znd propagated.
    ///
    /// In essence, these is the main router.  See [./APIDESIGN.md] for the list of methods
    /// and which is called in which context.
    ///
    /// Some calls have a parameter (type is `Param`) and it gets converted into the proper
    /// type automatically depending on the `dispatch` function wants to get.
    ///
    /// This is the `get` method for single results and a parameter.
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use atlas_rs::client::ClientBuilder;
    /// # use atlas_rs::core::probes::Probe;
    ///
    /// let mut c = ClientBuilder::new().api_key("FOO").build().unwrap();
    ///
    /// let res: Probe = c.probe().get(666).unwrap()
    /// # ;
    /// ```
    ///
    pub fn get<T>(&mut self, data: impl Into<Param> + Display) -> Result<T, APIError>
    where
        T: de::DeserializeOwned + Display,
    {
        // Get the parameter
        let add = get_ops_url(&self.ctx, Op::Get, data);

        let opts = self.c.opts.iter();

        // Setup URL with potential parameters like `key`.
        let url = reqwest::Url::parse_with_params(
            format!("{}{}", self.r.url().as_str(), add).as_str(),
            opts,
        )
        .unwrap();

        self.r = reqwest::blocking::Request::new(self.r.method().clone(), url);
        let resp = self
            .c
            .agent
            .as_ref()
            .unwrap()
            .get(self.r.url().as_str())
            .send()?;

        println!("{:?} - {:?}", self.c.opts, self.r.url().as_str());

        let txt = resp.text()?;
        println!("after text={}", txt);

        let r: T = serde_json::from_str(&txt)?;
        println!("after r={}", r);
        Ok(r)
    }

    /// This is the `list` method which return a set of results.
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use atlas_rs::client::ClientBuilder;
    /// # use atlas_rs::core::probes::Probe;
    ///
    /// let mut c = ClientBuilder::new().api_key("FOO").build().unwrap();
    ///
    ///
    /// let res: Vec<Probe> = c.probe().list(0).unwrap()
    /// # ;
    /// ```
    ///
    pub fn list<T>(&mut self, data: impl Into<Param> + Display) -> Result<Vec<T>, APIError>
    where
        T: de::DeserializeOwned + std::fmt::Display + std::fmt::Debug,
    {
        self.paged = true;
        // Main routing

        // Get the parameter
        let add = get_ops_url(&self.ctx, Op::List, data.into());

        let opts = self.c.opts.iter();

        // Setup URL with potential parameters like `key`.
        let url = reqwest::Url::parse_with_params(
            format!("{}{}", self.r.url().as_str(), add).as_str(),
            opts,
        )
        .unwrap();

        self.r = reqwest::blocking::Request::new(self.r.method().clone(), url);
        let resp = self
            .c
            .agent
            .as_ref()
            .unwrap()
            .get(self.r.url().as_str())
            .send()?;

        println!("{:?} - {:?}", self.c.opts, self.r.url().as_str());

        let txt = resp.text()?;
        println!("after text={}", txt);

        let r: Vec<T> = serde_json::from_str(&txt)?;
        println!("after r={:?}", r);
        Ok(r)
    }

    /// This is the `info` method close to `get` but without a parameter.
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use atlas_rs::client::ClientBuilder;
    /// # use atlas_rs::core::keys::Key;
    ///
    /// let mut c = ClientBuilder::new().api_key("FOO").build().unwrap();
    ///
    /// let res: Key = c.keys().info().unwrap()
    /// # ;
    /// ```
    ///
    pub fn info<T>(mut self) -> Result<T, APIError>
    where
        T: de::DeserializeOwned + std::fmt::Display,
    {
        // Get the parameter
        let add = get_ops_url(&self.ctx, Op::Info, 0u32);

        let opts = self.c.opts.iter();

        // Setup URL with potential parameters like `key`.
        let url = reqwest::Url::parse_with_params(
            format!("{}{}", self.r.url().as_str(), add).as_str(),
            opts,
        )
        .unwrap();

        self.r = reqwest::blocking::Request::new(self.r.method().clone(), url);
        let resp = self
            .c
            .agent
            .as_ref()
            .unwrap()
            .get(self.r.url().as_str())
            .send()?;

        println!("{:?} - {:?}", self.c.opts, self.r.url().as_str());

        let txt = resp.text()?;
        println!("after text={}", txt);

        let r: T = serde_json::from_str(&txt)?;
        println!("after r={}", r);
        Ok(r)
    }

    /// Makes it easy to specify options
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use atlas_rs::client::Client;
    /// # use atlas_rs::core::probes::Probe;
    ///
    /// let c = Client::new();
    ///
    /// let res: Vec<Probe> = c.probe()
    ///                        .with([("opt1", "foo"), ("opt2", "bar")])
    ///                        .list(0u32)
    ///                        .unwrap()
    /// # ;
    /// ```
    ///
    pub fn with(mut self, opts: impl Into<Options>) -> Self {
        self.c.opts.merge(&opts.into());
        self
    }
}

/// Take an url and a set of options to add to the parameters
///
/// Example!
/// ```no_run
/// # use atlas_rs::option::Options;
/// # use atlas_rs::request::add_opts;
///
/// let url = "https://example.com/";
/// let opts = Options::from([("foo", "bar")]);
/// let url = add_opts(&url, &opts);
/// ```
///
pub fn add_opts(url: &str, opts: &Options) -> String {
    let full = url.to_owned() + "?";
    let mut v = Vec::<String>::new();

    for name in opts.keys().sorted() {
        let opt = format!("{}={}", name, opts[name]);
        v.push(opt);
    }
    full + &v.join("&")
}

#[cfg(test)]
mod tests {
    use reqwest::blocking::Request;
    use reqwest::Url;

    use crate::option::Options;

    use super::*;

    #[test]
    fn test_requestbuilder_new() {
        let ctx = Ctx::None;
        let cl = Client::new();
        let url = Url::parse("http://localhost/").unwrap();
        let rq = Request::new(reqwest::Method::GET, url);
        let r = RequestBuilder::new(ctx, cl, rq);

        assert!(!r.paged);
        assert_eq!(reqwest::Method::GET, r.r.method());
    }

    #[test]
    fn test_add_opts() {
        let url = "/hello".to_string();
        let o = Options::from([("name", "foo"), ("bar", "baz")]);

        let url = add_opts(&url, &o);
        assert_eq!("/hello?bar=baz&name=foo", url);
    }
}
