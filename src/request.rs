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

// External crates
use anyhow::Result;
use serde::de;

// Our internal crates.
//
use crate::anchor_measurements;
use crate::anchor_measurements::AnchorMeasurement;
use crate::anchors;
use crate::anchors::Anchor;
use crate::client::{Client, Ctx};
use crate::common::Callable;
use crate::credits;
use crate::credits::Credits;
use crate::errors::APIError;
use crate::keys;
use crate::keys::Key;
use crate::measurements;
use crate::measurements::Measurement;
use crate::option::Options;
use crate::participation_requests;
use crate::participation_requests::ParticipationRequests;
use crate::probes;
use crate::probes::Probe;

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
    Tags,
    Transactions,
    Transfers,
    Update,
}

// Dispatch table for the various operations in the different contexts.
//
fn get_ops_url<T>(ctx: &Ctx, op: Op, p: T) -> String {
    match ctx {
        Ctx::AnchorMeasurements => anchor_measurements::set_url(op, p.into()),
        Ctx::Anchors => anchors::set_url(op, p.into()),
        Ctx::Credits => credits::set_url(op),
        Ctx::Keys => keys::set_url(op, p.into()),
        Ctx::Measurements => measurements::set_url(op, p.into()),
        Ctx::ParticipationRequests => participation_requests::set_url(op, p.into()),
        Ctx::Probes => probes::set_url(op, p.into()),
        Ctx::None => panic!("should not happen"),
    }
}

/// This enum is for passing the right kind of parameter to `get()`,
/// there might be a better way for this.
///
#[derive(Clone, Copy, Debug)]
pub enum Param<'a> {
    /// Represents the most usual 32-bit integer
    I(i32),
    /// Represents an unsigned 32-bit integer
    U(u32),
    /// Represents the long aka 64-bit integer
    L(i64),
    /// Represents the string pointer aka `str`
    S(&'a str),
}

// Implement From: for our enum to pass stuff around without explicitly converting before.

/// From &str to Param
///
impl<'a> From<&'a str> for Param<'a> {
    fn from(s: &'a str) -> Self {
        Param::S(s)
    }
}

/// From Param to &str
///
impl<'a> From<Param<'a>> for &'a str {
    fn from(p: Param<'a>) -> Self {
        match p {
            Param::S(s) => s,
            _ => "",
        }
    }
}

/// From Param to String
///
impl<'a> From<Param<'a>> for String {
    fn from(p: Param<'a>) -> Self {
        match p {
            Param::S(s) => s.to_string(),
            _ => "".to_string(),
        }
    }
}

/// From u32 to Param
///
impl<'a> From<u32> for Param<'a> {
    fn from(p: u32) -> Self {
        Param::U(p)
    }
}

/// From Param to u32
///
impl<'a> From<Param<'a>> for u32 {
    fn from(p: Param<'a>) -> Self {
        match p {
            Param::U(v) => v,
            _ => 0,
        }
    }
}

/// From i64 to Param
///
impl<'a> From<i64> for Param<'a> {
    fn from(p: i64) -> Self {
        Param::L(p)
    }
}

/// From i32 to Param
///
impl<'a> From<i32> for Param<'a> {
    fn from(p: i32) -> Self {
        Param::I(p)
    }
}

/// From Param to i32
///
impl<'a> From<Param<'a>> for i32 {
    fn from(p: Param<'a>) -> Self {
        match p {
            Param::I(v) => v,
            _ => 0,
        }
    }
}

/// From Param to i64
///
impl<'a> From<Param<'a>> for i64 {
    fn from(p: Param<'a>) -> Self {
        match p {
            Param::L(v) => v,
            _ => 0,
        }
    }
}

// ------------------------------------------------------------
// RequestBuilder itself

/// This is the chaining struct, containing all the state we are interesting in passing around.
/// We do not need a special `Request` singleton (like for `Client` as most of what we need to
/// pass around will be stored in either `cl` (the `Client`) or `r` (the `reqwest::Request` struct).
///
#[derive(Debug)]
pub struct RequestBuilder<'rq> {
    /// Context is which part of the API we are targetting (`/probe/`, etc.)
    pub ctx: Ctx,
    /// Do we return paginated results?
    pub paged: bool,
    /// Client for API calls
    pub c: Client<'rq>,
    /// Build our request here
    pub r: reqwest::blocking::Request,
}

/// Add methods for chaining and keeping state.
///
impl<'rq> RequestBuilder<'rq> {
    /// Create an empty struct RequestBuilder
    ///
    pub fn new(ctx: Ctx, c: Client<'rq>, r: reqwest::blocking::Request) -> Self {
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
    /// ```
    /// # use atlas_rs::client::Client;
    ///
    /// let c = Client::new();
    ///
    /// let res = c.probe()
    ///             .get(666)
    ///             .call()?
    /// # ;
    /// ```
    ///
    pub fn get<S, T>(&'rq mut self, data: S) -> Result<T, APIError>
    where
        S: Into<Param<'rq>>,
        T: de::DeserializeOwned + std::fmt::Display,
    {
        // Get the parameter
        let add = get_ops_url(&self.ctx, Op::Get, data.into());

        // Setup URL with potential parameters like `key`.
        let url = reqwest::Url::parse_with_params(
            format!("{}{}", self.r.url().as_str(), add).as_str(),
            &self.c.opts,
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
    /// ```
    /// # use atlas_rs::client::Client;
    ///
    /// let c = Client::new();
    ///
    /// let res = c.probe()
    ///             .list(data)?
    /// # ;
    /// ```
    ///
    pub fn list<S, T>(&'rq mut self, data: S) -> Result<Vec<T>, APIError>
    where
        S: Into<Param<'rq>>,
        T: de::DeserializeOwned + std::fmt::Display + std::fmt::Debug,
    {
        self.paged = true;
        // Main routing

        // Get the parameter
        let add = get_ops_url(&self.ctx, Op::List, data.into());

        // Setup URL with potential parameters like `key`.
        let url = reqwest::Url::parse_with_params(
            format!("{}{}", self.r.url().as_str(), add).as_str(),
            &self.c.opts,
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
    /// ```
    /// # use atlas_rs::client::Client;
    ///
    /// let c = Client::new();
    ///
    /// let res = c.probe()
    ///             .info()?
    /// # ;
    /// ```
    ///
    pub fn info<T>(&'rq mut self) -> Result<T, APIError>
    where
        T: de::DeserializeOwned + std::fmt::Display,
    {
        // Get the parameter
        let add = get_ops_url(&self.ctx, Op::Info, 0.into());

        // Setup URL with potential parameters like `key`.
        let url = reqwest::Url::parse_with_params(
            format!("{}{}", self.r.url().as_str(), add).as_str(),
            &self.c.opts,
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
    ///
    /// let c = Client::new();
    ///
    /// let res = c.probe()
    ///             .with([("opt1", "foo"), ("opt2", "bar")].into())
    ///             .list(data)?         // XXX
    /// # ;
    /// ```
    ///
    pub fn with(&'rq mut self, opts: &Options<'rq>) -> &'rq mut Self {
        for (key, item) in opts.iter() {
            self.c.opts.insert(*key, *item);
        }
        self
    }

    /// Finalize the chain and call the real API
    ///
    pub fn call<T>(&self) -> Result<T, APIError>
    where
        T: de::DeserializeOwned + std::fmt::Display,
    {
        unimplemented!()
    }
}

impl<K,V;N as usize>