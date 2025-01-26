//! Module implementing the `Single` type of requests,
//!

use std::fmt::Debug;

use reqwest::{Method, Url};
use serde::de::DeserializeOwned;

use crate::client::{Client, Ctx, ENDPOINT};
use crate::errors::APIError;
use crate::option::Options;
use crate::param::Param;
use crate::request::{get_ops_url, Callable, Op, RequestBuilder, Return};

/// A structure for making single-request calls to the API.
///
/// `Single` is a simplified version of `RequestBuilder`, providing a flatter
/// structure for managing API calls. It encapsulates multiple components like
/// context, options, query parameters, HTTP method, URL, and client details.
///
/// ### Fields
///
/// - `ctx`: Represents the API context being targeted (e.g., `/probe/`, etc.).
/// - `opts`: A set of available options, which are a combination of CLI input
///   and default configuration.
/// - `query`: The parameter(s) provided to the API (e.g., `Param::None` for
///   default calls like `infop()`).
/// - `m`: Represents the HTTP method for the request (e.g., GET, PUT, etc.).
/// - `url`: The URL used to make the API call.
/// - `c`: The HTTP client instance for handling requests.
/// - `op`: Specifies the type of API operation being performed.
///
/// ### Usage
///
/// The `Single` structure can be used to construct and execute API calls. Use
/// its methods to configure and chain operations conveniently.
///
/// #### Example: Adding Options
///
/// ```no_run
/// use atlas_api::client::Client;
///
/// let c = Client::new();
/// let query = vec!["country_code=fr"];
///
/// let res = c.probe()
///            .list(query)
///            .with([("opt1", "foo"), ("opt2", "bar")]);
/// ```
///
/// #### Example: Subcommand Style
///
/// ```no_run
/// use atlas_api::client::Client;
/// use atlas_api::errors::APIError;
/// use atlas_api::request::Return;
///
/// let c = Client::new();
/// let query = vec!["country_code=fr"];
///
/// let res = c.credits().list(query).with([("type", "transaction")]);
/// ```
///
#[derive(Debug)]
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
    /// API Operation
    pub op: Op,
}

impl Default for Single {
    fn default() -> Self {
        Single {
            ctx: Ctx::None,
            c: Client::new(),
            opts: Options::new(),
            query: Param::None,
            m: Method::GET,
            url: ENDPOINT.parse().unwrap(),
            op: Op::Null,
        }
    }
}

impl Single {
    /// Makes it easy to specify options
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use atlas_api::client::Client;
    /// # use atlas_api::core::probes::Probe;
    ///
    /// let c = Client::new();
    /// let query = vec!["country_code=fr"];
    ///
    /// let res = c.probe().list(query).with([("opt1", "foo"), ("opt2", "bar")]);
    /// ```
    ///
    /// This can be used to have subcommands like this:
    /// ```no_run
    /// # use atlas_api::client::Client;
    /// # use atlas_api::core::credits::Transaction;
    /// use atlas_api::errors::APIError;
    /// use atlas_api::request::Return;
    ///
    /// let c = Client::new();
    /// let query = vec!["country_code=fr"];
    ///
    /// let res = c.credits().list(query).with([("type", "transaction")]);
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
            ctx: rb.ctx,
            c: rb.c.clone(),
            opts: rb.c.opts.clone(),
            url: rb.url.clone(),
            m: rb.kw.clone(),
            query: rb.query.clone(),
            op: rb.op,
        }
    }
}

/// Calls the API endpoint using the `Single` structure.
///
/// This function prepares the API request, including constructing the URL,
/// attaching query parameters, and sending the HTTP request using the
/// `reqwest` library. It handles the response, parses it into the
/// specified type, and returns it wrapped in a `Return` type.
///
/// ### Usage
///
/// ```no_run
/// use atlas_api::client::Client;
/// use atlas_api::core::credits::Transaction;
/// use atlas_api::errors::APIError;
/// use atlas_api::request::Callable;
/// use atlas_api::request::Return;
///
/// fn main() -> Result<(), APIError> {
/// let c = Client::new();
///     let query = vec!["country_code=fr"];
///
///     let res: Return<Transaction> = c
///         .credits()
///         .list(query)
///         .with([("type", "transaction")])
///         .call()?;
///
///     println!("{:?}", res);
///     Ok(())
/// }
/// ```
///
/// ### Errors
///
/// Returns an `APIError` if the request fails for reasons such as:
///
/// - Invalid URL construction.
/// - HTTP client issues such as a timeout or request failure.
/// - Parsing the response body fails due to unexpected or invalid data.
///
/// ### Notes
///
/// - The `ctx` is used to determine the specific API context for the call.
/// - The `opts` and `query` are merged together to form the full request
///   query parameters.
/// - This function is synchronous and uses the `reqwest::blocking` API for
///   simplicity. Consider using asynchronous versions for high-performance
///   or latency-sensitive applications.
///
impl<T> Callable<T> for Single
where
    T: DeserializeOwned + Debug,
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
        let url = Url::parse_with_params(format!("{}{}", &self.url.as_str(), add).as_str(), opts)
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
