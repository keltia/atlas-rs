// Std library
use std::collections::HashMap;
use std::hash::Hash;

// External crates
use itertools::Itertools;

/// Our own option type
pub type Options<'o> = HashMap<&'o str, &'o str>;

/// Take an url and a set of options to add to the parameters
///
/// Example!
/// ```no_run
/// # use atlas_rs::option::{add_opts, Options};
///
/// let url = "https://example.com/";
/// let opts = Options::new().insert("foo", "bar");
/// let url = add_opts(&url, &opts);
/// ```
///
pub fn add_opts<'cl>(url: &str, opts: &Options) -> String {
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
    use crate::option::{Options, add_opts};

    #[test]
    fn test_add_opts() {
        let url = "/hello".to_string();
        let o = Options::from([("name", "foo"), ("bar", "baz")]);

        let url = add_opts(&url, &o);
        assert_eq!("/hello?bar=baz&name=foo", url);
    }
}
