//! Module implementing the `Paged` type of requests, it basically loops over the results
//! and returns a single vector.
//!

use std::fmt::Debug;
use std::slice::Iter;

use reqwest::{Method, Url};
use serde::de::DeserializeOwned;
use serde::Deserialize;

use crate::client::{Client, Ctx, ENDPOINT};
use crate::errors::APIError;
use crate::option::Options;
use crate::param::Param;
use crate::request::{get_ops_url, Callable, Op, RequestBuilder, Return};

// ------------------------------------------------------------

/// A paginated list structure used for handling paginated API responses.
///
/// The `List` struct represents a single page of results from an API call. It is
/// generic over the type of items contained in the `results` field. It also includes
/// metadata such as links to the next and previous pages, allowing for manual or
/// programmatic pagination.
///
/// # Fields
///
/// * `count` - An optional number indicating the count of results in the current block.
/// * `next` - An optional string containing the URL to fetch the next block of results.
/// * `previous` - An optional string containing the URL to fetch the previous block of results.
/// * `results` - A vector containing the actual results of type `T`.
///
/// # Example
///
/// ```
/// use atlas_api::request::paged::List;
///
/// #[derive(Debug, serde::Deserialize, Clone)]
/// struct Item {
///     id: u32,
///     name: String,
/// }
///
/// let page = List::<Item> {
///     count: Some(2),
///     next: Some(String::from("https://api.example.com/items?page=2")),
///     previous: None,
///     results: vec![
///         Item { id: 1, name: String::from("Item 1") },
///         Item { id: 2, name: String::from("Item 2") },
///     ],
/// };
///
/// for item in page.iter() {
///     println!("{:?}", item);
/// }
/// ```
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
where
    T: DeserializeOwned + Debug + Clone,
{
    /// Gets an iterator over the values of the map.
    #[inline]
    pub fn iter(&self) -> Iter<'_, T> {
        self.results.iter()
    }
}

///
/// Implements the `IntoIterator` trait for a reference to a `List<T>`.
///
/// This allows you to iterate over references to the elements in the `List`.
///
/// # Examples
///
/// ```
/// use atlas_api::request::paged::List;
///
/// #[derive(Debug, serde::Deserialize, Clone)]
/// struct Item {
///     id: u32,
///     name: String,
/// }
///
/// let page = List::<Item> {
///     count: Some(2),
///     next: Some(String::from("https://api.example.com/items?page=2")),
///     previous: None,
///     results: vec![
///         Item { id: 1, name: String::from("Item 1") },
///         Item { id: 2, name: String::from("Item 2") },
///     ],
/// };
///
/// for item in &page {
///     println!("{:?}", item);
/// }
/// ```
///
impl<'a, T> IntoIterator for &'a List<T>
where
    T: DeserializeOwned + Debug + Clone,
{
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.results.iter()
    }
}

/// Converts the `List` into an iterator.
///
/// This method consumes the `List` and creates an iterator over the results, allowing
/// for iteration through the items of the paginated list. The consuming nature of this
/// method means the original list is invalidated after it is called.
///
/// # Examples
///
/// ```
/// use atlas_api::request::paged::List;
///
/// #[derive(Debug, serde::Deserialize, Clone)]
/// struct Item {
///     id: u32,
///     name: String,
/// }
///
/// let page = List::<Item> {
///     count: Some(2),
///     next: Some(String::from("https://api.example.com/items?page=2")),
///     previous: None,
///     results: vec![
///         Item { id: 1, name: String::from("Item 1") },
///         Item { id: 2, name: String::from("Item 2") },
///     ],
/// };
///
/// let mut iter = page.into_iter();
/// while let Some(item) = iter.next() {
///     println!("{:?}", item);
/// }
/// ```
///
impl<T> List<T>
where
    T: DeserializeOwned + Debug + Clone,
{
    /// Creates a consuming iterator, which moves the results out of the List.
    pub fn into_iter(self) -> impl Iterator<Item = T> {
        self.results.into_iter()
    }
}

/// The `Paged` struct represents a configuration to control and fetch paginated results from an API.
///
/// It encapsulates all necessary details, such as context, HTTP options, query parameters,
/// and the base URL required to execute requests.
///
/// ## Fields
///
/// - `ctx: Ctx`
///   - Represents the context of the API request (e.g., specific API endpoints such as `probe/`).
///
/// - `opts: Options`
///   - Provides options configured as a combination of CLI input and default settings.
///
/// - `query: Param`
///   - Specifies query parameters for the API request. Typically used to filter or query specific data.
///
/// - `m: Method`
///   - Defines the HTTP method (GET, POST, etc.) used for the request.
///
/// - `url: Url`
///   - The base `Url` used to construct the final API endpoint.
///
/// - `c: Client`
///   - Represents the HTTP client used to make requests.
///
/// - `op: Op`
///   - Indicates the API operation being performed.
///
/// ## Default Implementation
///
/// The `Paged` struct implements the `Default` trait, which provides a sensible default configuration:
///
/// - `ctx`: Defaults to `Ctx::None`.
/// - `c`: Uses a new instance of the HTTP client.
/// - `opts`: Initialized with default options.
/// - `query`: Set to `Param::None`.
/// - `m`: Defaults to the `GET` method.
/// - `url`: Initialized with a default endpoint (`ENDPOINT.parse().unwrap()`).
/// - `op`: Defaults to `Op::Null`.
///
/// ## Examples
///
/// ### Example 1: Creating a Default Instance of `Paged`
///
/// ```
/// use atlas_api::request::paged::Paged;
///
/// let paged = Paged::default();
/// println!("{:?}", paged);
/// ```
///
/// ### Example 2: Customizing `Paged` with Options
///
/// ```no_run
/// use atlas_api::client::Client;
/// use atlas_api::request::paged::Paged;
///
/// let c = Client::new();
/// let query = vec!["country_code=fr"];
///
/// let paged = Paged::default()
///     .with([("filter", "foo"), ("page_size", "10")]);
/// ```
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
    /// API Operation
    pub op: Op,
}

impl Default for Paged {
    fn default() -> Self {
        Paged {
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

impl Paged {
    /// Makes it easy to specify options
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use atlas_api::client::Client;
    /// use atlas_api::core::probes::Probe;
    ///
    /// let c = Client::new();
    /// let query = vec!["country_code=fr"];
    ///
    /// let res= c.probe().list(query).with([("opt1", "foo"), ("opt2", "bar")]);
    /// ```
    /// This can be used to have subcommands like this:
    /// ```no_run
    /// # use atlas_api::client::Client;
    /// # use atlas_api::core::credits::Transaction;
    ///
    /// let c = Client::new();
    /// let query = vec!["country_code=fr"];
    ///
    /// let res = c.credits().list(query).with(("type", "transaction"));
    /// ```
    ///
    pub fn with(mut self, opts: impl Into<Options>) -> Self {
        self.opts.merge(&opts.into());
        dbg!(&self);
        self
    }

    /// Fetches a single page of results from the API.
    ///
    /// This function is designed to fetch a single paginated response from the API
    /// and deserialize the JSON response into a custom type. It returns a `List<T>`,
    /// where `T` is the type of objects contained in the paginated results.
    ///
    /// ### Parameters:
    /// - `url`: The target URL to make the HTTP request to.
    ///
    /// ### Type Parameters:
    /// - `T`: The type of the objects in the paginated results. Requires:
    ///   - `T: DeserializeOwned`: The type must be deserializable from JSON.
    ///   - `T: Debug`: For debugging purposes.
    ///   - `T: Clone`: To support cloning objects if necessary.
    ///
    /// ### Returns:
    /// - `Ok(List<T>)`: If the request is successful and the data is deserialized correctly.
    /// - `Err(APIError)`: If there is an error during the request, status check, or deserialization.
    ///
    /// ### Errors:
    /// - Returns an `APIError` for any of the following:
    ///   - Network issues when making the request.
    ///   - Invalid response status from the server.
    ///   - Deserialization errors if the response body cannot be converted into `List<T>`.
    ///
    /// ### Example:
    /// ```no_run
    /// use reqwest::Url;
    /// use atlas_api::request::paged::{Paged, List};
    /// use atlas_api::core::probes::Probe;
    ///
    /// let url = Url::parse("https://foo.example.net/").unwrap();
    /// let paged: Paged = Paged::default();
    ///
    /// let result: Result<List<Probe>, _> = paged.fetch_one_page(url);
    /// match result {
    ///     Ok(list) => {
    ///         if list.next.is_some() {
    ///             println!("More pages to fetch!");
    ///         }
    ///         println!("Page data: {:?}", list.results);
    ///     }
    ///     Err(e) => eprintln!("Error fetching data: {}", e),
    /// }
    /// ```
    ///
    pub fn fetch_one_page<T>(&self, url: Url) -> Result<List<T>, APIError>
    where
        T: DeserializeOwned + Debug + Clone,
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

/// Converts a `RequestBuilder` instance into a `Paged` structure.
///
/// This transformation allows seamless chaining of operations between `RequestBuilder`
/// and the pagination processing logic encapsulated within the `Paged` structure.
///
/// ### Parameters:
/// - `rb`: The `RequestBuilder` instance containing the necessary context,
///   client configuration, and request options to construct a `Paged` instance.
///
/// ### Returns:
/// - A new `Paged` instance initialized with the data from the `RequestBuilder`.
///
/// ### Example:
/// ```no_run
/// # use atlas_api::request::paged::Paged;
/// # use atlas_api::request::RequestBuilder;
/// #
/// # let rb = RequestBuilder::dummy(); // Example: A placeholder for demonstration.
/// let paged: Paged = rb.into();
/// dbg!(&paged);
/// ```
///
impl From<RequestBuilder> for Paged {
    /// Makes chaining easier.
    ///
    fn from(rb: RequestBuilder) -> Self {
        Paged {
            ctx: rb.ctx,
            c: rb.c.clone(),
            opts: rb.c.opts.clone(),
            query: rb.query.clone(),
            url: rb.url.clone(),
            m: rb.kw.clone(),
            op: rb.op,
        }
    }
}

/// Implements the `Callable` trait for the `Paged` structure.
///
/// This implementation is responsible for executing the main logic
/// to fetch paginated results from the API, processing the results
/// while handling errors along the way.
///
/// ### Logic:
/// 1. Checks for the "type" option in the provided options and adjusts the operation accordingly.
/// 2. Filters out the "type" option from the options to avoid sending it in API queries.
/// 3. Constructs the target URL for the API call with any necessary URL parameters.
/// 4. Initiates the first API call using the `fetch_one_page` function.
/// 5. Handles errors or cases where no data is returned from the API.
/// 6. Aggregates all results from the paginated response into a single collection.
/// 7. Verifies that the result count matches the expected total, ensuring data consistency.
///
/// ### Example:
/// ```no_run
/// # use atlas_api::client::{Client, Ctx};
/// # use atlas_api::core::probes::Probe;
/// # use atlas_api::request::paged::Paged;
/// #
/// # let c = Client::new();
/// # let ctx = Ctx::None;
///
/// let request = Paged::new(ctx, c, "https://foo.example.net/api/endpoint", None);
/// let result = request.call::<Probe>();
///
/// match result {
///     Ok(data) => {
///         for item in data.results.iter() {
///             println!("{:?}", item);
///         }
///     },
///     Err(err) => {
///         eprintln!("Error occurred: {}", err);
///     },
/// }
/// ```
///
/// ### Requirements:
/// - The type `T` must implement both the `DeserializeOwned` and the `Debug` traits to be
///   processed by the API client.
/// - The operation assumes that the target API follows a paginated format with "next" pointers
///   and a known data count.
///
/// ### Errors:
/// - Returns an `APIError` in the following cases:
///   - API response status is not `200 OK`.
///   - Serialization or deserialization issues while parsing the API response.
///   - No data is returned from the API with a valid "next" pointer.
///
impl<T> Callable<T> for Paged
where
    T: DeserializeOwned + Debug + Clone,
{
    /// Single most important call for the whole structure
    ///
    /// Executes the `Paged` call and retrieves the paginated results.
    ///
    /// This method is responsible for executing the logic encapsulated within the `Paged` struct
    /// to fetch paginated results from the target API. It supports handling custom type-based filtering
    /// mechanisms and fetches, processes, and aggregates the results into a single collection.
    ///
    /// ### Steps performed by this method:
    /// 1. **Operation Adjustment**: Adjusts the operation (`Op`) based on the `type` parameter in the options.
    ///    Removes the `type` parameter to avoid sending it in the query.
    /// 2. **Build URL**: Constructs the target URL with additional query parameters.
    /// 3. **Fetch First Page**: Calls the `fetch_one_page` method to retrieve the first batch of results.
    /// 4. **Error Handling**: Handles cases where no data is returned or an error occurs during the call.
    /// 5. **Fetch Remaining Pages**: Continues fetching subsequent pages if a `next` pointer is present in the response.
    /// 6. **Result Verification**: Verifies that the total count of items matches the expected result count.
    ///
    /// ### Returns:
    /// - `Ok(Return<T>)`: A successful result with all aggregated data.
    /// - `Err(APIError)`: Returns an error if any issues occur during API processing or validation.
    ///
    /// ### Requirements:
    /// - The type `T` must implement the `DeserializeOwned`, `Debug`, and `Clone` traits as these are essential
    ///   for serialization, debugging, and cloning operations.
    ///
    /// ### Errors:
    /// - Returns `APIError` if:
    ///   - API response is invalid or status is not `200 OK`.
    ///   - The API returns no data for valid pagination pointers.
    ///   - Unexpected serialization or deserialization issues occur.
    ///
    /// ### Example:
    /// ```no_run
    /// # use atlas_api::client::{Client, Ctx};
    /// # use atlas_api::request::paged::Paged;
    /// #
    /// # let client = Client::new();
    /// # let context = Ctx::None; // Example: placeholder for context.
    /// #
    /// let request = Paged::new(context, client, "https://foo.example.net/api/endpoint", None);
    /// let result = request.call::<String>(); // Replace `String` with the target deserializable type.
    ///
    /// match result {
    ///     Ok(data) => {
    ///         println!("Fetched {} results:", data.results.len());
    ///         for item in data.results.iter() {
    ///             println!("{:?}", item);
    ///         }
    ///     }
    ///     Err(err) => {
    ///         eprintln!("An API error occurred: {}", err);
    ///     }
    /// }
    /// ```
    ///
    fn call(self) -> Result<Return<T>, APIError> {
        let mut op = self.op.clone();

        // Get the potential "type" option
        //
        dbg!(&self.opts);
        if self.opts.contains_key("type") {
            // Now, check the "type" value
            //
            op = match self.opts["type"].as_str() {
                // Credits stuff
                "expense-items" => Op::Expenses,
                "income-items" => Op::Incomes,
                "members" => Op::Members,
                "transactions" => Op::Transactions,
                "transfer" => Op::Transfers,
                //
                _ => Op::Info,
            };
        }

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

        let query = self.query.to_owned();
        let add = get_ops_url(&self.ctx, op, query);
        dbg!(&add);

        // Setup URL with potential parameters like `key`.
        //
        let url = Url::parse_with_params(format!("{}{}", &self.url.as_str(), add).as_str(), opts)
            .unwrap();

        // Get data / opts for 1st call
        //
        let rawlist: List<T> = match self.fetch_one_page(url) {
            Ok(list) => list,
            Err(e) => return Err(e),
        };

        // Exit early with error if nothing
        //
        if let Some(count) = rawlist.count {
            if count == 0 {
                return Err(APIError::new(
                    400,
                    "Bad Call",
                    "no data returned on pagination",
                    "fetch_one_page",
                ));
            }
        }

        // We will append all results here.
        //
        let mut res = Vec::new();

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

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct TestStruct {
        id: u32,
        name: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct List<T> {
        count: Option<u32>,
        next: Option<String>,
        previous: Option<String>,
        results: Vec<T>,
    }

    #[test]
    fn test_list_basic_functionality() {
        let items = vec![
            TestStruct {
                id: 1,
                name: "Item 1".to_string(),
            },
            TestStruct {
                id: 2,
                name: "Item 2".to_string(),
            },
        ];

        let list = List {
            count: Some(items.len() as u32),
            next: Some("https://example.com/next".to_string()),
            previous: None,
            results: items.clone(),
        };

        assert_eq!(list.count, Some(2));
        assert_eq!(list.next, Some("https://example.com/next".to_string()));
        assert!(list.previous.is_none());
        assert_eq!(list.results, items);
    }

    #[test]
    fn test_list_empty_results() {
        let list: List<TestStruct> = List {
            count: Some(0),
            next: None,
            previous: None,
            results: Vec::new(),
        };

        assert_eq!(list.count, Some(0));
        assert!(list.next.is_none());
        assert!(list.previous.is_none());
        assert!(list.results.is_empty());
    }

    #[test]
    fn test_list_with_pagination() {
        let items_page_1 = vec![TestStruct {
            id: 1,
            name: "Item 1".to_string(),
        }];
        let items_page_2 = vec![TestStruct {
            id: 2,
            name: "Item 2".to_string(),
        }];

        let page_1 = List {
            count: Some(2),
            next: Some("https://example.com/page2".to_string()),
            previous: None,
            results: items_page_1.clone(),
        };

        let page_2 = List {
            count: Some(2),
            next: None,
            previous: Some("https://example.com/page1".to_string()),
            results: items_page_2.clone(),
        };

        assert_eq!(page_1.count, Some(2));
        assert_eq!(page_1.next, Some("https://example.com/page2".to_string()));
        assert!(page_1.previous.is_none());
        assert_eq!(page_1.results, items_page_1);

        assert_eq!(page_2.count, Some(2));
        assert!(page_2.next.is_none());
        assert_eq!(
            page_2.previous,
            Some("https://example.com/page1".to_string())
        );
        assert_eq!(page_2.results, items_page_2);
    }

    #[test]
    fn test_list_deserialization() {
        let json_data = r#"
        {
            "count": 2,
            "next": "https://example.com/next",
            "previous": null,
            "results": [
                { "id": 1, "name": "Item 1" },
                { "id": 2, "name": "Item 2" }
            ]
        }
        "#;

        let deserialized: List<TestStruct> = serde_json::from_str(json_data).unwrap();

        assert_eq!(deserialized.count, Some(2));
        assert_eq!(
            deserialized.next,
            Some("https://example.com/next".to_string())
        );
        assert!(deserialized.previous.is_none());
        assert_eq!(
            deserialized.results,
            vec![
                TestStruct {
                    id: 1,
                    name: "Item 1".to_string()
                },
                TestStruct {
                    id: 2,
                    name: "Item 2".to_string()
                }
            ]
        );
    }

    #[test]
    fn test_list_serialization() {
        let items = vec![
            TestStruct {
                id: 1,
                name: "Item 1".to_string(),
            },
            TestStruct {
                id: 2,
                name: "Item 2".to_string(),
            },
        ];

        let list = List {
            count: Some(items.len() as u32),
            next: Some("https://example.com/next".to_string()),
            previous: None,
            results: items.clone(),
        };

        let serialized = serde_json::to_string(&list).unwrap();
        let expected_json = r#"{"count":2,"next":"https://example.com/next","previous":null,"results":[{"id":1,"name":"Item 1"},{"id":2,"name":"Item 2"}]}"#;

        assert_eq!(serialized, expected_json);
    }

    #[test]
    fn test_paged_with_multiple_pages() {
        let items_page_1 = vec![TestStruct {
            id: 1,
            name: "Item 1".to_string(),
        }];
        let items_page_2 = vec![TestStruct {
            id: 2,
            name: "Item 2".to_string(),
        }];

        let page_1 = List {
            count: Some(2),
            next: Some("https://example.com/page2".to_string()),
            previous: None,
            results: items_page_1.clone(),
        };

        let page_2 = List {
            count: Some(2),
            next: None,
            previous: Some("https://example.com/page1".to_string()),
            results: items_page_2.clone(),
        };

        // Simulate the pagination fetch sequence
        let mut all_results = Vec::new();

        all_results.extend(page_1.results.clone());
        all_results.extend(page_2.results.clone());

        assert_eq!(all_results.len(), 2);
        assert_eq!(
            all_results,
            vec![
                TestStruct {
                    id: 1,
                    name: "Item 1".to_string()
                },
                TestStruct {
                    id: 2,
                    name: "Item 2".to_string()
                }
            ]
        );
    }

    #[test]
    fn test_paged_empty_result() {
        let empty_page: List<TestStruct> = List {
            count: Some(0),
            next: None,
            previous: None,
            results: Vec::new(),
        };

        assert_eq!(empty_page.count, Some(0));
        assert!(empty_page.next.is_none());
        assert!(empty_page.previous.is_none());
        assert!(empty_page.results.is_empty());
    }

    #[test]
    fn test_paged_partial_pages() {
        let items_page_1 = vec![TestStruct {
            id: 1,
            name: "Item 1".to_string(),
        }];
        let items_page_2 = vec![TestStruct {
            id: 2,
            name: "Item 2".to_string(),
        }];
        let items_page_3 = vec![TestStruct {
            id: 3,
            name: "Item 3".to_string(),
        }];

        let page_1 = List {
            count: Some(3),
            next: Some("https://example.com/page2".to_string()),
            previous: None,
            results: items_page_1.clone(),
        };

        let page_2 = List {
            count: Some(3),
            next: Some("https://example.com/page3".to_string()),
            previous: Some("https://example.com/page1".to_string()),
            results: items_page_2.clone(),
        };

        let page_3 = List {
            count: Some(3),
            next: None,
            previous: Some("https://example.com/page2".to_string()),
            results: items_page_3.clone(),
        };

        // Simulating the paging
        let mut all_results = Vec::new();
        all_results.extend(page_1.results.clone());
        all_results.extend(page_2.results.clone());
        all_results.extend(page_3.results.clone());

        assert_eq!(all_results.len(), 3);
        assert_eq!(
            all_results,
            vec![
                TestStruct {
                    id: 1,
                    name: "Item 1".to_string()
                },
                TestStruct {
                    id: 2,
                    name: "Item 2".to_string()
                },
                TestStruct {
                    id: 3,
                    name: "Item 3".to_string()
                }
            ]
        );
    }
}
