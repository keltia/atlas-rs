//! Module implementing the `Single` type of requests,
//!

use std::fmt::Debug;
use reqwest::{Method, Url};
use serde::Deserialize;

use crate::client::{Client, Ctx};
use crate::errors::APIError;
use crate::option::Options;
use crate::param::Param;
use crate::request::{Callable, get_ops_url, Op, RequestBuilder, Return};

/// Derivative of `RequestBuilder` with a flatter structure
///
#[derive(Debug, Deserialize)]
pub struct Single {
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

impl Default for Single {
    fn default() -> Self {
        Single {
            ctx: Ctx::None,
            c: Client::new(),
            opts: Options::new(),
            query: Param::None,
            m: Method::GET,
            url: "".parse().unwrap(),
        }
    }
}

impl Single {
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
}

impl From<RequestBuilder> for Single {
    /// Makes chaining easier.
    ///
    fn from(rb: RequestBuilder) -> Self {
        Single {
            c: rb.c.clone(),
            opts: rb.c.opts.clone(),
            url: rb.r.url().clone(),
            ..Default::default()
        }
    }
}

impl<'a, T> Callable<T> for Single
    where T: Deserialize<'a> + Debug + Copy,
{
    /// Single most important call for the whole structure
    ///
    fn call(self) -> Result<Return<T>, APIError> {
        // Setup everything
        //
        let add = get_ops_url(&self.ctx, Op::Get, self.query);
        dbg!(&add);
        let opts = self.c.opts.iter();

        // Setup URL with potential parameters like `key`.
        //
        let url =
            Url::parse_with_params(format!("{}{}", &self.url.as_str(), add).as_str(), opts)
                .unwrap();

        let r = reqwest::blocking::Request::new(self.m.clone(), url);
        let resp = self
            .c
            .agent
            .as_ref()
            .unwrap()
            .get(r.url().as_str())
            .send()?;

        println!("{:?} - {:?}", self.c.opts, r.url().as_str());

        let txt = resp.text()?;
        println!("after text={}", txt);

        let res: T = serde_json::from_str(&txt)?;
        dbg!(&res);

        Ok(Return::Single(res))
    }
}
