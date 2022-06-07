//! Module implementing the `Single` type of requests,
//!

use reqwest::{Method, Url};

use crate::client::{Client, Ctx};
use crate::errors::APIError;
use crate::option::Options;
use crate::param::Param;
use crate::request::{Callable, get_ops_url, Op, RequestBuilder, Return};

/// Derivative of `RequestBuilder` with a flatter structure
///
pub struct Single {
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
            c: Client::new(),
            opts: Options::new(),
            query: Param::None,
            m: Method::GET,
            url: "".parse().unwrap(),
        }
    }
}

impl Single {
    /// We need `with()` for both `Single` and `Paged` to be consistent.
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
            url: rb.url.clone(),
            ..Default::default()
        }
    }
}

impl<T> Callable<T> for Single {
    /// Single most important call for the whole structure
    ///
    fn call(self) -> Result<Return<T>, APIError> {
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

        let res: T = serde_json::from_str(&txt)?;
        dbg!(&res);

        Ok(Return::Single(res))
    }
}
