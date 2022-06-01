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

// Std library
//
use std::fmt::Display;

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

// ------------------------------------------------------------

/// All operations available to the various calls.
///
/// The selection of available operations for each type of data is done through the "core" module.
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
    pub count: u32,
    /// URL to fetch the next block
    pub next: Option<String>,
    /// URL to fetch previous block
    pub previous: Option<String>,
    /// Current key block
    pub results: Vec<S>,
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
    pub fn get<T>(
        &mut self,
        data: impl Into<Param> + Display + std::fmt::Debug,
    ) -> Result<T, APIError>
    where
        T: de::DeserializeOwned + Display,
    {
        // Setup everything
        //
        let add = get_ops_url(&self.ctx, Op::Get, data.into());
        dbg!(&add);
        let opts = self.c.opts.iter();

        // Setup URL with potential parameters like `key`.
        //
        let url =
            Url::parse_with_params(format!("{}{}", &self.r.url().as_str(), add).as_str(), opts)
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
    pub fn list<P: Into<Param>, T>(&mut self, data: P) -> Result<Vec<T>, APIError>
    where
        T: de::DeserializeOwned + Display + std::fmt::Debug + Clone,
    {
        self.paged = true;

        // We will append all results here.
        //
        let mut res = Vec::<T>::new();

        let add = get_ops_url(&self.ctx, Op::List, data.into());
        dbg!(&add);
        let opts = self.c.opts.iter();

        // Setup URL with potential parameters like `key`.
        //
        let url =
            Url::parse_with_params(format!("{}{}", &self.r.url().as_str(), add).as_str(), opts)
                .unwrap();

        // Get data / opts for 1st call
        //
        let rawlist: List<T> = match self.fetch_one_page(url) {
            Ok(list) => list,
            Err(e) => return Err(e),
        };

        // Exit early with error if nothing
        //
        if rawlist.count == 0 {
            return Err(APIError::new(
                400,
                "Bad Call",
                "no data returned on pagination",
                "fetch_one_page",
            ));
        }

        // Get first results in
        //
        for e in rawlist.results.iter() {
            res.push(e.clone());
        }

        // Is there anything else?
        //
        if rawlist.next.is_some() {
            let mut nxt = rawlist.next;
            //let pn = get_page_num(nxt.as_ref().unwrap().to_owned());
            while nxt.is_some() {
                //let page = pn;
                let url = Url::parse(&nxt.unwrap()).unwrap();

                let rawlist: List<T> = match self.fetch_one_page(url) {
                    Ok(list) => list,
                    Err(e) => return Err(e),
                };
                // Get more results in
                for e in rawlist.results.iter() {
                    res.push(e.clone());
                }
                nxt = rawlist.next;
            }
        }

        println!("after res={:?}", res);
        Ok(res)
    }

    /// Implement a generic fetch_one_page() function.
    ///
    /// The API has complete support for this through a specific structure with previous and next
    /// pointers, along with the total item count and results which represente the actual data.
    ///
    /// You setup the first call as usual inserting your options and stuff and the next ones are
    /// just reusing the next pointer.
    ///
    /// This function just returns a `Vec<T>` where T the type of objects you are getting from the
    /// calls.
    ///
    /// Example:
    /// ```no_run
    /// # use atlas_rs::client::{Client, Ctx};
    /// # use atlas_rs::request::{List, RequestBuilder};
    /// # use atlas_rs::core::probes::Probe;
    /// #
    /// # let c = Client::new();
    /// # let ctx = Ctx::None;
    ///
    /// let url = reqwest::Url::parse("https://foo.example.net/").unwrap();
    /// let r = reqwest::blocking::Request::new(reqwest::Method::GET, url.clone());
    /// let rq = RequestBuilder::new(ctx, c, r);
    ///
    /// let rawlist: List<Probe> = rq.fetch_one_page(url).unwrap();
    /// if rawlist.next.is_some() {
    /// #
    /// }
    /// ```
    ///
    pub fn fetch_one_page<T>(&self, url: Url) -> Result<List<T>, APIError>
    where
        T: de::DeserializeOwned,
    {
        // Call the service
        //
        let req = reqwest::blocking::Request::new(self.r.method().clone(), url);
        let resp = self
            .c
            .agent
            .as_ref()
            .unwrap()
            .get(req.url().as_str())
            .send();

        match resp {
            Ok(resp) => {
                // Try to see if we got an error
                //
                match resp.status() {
                    reqwest::StatusCode::OK => {
                        // We could use Response::json() here but it consumes the body.
                        //
                        let r = resp.text()?;
                        println!("p={}", r);
                        let p: List<T> = serde_json::from_str(&r)?;
                        Ok(p)
                    }
                    _ => {
                        let aerr = resp.json::<APIError>()?;
                        Err(aerr)
                    }
                }
            }
            Err(e) => Err(APIError::new(
                e.status().unwrap().as_u16(),
                "Bad",
                e.to_string().as_str(),
                "fetch_one_page",
            )),
        }
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
    pub fn info<T>(mut self) -> Result<T, APIError>
    where
        T: de::DeserializeOwned + Display,
    {
        // Setup everything
        //
        let add = get_ops_url(&self.ctx, Op::Info, Param::None);
        dbg!(&add);
        let opts = self.c.opts.iter();

        // Setup URL with potential parameters like `key`.
        //
        let url =
            Url::parse_with_params(format!("{}{}", &self.r.url().as_str(), add).as_str(), opts)
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
