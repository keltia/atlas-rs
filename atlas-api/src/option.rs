//! We define our own set of options to simplify the code dealing with them.
//!

// Std library
use std::collections::hash_map::{IntoValues, Iter, Keys, Values, ValuesMut};
use std::collections::HashMap;
use std::iter::{FromIterator, IntoIterator};
use std::ops::{Index, IndexMut};

// External crates

/// The `Options` struct provides a convenient and flexible way to manage key-value pairs
/// as options in the form of a `HashMap<String, String>`. It is particularly useful for
/// passing around sets of options across various functions and components.
///
/// # Examples
///
/// Creating a new `Options` object:
/// ```
/// # use atlas_api::option::Options;
/// let mut options = Options::new();
/// options.insert("key".to_string(), "value".to_string());
/// assert_eq!(options["key"], "value");
/// ```
///
/// Merging two `Options` objects:
/// ```
/// # use atlas_api::option::Options;
/// let mut options1 = Options::from([("foo", "bar")]);
/// let options2 = Options::from([("baz", "qux")]);
///
/// options1.merge(&options2);
/// assert_eq!(options1["baz"], "qux");
/// ```
///
/// Accessing a value by key using indexing:
/// ```
/// # use atlas_api::option::Options;
/// let options = Options::from([("key", "value")]);
/// assert_eq!(options["key"], "value");
/// ```
///
/// Modifying a value by key:
/// ```
/// # use atlas_api::option::Options;
/// let mut options = Options::new();
/// options["key"] = "new_value".to_string();
/// assert_eq!(options["key"], "new_value");
/// ```
///
/// # Features
///
/// - Add, remove, or manipulate key-value pairs
/// - Iterate over keys, values, or key-value pairs
/// - Merge options from another `Options` instance
/// - Access values safely using indexing
///
/// # Notes
/// The `Index` and `IndexMut` implementations on `Options` will panic if an unknown key
/// is accessed. Use the `contains_key` method to verify if a key exists before accessing it
/// directly to avoid panics.
///
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
    pub fn contains_key(&self, s: &str) -> bool {
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

/// Implements the `From` trait on arrays of tuples for creating an `Options` instance.
///
/// This trait allows directly converting an array of key-value pairs (`[(&str, &str)]`) into
/// an `Options` object, which is essentially a wrapper around a HashMap.
///
/// # Examples
///
/// ```
/// # use atlas_api::option::Options;
/// let options = Options::from([("key1", "value1"), ("key2", "value2")]);
///
/// assert_eq!(options["key1"], "value1");
/// assert_eq!(options["key2"], "value2");
/// assert_eq!(options.len(), 2);
/// ```
///
impl<const N: usize> From<[(&str, &str); N]> for Options {
    #[inline]
    fn from(arr: [(&str, &str); N]) -> Self {
        let mut h = HashMap::new();
        for (k, v) in arr.iter() {
            h.insert(k.to_string(), v.to_string());
        }
        Options(h)
    }
}

/// Implements the `From` trait for creating an `Options` instance from a tuple.
///
/// This trait allows directly converting a tuple (`(&str, &str)`) into
/// an `Options` object, which wraps key-value pairs in a HashMap.
///
/// # Examples
///
/// ```
/// # use atlas_api::option::Options;
/// let options = Options::from(("key", "value"));
///
/// assert_eq!(options["key"], "value");
/// assert_eq!(options.len(), 1);
/// ```
///
impl<'a> From<(&'a str, &'a str)> for Options {
    fn from(tpl: (&'a str, &'a str)) -> Self {
        Options::from([tpl])
    }
}

impl<'a> FromIterator<(&'a str, &'a str)> for Options {
    fn from_iter<T: IntoIterator<Item = (&'a str, &'a str)>>(iter: T) -> Self {
        let mut h = HashMap::new();
        for (k, v) in iter {
            h.insert(k.to_string(), v.to_string());
        }
        Options(h)
    }
}

/// Implements the `IntoIterator` trait for `&Options`.
///
/// This allows iterating over key-value pairs within an `Options` instance.
///
/// Example:
/// ```
/// # use std::collections::HashMap;
/// # use atlas_api::option::Options;
/// let o = Options::from([("key1", "val1"), ("key2", "val2")]);
/// for (key, value) in &o {
///     println!("{}: {}", key, value);
/// }
/// ```
///
/// This method returns an iterator over the key-value pairs in the `Options` hashmap.
///
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
    /// # use atlas_api::option::Options;
    /// let mut o = Options::from([("foo", "bar")]);
    ///
    /// println!("{}", o["foo"]);
    /// ```
    ///
    #[inline]
    fn index(&self, index: &str) -> &Self::Output {
        let me = self.0.get(index);
        me.unwrap()
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
    /// # use atlas_api::option::Options;
    /// let mut o = Options::new();
    ///
    /// o["foo"] = "blah".to_string();
    /// ```
    ///
    #[inline]
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        let me = self.0.get_mut(index);
        if me.is_none() {
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
    #[should_panic]
    fn test_index_nok() {
        let o = Options::from([("foo", "bar")]);

        assert!(o["baz"].is_empty());
    }

    #[test]
    fn test_index_mut() {
        let mut o = Options::from([("foo", "bar")]);

        o["foo"] = "blah".to_string();
        assert_eq!("blah", o["foo"]);

        o["baz"] = "hello".to_string();
        assert_eq!("hello", o["baz"]);
    }

    #[test]
    fn test_empty_options() {
        let o = Options::new();
        assert!(o.is_empty());
        assert_eq!(o.len(), 0);
    }

    #[test]
    fn test_contains_key() {
        let o = Options::from([("foo", "bar"), ("baz", "blah")]);
        assert!(o.contains_key("foo"));
        assert!(o.contains_key("baz"));
        assert!(!o.contains_key("nonexistent"));
    }

    #[test]
    fn test_iterate_options() {
        let o = Options::from([("key1", "value1"), ("key2", "value2")]);
        let mut collected: Vec<(&String, &String)> = o.iter().collect();
        collected.sort_by(|a, b| a.0.cmp(b.0));

        assert_eq!(
            collected,
            vec![
                (&"key1".to_string(), &"value1".to_string()),
                (&"key2".to_string(), &"value2".to_string())
            ]
        );
    }

    #[test]
    fn test_merge_overwrites_values() {
        let mut o1 = Options::from([("key1", "value1"), ("key2", "value2")]);
        let o2 = Options::from([("key2", "new_value2"), ("key3", "value3")]);

        o1.merge(&o2);

        assert_eq!(o1["key1"], "value1");
        assert_eq!(o1["key2"], "new_value2");
        assert_eq!(o1["key3"], "value3");
    }

    #[test]
    fn test_from_tuple() {
        let o = Options::from(("foo", "bar"));
        assert_eq!(o["foo"], "bar");
        assert_eq!(o.len(), 1);
    }

    #[test]
    fn test_from_array() {
        let o = Options::from([("key1", "value1"), ("key2", "value2")]);
        assert_eq!(o["key1"], "value1");
        assert_eq!(o["key2"], "value2");
        assert_eq!(o.len(), 2);
    }

    #[test]
    fn test_from_iterator() {
        let vec = vec![("key1", "value1"), ("key2", "value2")];
        let o: Options = vec.into_iter().collect();

        assert_eq!(o["key1"], "value1");
        assert_eq!(o["key2"], "value2");
        assert_eq!(o.len(), 2);
    }

    #[test]
    fn test_into_iter() {
        let o = Options::from([("key1", "value1"), ("key2", "value2")]);
        let mut collected: Vec<(&String, &String)> = o.into_iter().collect();
        collected.sort_by(|a, b| a.0.cmp(b.0));

        assert_eq!(
            collected,
            vec![
                (&"key1".to_string(), &"value1".to_string()),
                (&"key2".to_string(), &"value2".to_string())
            ]
        );
    }
}
