//! Module implementing the `Paged` type of requests, it basically loops over the results
//! and returns a single vector.
//!
//! TODO: add an iterator.
//!

use std::fmt::Debug;
use std::slice::Iter;

use reqwest::{Method, Url};
use serde::de::DeserializeOwned;
use serde::Deserialize;

use crate::client::{Client, Ctx};
use crate::errors::APIError;
use crate::option::Options;
use crate::param::Param;
use crate::request::{Callable, get_ops_url, Op, RequestBuilder, Return};

// ------------------------------------------------------------

/// When asking for a list of S, this generic struct is used for pagination
///
#[derive(Clone, Debug, Deserialize)]
pub struct List<T> {
    /// How many results in this block
    pub count: Option<u32>,
    /// URL to fetch the next block
    pub next: Option<String>,
    /// URL to fetch previous block
    pub previous: Option<String>,
    /// Current key block
    pub results: Vec<T>,
}

impl<T> List<T>
    where T: DeserializeOwned + Debug + Clone,
{
    /// Gets an iterator over the values of the map.
    #[inline]
    pub fn iter(&self) -> Iter<'_, T> {
        self.results.iter()
    }
}

/*impl<'de, T> IntoIterator for &'de List<T>
where T: DeserializeOwned,
{
    type Item = &'de T;
    type IntoIter = Iter<'de, T>;

    fn into_iter(self) -> Iter<'de, T> {
        self.results.iter()
    }
}*/

/// Derivative of `RequestBuilder` with a flatter structure
///
#[derive(Debug)]
pub struct Paged {
    /// Context is which part of the API we are targetting (`/probe/`, etc.)
    pub ctx: Ctx,
    /// Options, merge of CLI input and default config.
    pub opts: Options,
    /// Parameter given to `get()`, will be `Param::None` for `infop()`.
    pub query: Param,
    /// Cache of the URL method (GET, PUT, etc.)
    pub m: Method,
    /// Will be used to construct the final URL to call
    pub url: Url,
    /// HTTP Client
    pub c: Client,
}

impl Default for Paged {
    fn default() -> Self {
        Paged {
            ctx: Ctx::None,
            c: Client::new(),
            opts: Options::new(),
            query: Param::None,
            m: Method::GET,
            url: "".parse().unwrap(),
        }
    }
}

impl Paged {
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
        self.opts.merge(&opts.into());
        self
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
    /// # use atlas_rs::core::probes::Probe;
    /// # use atlas_rs::request::paged::List;
    /// # use atlas_rs::request::RequestBuilder;
    /// #
    /// # let c = Client::new();
    /// # let ctx = Ctx::None;
    ///
    /// let url = reqwest::Url::parse("https://foo.example.net/").unwrap();
    /// let rq = RequestBuilder::new(ctx, c, reqwest::Method::GET, url);
    ///
    /// let rawlist: List<Probe> = rq.fetch_one_page(url.as_str()).unwrap();
    /// if rawlist.next.is_some() {
    /// #
    /// }
    /// ```
    ///
    pub fn fetch_one_page<T>(&self, url: Url) -> Result<List<T>, APIError>
        where T: DeserializeOwned + Debug + Clone,
    {
        // Call the service
        //
        let req = reqwest::blocking::Request::new(self.m.clone(), url);
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
}

impl From<RequestBuilder> for Paged {
    /// Makes chaining easier.
    ///
    fn from(rb: RequestBuilder) -> Self {
        Paged {
            c: rb.c.clone(),
            opts: rb.c.opts.clone(),
            query: rb.query.clone(),
            url: rb.url.clone(),
            ..Default::default()
        }
    }
}

impl<T> Callable<T> for Paged
    where T: DeserializeOwned + Debug + Clone,
{
    /// Single most important call for the whole structure
    ///
    fn call(self) -> Result<Return<T>, APIError>
    {
        // Get the potential "type" option
        //
        let tt = &self.c.opts["type"];

        // Keep all options except for "type" as we don't want to send this internal option
        // along with the query.
        //
        let opts = self.c.opts.iter().filter_map(|k| {
            if k.0 != "type" {
                Some((k.0.as_str(), k.1.as_str()))
            } else {
                None
            }
        });

        // Now, check the "type" value
        //
        let op = match tt.as_str() {
            // Credits stuff
            "expense-items" => Op::Expenses,
            "income-items" => Op::Incomes,
            "members" => Op::Members,
            "transactions" => Op::Transactions,
            "transfer" => Op::Transfers,
            //
            _ => Op::Info,
        };

        let query = self.query.to_owned();
        let add = get_ops_url(&self.ctx, op, query);
        dbg!(&add);

        // Setup URL with potential parameters like `key`.
        //
        let url =
            Url::parse_with_params(format!("{}{}", &self.url.as_str(), add).as_str(), opts)
                .unwrap();

        // Get data / opts for 1st call
        //
        let rawlist: List<T> = match self.fetch_one_page(url) {
            Ok(list) => list,
            Err(e) => return Err(e),
        };

        // Exit early with error if nothing
        //
        match rawlist.count {
            Some(count) => if count == 0 {
                return Err(APIError::new(
                    400,
                    "Bad Call",
                    "no data returned on pagination",
                    "fetch_one_page",
                ))
            }
            _ => (),
        }

        // We will append all results here.
        //
        let mut res = Vec::<T>::with_capacity(rawlist.count.unwrap() as usize);

        // Get first results in
        //
        for elem in rawlist.results.iter() {
            res.push(elem.clone());
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

        assert_eq!(rawlist.count.unwrap() as usize, res.len());
        dbg!(&res);
        Ok(Return::Paged(res))
    }
}

