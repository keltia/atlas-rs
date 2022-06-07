//! This is the main set of type and methods implementing the main routing and dispatching
//! involved in method chaining to setup and run an HTTP request through `reqwest`.
//!
//! There is the ` Request` struct and its builder counterpart `RequestBuilder`.
//!
//! See [APIDESIGN.md](./APIDESIGN.md) for the list of methods and which is called in which context.
//!
//! The process is always creating a `Client` instance either with `new()` or through
//! the `ClientBuilder` chain.  Requests are then initiated by calling one of the categories
//! methods (like `probes()` and `keys()`) followed by the keyword of the action itself (like
//! `get()` or `list()` to fill-in parameters) to finish and do the actual API call.
//!
//! Almost everything is done here in `RequestBuilder` through its methods.  Everything that is
//! in the `core` crate is routing and establishing the URL and gathering the parameters.
//!
//! The calls here are generic over the type data you need to be returned like ‘Probe‘, ‘Key`, etc.
//!

mod paged;
mod single;

// Std library
//
use std::fmt::{Debug, Display};

// External crates
//
use anyhow::Result;
use itertools::Itertools;
use reqwest::Url;
use serde::de;
use serde::Deserialize;

// Our internal crates.
//
use crate::client::{Client, Ctx};
use crate::core::{
    anchor_measurements::AnchorMeasurement, anchors::Anchor, credits::Credits, keys::Key,
    measurements::Measurement, param::Param, participation_requests::ParticipationRequests,
    probes::Probe,
};
use crate::errors::APIError;
use crate::option::Options;
use crate::request::{paged::Paged, single::Single};

// ------------------------------------------------------------

/// All operations available to the various calls.
///
/// The selection of available operations for each type of data is done through the "core" module.
/// This is a flat list despite not all operations being available to all first level.
///
#[derive(Debug)]
pub enum Op {
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
fn get_ops_url(ctx: &Ctx, op: Op, p: Param) -> String {
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

/// When asking for a list of S, this generic struct is used for pagination
///
#[derive(Deserialize, Debug)]
pub struct List<S> {
    /// How many results in this block
    pub count: Option<u32>,
    /// URL to fetch the next block
    pub next: Option<String>,
    /// URL to fetch previous block
    pub previous: Option<String>,
    /// Current key block
    pub results: Vec<S>,
}

// -----------------

#[derive(Debug)]
pub enum Return<T> {
    Single(T),
    Paged(Vec<T>),
}

/// This is the trait we need to use for the call() stuff.
///
pub trait Callable<T> {
    /// Main function here
    ///
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

    /// Makes it easy to specify options
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use atlas_rs::client::Client;
    /// # use atlas_rs::core::probes::Probe;
    ///
    /// let c = Client::new();
    /// let query = vec!["country_code=fr"];
    ///
    /// let res: Vec<Probe> = c.probe()
    ///                        .with([("opt1", "foo"), ("opt2", "bar")])
    ///                        .list(query)
    ///                        .unwrap();
    /// ```
    /// This can be used to have subcommands like this:
    /// ```no_run
    /// # use atlas_rs::client::Client;
    /// # use atlas_rs::core::credits::Transaction;
    ///
    /// let c = Client::new();
    /// let query = vec!["country_code=fr"];
    ///
    /// let res: Vec<Transaction> = c.credits()
    ///                              .with(("type", "transaction"))
    ///                              .list(query)
    ///                              .unwrap();
    /// ```
    ///
    pub fn with(mut self, opts: impl Into<Options>) -> Self {
        self.c.opts.merge(&opts.into());
        self
    }

    // ------------------------------------------------------------------------------------
    /// Establish the final URL before call()
    ///
    /// These methods expect to be called by one of the main "categories" methods like
    /// `probes()` or `keys()`.  That way, context is established and propagated.
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
    pub fn get<P, T>(
        self,
        data: P,
    ) -> Box<dyn Callable<T>>
        where
            P: Into<Param> + Display + Debug,
            T: de::DeserializeOwned + Display,
    {
        let mut single = Single::from(self);
        single.query = data.into();
        Box::new(single)
    }

    /// This is the `list` method which return a set of results.  The results are automatically
    /// paginated, returning a different structure with pointers to the previous and next pages.
    ///
    /// ‘list()‘ takes a Param which represents a query made with Atlas' specific keywords and
    /// returns a ‘Vec<T>‘ representing a set of T objects.
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use atlas_rs::client::ClientBuilder;
    /// # use atlas_rs::core::probes::Probe;
    ///
    /// let mut c = ClientBuilder::new().api_key("FOO").build().unwrap();
    /// let query = vec!["country_code=fr"];
    ///
    /// let res: Vec<Probe> = c.probe().list(query).unwrap()
    /// # ;
    /// ```
    ///
    pub fn list<P, T>(self, data: P) -> Box<dyn Callable<T>>
        where
            P: Into<Param>,
            T: de::DeserializeOwned + std::fmt::Debug + Clone,
    {
        let mut paged = Paged::from(self);
        paged.query = data.into();
        Box::new(paged)
    }

    /// This is the `info` method close to `get` but without a parameter.
    ///
    /// You still get all the parameters from the options.
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
    pub fn info<T>(mut self) -> Box<dyn Callable<T>>
        where
            T: de::DeserializeOwned + Debug,
    {
        Box::new(Single::from(self))
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

    #[test]
    fn test_with() {
        let ctx = Ctx::Credits;
        let cl = Client::new();
        let url = Url::parse("http://localhost/").unwrap();
        let rq = Request::new(reqwest::Method::GET, url);
        let r = RequestBuilder::new(ctx, cl, rq);

        let r = r.with(("type", "income"));
        let add = get_ops_url(&ctx, Op::Info, Param::None);

        assert_eq!(reqwest::Method::GET, r.r.method());
        //assert_eq!("/credits/income-items", add);
    }
}
