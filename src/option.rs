//! We define our own set of options to simplify the code dealing with them.
//!

// Std library
use std::array::IntoIter;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::ops::{Index, IndexMut};

// External crates
use itertools::Itertools;

// Our crates
//
use crate::request::Op;

/// Our own option type
#[derive(Debug)]
pub struct Options<'o>(HashMap<&'o str, &'o str>);

impl<'o, K, V, const N: usize> From<[(K, V); N]> for Options<'o> {
    fn from(arr: [(K, V); N]) -> Self {
        IntoIter::into_iter(arr)
    }
}

impl<'o> Options<'o> {
    #[inline]
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    #[inline]
    pub fn insert(&mut self, k: &'o str, v: &'o str) -> Option<&'o str> {
        self.0.insert(k, v)
    }

    /// Gets an iterator over the keys of the map.
    #[inline]
    pub fn keys<K, V>(&self) -> Keys<'_, K, V> {
        Keys {
            iter: self.0.keys()
        }
    }
}

impl<'o> Iterator for Options<'o> {
    type Item = (&'o str, &'o str);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.into_iter().next()
    }
}

impl<'o> IntoIterator for Options<'o> {
    type Item = (&'o str, &'o str);
    type IntoIter = std::hash::IntoIter<&'o Self::Item>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
}

impl<'o> FromIterator<(&'o str, &'o str)> for Options<'o> {
    fn from_iter<T>(iter: T) -> Self
        where
            T: IntoIterator<Item = (&'o str, &'o str)>,
    {
        Options(FromIterator::from_iter(iter))
    }
}

/// Implement `Index` on `Options` for accessing list elements.
///
impl<'o> Index<&'o str> for Options<'o> {
    type Output = &'o str;

    /// Example:
    /// ```
    /// ```
    ///
    #[inline]
    fn index(&self, index: &'o str) -> &'o Self::Output {
        &self.0[index]
    }
}

/// Implement `IndexMut` on `Options` for accessing list elements as mutable objects.
///
impl<'o> IndexMut<&'o str> for Options<'o> {
    /// Example:
    /// ```
    /// ```
    ///
    #[inline]
    fn index_mut(&'o mut self, index: &'o str) -> &'o mut Self::Output {
        &mut self.0[index]
    }
}


/// An iterator over an Options  keys.
pub struct Keys<'a, K, V> {
    iter: KeysImpl<'a, K, V>,
}

type KeysImpl<'a, K, V> = <HashMap<K,V> as Trait>::Keys<'a>;

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
