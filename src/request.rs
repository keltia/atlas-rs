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
use crate::anchors;
use crate::anchors::Anchor;
use crate::anchor_measurements;
use crate::anchor_measurements::AnchorMeasurement;
use crate::client::{Client, Cmd};
use crate::credits;
use crate::credits::Credits;
use crate::errors::APIError;
use crate::keys;
use crate::keys::Key;
use crate::option::Options;
use crate::probes;
use crate::probes::Probe;

// ------------------------------------------------------------

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

// Implement From: for our enum to pass stuff around without explicitely converting before.

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
    pub ctx: Cmd,
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
    pub fn new(ctx: Cmd, c: Client<'rq>, r: reqwest::blocking::Request) -> Self {
        RequestBuilder { ctx, c, r }
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
    /// ```rs
    /// # use atlas_rs::client::Client;
    ///
    /// let c = Client::new();
    ///
    /// let res = c.probe()
    ///             .get(666)
    ///             .call()?
    /// ```
    ///
    pub fn get<S: Into<Param<'rq>>>(&'rq mut self, data: S) -> &mut Self {
        // Main routing
        match self.ctx {
            Cmd::Probes => Probe::dispatch(self, probes::Ops::Get, data.into()),
            Cmd::Measurements => unimplemented!(),
            Cmd::AnchorMeasurements => AnchorMeasurement::dispatch(self, anchor_measurements::Ops::Get, data.into()),
            Cmd::Credits => Credits::dispatch(self, credits::Ops::Get, data.into()),
            Cmd::Anchors => Anchor::dispatch(self, anchors::Ops::Get, data.into()),
            Cmd::Keys => Key::dispatch(self, keys::Ops::Get, data.into()),
            Cmd::ParticipationRequests => unimplemented!(),
            Cmd::None => panic!("No Cmd"),
        }
    }

    /// This is the `list` method which return a set of results.
    ///
    /// Example:
    ///
    /// ```rs
    /// # use atlas_rs::client::Client;
    ///
    /// let c = Client::new();
    ///
    /// let res = c.probe()
    ///             .list()         // XXX
    ///             .call()?
    /// ```
    ///
    pub fn list<S: Into<Param<'rq>>>(&'rq mut self, data: S) -> &'rq mut Self {
        // Main routing
        match self.ctx {
            Cmd::Probes => Probe::dispatch(self, probes::Ops::List, data.into()),
            Cmd::Measurements => unimplemented!(),
            Cmd::AnchorMeasurements => unimplemented!(),
            Cmd::Credits => unimplemented!(),
            Cmd::Anchors => Anchor::dispatch(self, anchors::Ops::List, data.into()),
            Cmd::Keys => Key::dispatch(self, keys::Ops::List, data.into()),
            Cmd::ParticipationRequests => unimplemented!(),
            Cmd::None => panic!("No Cmd"),
        }
    }

    /// This is the `info` method close to `get` but without a parameter.
    ///
    /// Example:
    ///
    /// ```rs
    /// # use atlas_rs::client::Client;
    ///
    /// let c = Client::new();
    ///
    /// let res = c.probe()
    ///             .info()         // XXX
    ///             .call()?
    /// ```
    ///
    pub fn info(&'rq mut self) -> &'rq mut Self {
        // Main routing
        match self.ctx {
            Cmd::Probes => unimplemented!(),
            Cmd::Measurements => unimplemented!(),
            Cmd::AnchorMeasurements => unimplemented!(),
            Cmd::Credits => Credits::dispatch(self, credits::Ops::Get, 0.into()),
            Cmd::Anchors => unimplemented!(),
            Cmd::Keys => unimplemented!(),
            Cmd::ParticipationRequests => unimplemented!(),
            Cmd::None => panic!("No Cmd"),
        }
    }

    /// Makes it easy to specify options
    ///
    /// Example:
    ///
    /// ```rs
    /// # use atlas_rs::client::Client;
    ///
    /// let c = Client::new();
    ///
    /// let res = c.probe()
    ///             .get(pn)         // XXX
    ///             .with([("opt1", "foo"), ("opt2", "bar"))
    ///             call()?
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
        println!("in call");
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
}
