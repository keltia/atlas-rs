//! We define our own set of options to simplify the code dealing with them.
//!

// Std library
use std::collections::HashMap;
use std::iter::FromIterator;
use std::array::IntoIter;

// External crates
use itertools::Itertools;

/// Our own option type
#[derive(Debug)]
pub struct Options<'o> { h: HashMap<&'o str, &'o str> }

impl<K, V, const N: usize> From<[(K, V); N]> for Options<'_> {
    fn from(a: [(K, V); N]) -> Self {
        std::array::IntoIter::new(a).collect()
    }
}

impl<'o> Options<'o> {
    #[inline]
    pub fn insert(&mut self, k: &'o str, v: &'o str) -> Option<&'o str> {
        self.h.insert(k, v)
    }

    /// Gets an iterator over the keys of the map.
    #[inline]
    pub fn keys<K, V>(&self) -> Keys<'_, K, V> {
        Keys {
            iter: self.h.keys()
        }
    }
}

impl<'o> FromIterator<(&'o str, &'o str)> for Options<'o> {
    fn from_iter<T>(iter: T) -> Self
        where
            T: IntoIterator<Item = (&'o str, &'o str)>,
    {
        Options {
            h: FromIterator::from_iter(iter),
        }
    }
}

/// An iterator over an Options  keys.
pub struct Keys<'a, K, V> {
    iter: KeysImpl<'a, K, V>,
}

type KeysImpl<'a, K, V> = HashMap<K,V>::Keys<'a>;

//delegate_iterator!((Keys<'a>) => &'a str);

/// Take an url and a set of options to add to the parameters
///
/// Example!
/// ```no_run
/// # use atlas_rs::option::{add_opts, Options};
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
    use crate::option::{add_opts, Options};

    #[test]
    fn test_add_opts() {
        let url = "/hello".to_string();
        let o = Options::from([("name", "foo"), ("bar", "baz")]);

        let url = add_opts(&url, &o);
        assert_eq!("/hello?bar=baz&name=foo", url);
    }
}
