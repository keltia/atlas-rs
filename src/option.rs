//! We define our own set of options to simplify the code dealing with them.
//!

// Std library
use std::collections::hash_map::{IntoValues, Iter, Keys, Values, ValuesMut};
use std::collections::HashMap;
use std::iter::{FromIterator, IntoIterator};
use std::ops::{Index, IndexMut};

// External crates

// Our crates
//

/// Our own option type
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Options(HashMap<String, String>);

impl Options {
    #[inline]
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    #[inline]
    pub fn insert(&mut self, k: String, v: String) -> Option<String> {
        self.0.insert(k, v)
    }

    /// Gets an iterator over the keys of the map.
    #[inline]
    pub fn keys(&self) -> Keys<'_, String, String> {
        self.0.keys()
    }

    /// Gets an iterator over the values of the map.
    #[inline]
    pub fn values(&self) -> Values<'_, String, String> {
        self.0.values()
    }

    /// Gets an iterator over the values of the map.
    #[inline]
    pub fn values_mut(&mut self) -> ValuesMut<'_, String, String> {
        self.0.values_mut()
    }

    /// Gets an iterator over the values of the map.
    #[inline]
    pub fn into_values(self) -> IntoValues<String, String> {
        self.0.into_values()
    }

    /// Gets an iterator over the values of the map.
    #[inline]
    pub fn iter(&self) -> Iter<'_, String, String> {
        self.0.iter()
    }

    /// Check is given option exist
    #[inline]
    pub fn contains_key(self, s: &str) -> bool {
        self.0.contains_key(s)
    }

    /// Return the number of options
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Check if there is any option
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Merge another set of option into our own
    ///
    pub fn merge(&mut self, o: &Options) -> &mut Self {
        for (k, v) in o.iter() {
            self.insert(k.clone(), v.clone());
        }
        self
    }
}

impl Default for Options {
    fn default() -> Self {
        Options::new()
    }
}

impl<const N: usize> From<[(&str, &str); N]> for Options {
    /// Used as a shortcut to `from_iter()`
    ///
    /// Example:
    /// ```
    /// # use atlas_rs::option::Options;
    /// let o = Options::from([("foo", "bar"), ("baz", "nope")]);
    ///
    /// assert_eq!(2, o.len());
    /// assert_eq!("bar".to_string(), o["foo"]);
    /// ```
    ///
    #[inline]
    fn from(arr: [(&str, &str); N]) -> Self {
        let mut h = HashMap::new();
        for (k, v) in arr.iter() {
            h.insert(k.to_string(), v.to_string());
        }
        Options(h)
    }
}

impl<'a> From<(&'a str, &'a str)> for Options {
    fn from(tpl: (&'a str, &'a str)) -> Self {
        Options::from([tpl])
    }
}

impl<'a> FromIterator<(&'a str, &'a str)> for Options {
    fn from_iter<T: IntoIterator<Item=(&'a str, &'a str)>>(iter: T) -> Self {
        let mut h = HashMap::new();
        for (k, v) in iter {
            h.insert(k.to_string(), v.to_string());
        }
        Options(h)
    }
}

impl<'a> IntoIterator for &'a Options {
    type Item = (&'a String, &'a String);
    type IntoIter = Iter<'a, String, String>;

    fn into_iter(self) -> Iter<'a, String, String> {
        self.0.iter()
    }
}

/// Implement `Index` on `Options` for accessing list elements.
///
impl Index<&str> for Options {
    type Output = String;

    /// Example:
    /// ```
    /// # use atlas_rs::option::Options;
    /// let mut o = Options::from([("foo", "bar")]);
    ///
    /// println!("{}", o["foo"]);
    /// ```
    ///
    #[inline]
    fn index(&self, index: &str) -> &Self::Output {
        &self.0[&index.to_string()]
    }
}

/// Implement `IndexMut` on `Options` for accessing list elements as mutable objects.
///
impl IndexMut<&str> for Options {
    /// Access elements as mutable
    ///
    /// XXX If an element is not present, it will create it.
    ///
    /// Example:
    /// ```
    /// # use atlas_rs::option::Options;
    /// let mut o = Options::new();
    ///
    /// o["foo"] = "blah".to_string();
    /// ```
    ///
    #[inline]
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        let me = self.0.get_mut(index);
        if me == None {
            self.0.insert(index.to_string(), "".to_string());
        }
        self.0.get_mut(index).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::option::Options;

    #[test]
    fn test_options_merge() {
        let mut s1 = Options::from([("foo", "bar")]);
        let s2 = Options::from([("baz", "blah")]);
        let s = Options::from([("foo", "bar"), ("baz", "blah")]);

        let r = s1.merge(&s2);
        assert_eq!(s, *r);
    }

    #[test]
    fn test_index() {
        let o = Options::from([("foo", "bar")]);

        assert_eq!("bar", o["foo"]);
    }

    #[test]
    fn test_index_mut() {
        let mut o = Options::from([("foo", "bar")]);

        o["foo"] = "blah".to_string();
        assert_eq!("blah", o["foo"]);

        o["baz"] = "hello".to_string();
        assert_eq!("hello", o["baz"]);
    }
}
