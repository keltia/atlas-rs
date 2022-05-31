//! Some commonly used functions
//!
//! Here we have an implementation of a generic paginator

// Standard library

// External crates
use lazy_regex::regex;

// Our crates

/// Get a n URL and parse it to extract the next page number.
///
/// Example:
/// ```no_run
/// # use atlas_rs::common::{get_page_num, List};
/// # use atlas_rs::core::keys::Key;
/// # use atlas_rs::client::Client;
///
/// let c = Client::new();
/// let url = "https://example.net/api/v2/foo".to_string();
/// let rawlist: List<Key> = c.fetch_one_page(url, 1).unwrap();
///
/// let pn = get_page_num(rawlist.next);
/// if pn != 0 {
///     // do something
/// }
/// ```
///
pub fn get_page_num(url: String) -> usize {
    let re = regex!(r"page=(\d+)");

    // If None, return 0
    match re.captures(&url) {
        None => 0,
        Some(m) => m.get(1).unwrap().as_str().parse::<usize>().unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::common::get_page_num;

    #[rstest]
    #[case("", 0)]
    #[case("foo?zorglub=1", 0)]
    #[case("foo&page=0", 0)]
    #[case("foo&page=1", 1)]
    #[case("foo&page=n", 0)]
    fn test_get_page_num(#[case] url: &str, #[case] n: usize) {
        assert_eq!(n, get_page_num(url.to_string()));
    }
}
