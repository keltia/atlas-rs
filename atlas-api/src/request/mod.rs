//! This is the main module with its set of type and methods implementing the main routing
//! and dispatching involved in method chaining to setup and run an HTTP request through `reqwest`.
//!
//! This is the [builder] pattern with struct `RequestBuilder`.
//!
//! See [APIDESIGN] for the list of methods and which is called in which context.
//!
//! The process is always started by creating a `Client` instance either with `new()` or through
//! the `ClientBuilder` chain.  Requests are then initiated by calling one of the categories
//! methods (like `probes()` and `keys()`) followed by the keyword of the action itself (like
//! `get()` or `list()` to fill-in parameters). To finish and return results, use `.call()`.
//!
//! Almost everything is done here in `RequestBuilder` through its methods.  Everything that is
//! in the `core` crate is routing and establishing the URL and gathering the parameters.
//!
//! The calls here are generic over the type data you need to be returned like `Probe`, `Key`, etc.
//!
//! [builder]: https://en.wikipedia.org/wiki/Builder_pattern
//! [APIDESIGN]: ./APIDESIGN.md

// Std library
//
use std::fmt::Debug;

// External crates
//
use anyhow::Result;
use itertools::Itertools;
use reqwest::{Method, Url};

// Our internal crates.
//
use crate::client::{Client, Ctx};
use crate::core::{
    anchor_measurements::AnchorMeasurement, anchors::Anchor, credits::Credits, keys::Key,
    measurements::Measurement, participation_requests::ParticipationRequests, probes::Probe,
};
use crate::errors::APIError;
use crate::option::Options;
use crate::param::Param;
use crate::request::{paged::Paged, single::Single};

#[macro_use]
pub mod macros;

pub mod paged;
pub mod single;

// ------------------------------------------------------------

/// All operations available to the various calls.
///
/// The selection of available operations for each type of data is done through the "core" module.
/// This is a flat list despite not all operations being available to all first level.
///
#[derive(Clone, Debug)]
pub enum Op {
    /// Null op
    Null = 0,
    /// For Probe
    Archive,
    /// For Credits>Members
    Claim,
    /// For Key, Measurement
    Create,
    /// For Key, Measurement
    Delete,
    /// Credits
    Expenses,
    /// Anchor-Measurement, Anchors, Credits, Key, Measurement, Participation_Requests, Probes
    Get,
    /// Credits
    Incomes,
    /// Credits
    Info,
    /// Anchor-Measurement, Anchors, Credits, Key, Measurement, Participation_Requests, Probes
    List,
    /// Probe
    Measurement,
    /// Credits
    Members,
    /// Key
    Permissions,
    /// Probe
    Rankings,
    /// Key, Probe
    Set,
    /// Probe
    Slugs,
    /// Probe>Tag
    Tags,
    /// Key>Permissions
    Targets,
    /// Credits
    Transactions,
    /// Credits
    Transfers,
    /// Measurement, Probe
    Update,
}

// Dispatch table for the various operations in the different contexts.
//
pub fn get_ops_url(ctx: &Ctx, op: Op, p: Param) -> String {
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

// -----------------

#[derive(Debug)]
pub enum Return<T> {
    /// This gets deserialize as a single struct T
    Single(T),
    /// This generate a `Vec` of struct T
    Paged(Vec<T>),
    /// This is no result
    Null,
}

/// This is the trait we need to use for the call() stuff.
///
pub trait Callable<T> {
    fn call(self) -> Result<Return<T>, APIError>;
}

// RequestBuilder itself

/// This is the chaining struct, containing all the state we are interesting in passing around.
/// We do not need a special `Request` singleton (like for `Client` as most of what we need to
/// pass around will be stored in either `cl` (the `Client`) or `r` (the `reqwest::Request` struct).
///
#[derive(Debug)]
pub struct RequestBuilder {
    /// Context is which part of the API we are targetting (`/probe/`, etc.)
    pub ctx: Ctx,
    /// Client for API calls
    pub c: Client,
    /// Method to use for the HTTP call
    pub kw: reqwest::Method,
    /// Build our request here
    pub url: reqwest::Url,
    /// Full operation
    pub op: Op,
    /// Query parameters
    pub query: Param,
}

impl Default for RequestBuilder {
    /// Have some reasonable default
    ///
    fn default() -> Self {
        RequestBuilder {
            ctx: Ctx::None,
            c: Client::new(),
            kw: Method::GET,
            url: Url::parse("https://locahost").unwrap(),
            op: Op::Null,
            query: Param::None,
        }
    }
}

/// Methods for chaining and keeping state.
///
/// These methods expect to be called by one of the main "categories" methods like
/// `probes()` or `keys()`.  That way, context is established and propagated with all
/// necessary parameters.
///
/// Some calls have a parameter (type is `Param`) and it gets converted into the proper
/// type automatically depending on what the function wants to get.
///
/// Look for the [`Param`] enum for description of the various parameter types.
///
/// [`Param`]: crate::param::Param
///
impl RequestBuilder {
    /// Create an empty struct RequestBuilder
    ///
    pub fn new(ctx: Ctx, c: Client, kw: Method, url: reqwest::Url) -> Self {
        RequestBuilder {
            ctx,
            c,
            kw,
            url,
            ..RequestBuilder::default()
        }
    }

    // ------------------------------------------------------------------------------------
    // These invocations of the `action_keyword` macro generate the function body and its
    // documentation.
    //
    action_keyword!(get, Get, Single, data);

    action_keyword!(list, List, Paged, data);

    action_keyword!(info, Info, Single);

    action_keyword!(update, Update, Single, data);

    action_keyword!(delete, Delete, Single, data);

    action_keyword!(post, Update, Single, data);
}

/// Take an url and a set of options to add to the parameters
///
/// Example!
/// ```no_run
/// # use atlas_api::option::Options;
/// # use atlas_api::request::add_opts;
/// # use atlas_rs::option::Options;
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

    use crate::client::ENDPOINT;
    use crate::option::Options;

    use super::*;

    #[test]
    fn test_requestbuilder_new() {
        let ctx = Ctx::None;
        let cl = Client::new();
        let url = Url::parse("http://localhost/").unwrap();
        let r = RequestBuilder::new(ctx, cl, reqwest::Method::GET, url);

        assert_eq!(reqwest::Method::GET, r.kw);
    }

    #[test]
    fn test_add_opts() {
        let url = "/hello".to_string();
        let o = Options::from([("name", "foo"), ("bar", "baz")]);

        let url = add_opts(&url, &o);
        assert_eq!("/hello?bar=baz&name=foo", url);
    }

    #[test]
    fn test_with() {
        let ctx = Ctx::Credits;
        let cl = Client::new();
        let url = Url::parse("http://localhost/").unwrap();
        let r = RequestBuilder::new(ctx, cl, reqwest::Method::GET, url);

        //let r = r.with(("type", "income"));
        let add = get_ops_url(&ctx, Op::Info, Param::None);

        assert_eq!(reqwest::Method::GET, r.kw);
        //assert_eq!("/credits/income-items", add);
    }
}