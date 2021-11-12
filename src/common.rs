//! Some commonly used functions
//!

/// Standard library
use std::collections::HashMap;

/// Our crates
use crate::client::Client;

/// External crates
use itertools::Itertools;

impl<'cl> Client<'cl> {}

/// Take an url and a set of options to add to the parameters
pub fn add_opts<'cl>(url: &str, opts: HashMap<&'cl str, &'cl str>) -> String {
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
    use crate::common::add_opts;
    use std::collections::HashMap;

    #[test]
    fn test_add_opts() {
        let url = "/hello".to_string();
        let o = HashMap::from([("name", "foo"), ("bar", "baz")]);

        let url = add_opts(&url, o);
        assert_eq!("/hello?bar=baz&name=foo", url);
    }
}
