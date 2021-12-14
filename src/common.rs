//! Some commonly used functions
//!
//! Here we have an implementation of a generic paginator

// Standard library

// Our crates
use crate::client::Client;
use crate::errors::APIError;

// External crates
use anyhow::Result;
use lazy_regex::regex;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

/// When asking for a list of S, this generic struct is used for pagination
///
#[derive(Serialize, Deserialize, Debug)]
pub struct List<S> {
    /// How many results in this block
    pub count: u32,
    /// URL to fetch the next block
    pub next: String,
    /// URL to fetch previous block
    pub previous: String,
    /// Current key block
    pub data: Vec<S>,
}

/// Implement a generic fetch_one_page() function
///
/// Example:
/// ```no_run
/// # use atlas_rs::client::Client;
/// # use atlas_rs::common::List;
/// # use atlas_rs::probes::Probe;
///
/// # let c = Client::new();
/// # let url = "https://foo.example.net/";
///
/// let rawlist: List<Probe> = c.fetch_one_page(url, 1)?;
/// if rawlist.next.is_empty() {
/// #
/// }
/// ```
///
impl<'cl> Client<'cl> {
    pub fn fetch_one_page<S>(&self, url: &'cl str, page: usize) -> Result<List<S>, APIError>
    where
        S: DeserializeOwned,
    {
        let url = format!("{}&page={}", url, page);

        let resp = self.agent.as_ref().unwrap().get(&url).send();

        let resp = match resp {
            Ok(resp) => resp,
            Err(e) => {
                let aerr = APIError::new(
                    e.status().unwrap().as_u16(),
                    "Bad",
                    e.to_string().as_str(),
                    "fetch_one_page",
                );
                return Err(aerr);
            }
        };

        // Try to see if we got an error
        match resp.status() {
            StatusCode::OK => {
                // We could use Response::json() here but it consumes the body.
                let r = resp.text()?;
                println!("p={}", r);
                let p: List<S> = serde_json::from_str(&r)?;
                Ok(p)
            }
            _ => {
                let aerr = resp.json::<APIError>()?;
                Err(aerr)
            }
        }
    }
}

/// Get a n URL and parse it to extract the next page number.
///
/// Example:
/// ```no_run
/// # use atlas_rs::common::{get_page_num, List};
/// # use atlas_rs::keys::Key;
/// # use atlas_rs::client::Client;
///
/// let c = Client::new();
/// let url = "https://example.net/api/v2/foo";
/// let rawlist: List<Key> = c.fetch_one_page(url, 1).unwrap();
///
/// let pn = get_page_num(&rawlist.next);
/// if pn != 0 {
///     // do something
/// }
/// ```
///
pub fn get_page_num(url: &str) -> usize {
    let re = regex!(r"page=(\d+)");

    // If None, return 0
    match re.captures(url) {
        None => 0,
        Some(m) => m.get(1).unwrap().as_str().parse::<usize>().unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use crate::common::get_page_num;
    use rstest::rstest;

    #[rstest]
    #[case("", 0)]
    #[case("foo?zorglub=1", 0)]
    #[case("foo&page=0", 0)]
    #[case("foo&page=1", 1)]
    #[case("foo&page=n", 0)]
    fn test_get_page_num(#[case] url: &str, #[case] n: usize) {
        assert_eq!(n, get_page_num(url));
    }
}
