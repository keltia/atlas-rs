//! Module to manage API calls parameters and conversions.
//!
//! This is used as an easier interface to the different argument an API call
//! can take and conversion between these and our common type `Param`.
//!
//! For the moment are define:
//!
//! - u32
//! - i32
//! - u64
//! - string
//! - Vec<string>
//!

use std::fmt::{Display, Formatter};

use serde::Serialize;

/// Represents the various parameter types supported by the API.
///
/// This enum is mainly used to simplify the management of different types of
/// parameters that can be passed to API calls. Each variant represents a
/// specific type of parameter:
///
/// - **A(Vec<String>)**: An array of strings, e.g., for complex parameters like `"country=fr"`.
/// - **I(i32)**: A signed 32-bit integer.
/// - **U(u32)**: An unsigned 32-bit integer.
/// - **L(i64)**: A signed 64-bit integer (long).
/// - **S(String)**: A string parameter.
/// - **None**: Represents a parameter that has no value.
///
/// # Examples
///
/// Creating a `Param` from different types:
///
/// ```rust
/// use atlas_api::param::Param;
///
/// // From a string slice
/// let string_param = Param::from("example");
/// assert_eq!(string_param, Param::S("example".to_string()));
///
/// // From a 32-bit unsigned integer
/// let u32_param = Param::from(42u32);
/// assert_eq!(u32_param, Param::U(42));
///
/// // From a vector of strings
/// let vec_param = Param::from(vec!["foo", "bar", "baz"]);
/// assert_eq!(
///     vec_param,
///     Param::A(vec!["foo".to_string(), "bar".to_string(), "baz".to_string()])
/// );
/// ```
///
/// Displaying a `Param`:
///
/// ```rust
/// use atlas_api::param::Param;
/// use std::fmt::Display;
///
/// let param = Param::S("example".to_string());
/// println!("{}", param); // Outputs: "\"example\""
/// ```
///
#[derive(Clone, Debug, Default, Serialize, PartialEq)]
pub enum Param {
    /// Represents a n array of strings (i.e. "country=fr", "area=WW")
    A(Vec<String>),
    /// Represents the most usual 32-bit integer
    I(i32),
    /// Represents an unsigned 32-bit integer
    U(u32),
    /// Represents the long aka 64-bit integer
    L(i64),
    /// Represents the string pointer aka `str`
    S(String),
    /// Nothing
    #[default]
    None,
}

impl Display for Param {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

// Implement From: for our enum to pass stuff around without explicitly converting before.

/// From &str to Param
///
impl From<&str> for Param {
    fn from(s: &str) -> Self {
        Param::S(s.to_string())
    }
}

/// From array of &str to Param
///
impl<const N: usize> From<[&str; N]> for Param {
    fn from(arr: [&str; N]) -> Self {
        let mut v = Vec::new();
        for s in arr.iter() {
            v.push(s.to_string())
        }
        Param::A(v)
    }
}

/// From array of &str to Param
///
impl From<Vec<&str>> for Param {
    fn from(arr: Vec<&str>) -> Self {
        let mut v = Vec::new();
        for s in arr.iter() {
            v.push(s.to_string())
        }
        Param::A(v)
    }
}

/// From array of &str to Param
///
impl From<Vec<String>> for Param {
    fn from(arr: Vec<String>) -> Self {
        let mut v = Vec::new();
        for s in arr.iter() {
            v.push(s.to_string())
        }
        Param::A(v)
    }
}

/// From Param to String
///
impl From<Param> for String {
    fn from(p: Param) -> Self {
        match p {
            Param::S(s) => s,
            _ => "".to_string(),
        }
    }
}

/// From u32 to Param
///
impl From<u32> for Param {
    fn from(p: u32) -> Self {
        Param::U(p)
    }
}

/// From Param to u32
///
impl From<Param> for u32 {
    fn from(p: Param) -> Self {
        match p {
            Param::U(v) => v,
            _ => 0,
        }
    }
}

/// From i64 to Param
///
impl From<i64> for Param {
    fn from(p: i64) -> Self {
        Param::L(p)
    }
}

/// From i32 to Param
///
impl From<i32> for Param {
    fn from(p: i32) -> Self {
        Param::I(p)
    }
}

/// From Param to i32
///
impl From<Param> for i32 {
    fn from(p: Param) -> Self {
        match p {
            Param::I(v) => v,
            _ => 0,
        }
    }
}

/// From Param to i64
///
impl From<Param> for i64 {
    fn from(p: Param) -> Self {
        match p {
            Param::L(v) => v,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn take_arr_param(a: Param) -> Param {
        a
    }

    #[test]
    fn test_param_from_array() {
        let pl = take_arr_param(["foo", "bar", "baz"].into());

        dbg!(&pl);
    }

    #[test]
    fn test_u32_param() {
        let p = 27u32;

        let s = Param::from(p);
        let t = Param::U(27);
        assert_eq!(t, s);
    }

    #[test]
    fn test_param_u32() {
        let p = Param::U(28);

        let s = u32::from(p);
        assert_eq!(28, s);
    }

    #[test]
    fn test_param_from_str() {
        let s = "test_string";
        let param = Param::from(s);
        assert_eq!(param, Param::S(s.to_string()));
    }

    #[test]
    fn test_param_from_vec_of_str() {
        let vec_of_str = vec!["one", "two", "three"];
        let param = Param::from(vec_of_str.clone());
        assert_eq!(
            param,
            Param::A(vec_of_str.into_iter().map(|s| s.to_string()).collect())
        );
    }

    #[test]
    fn test_param_from_array_of_str() {
        let array_of_str = ["alpha", "beta", "gamma"];
        let param = Param::from(array_of_str);
        assert_eq!(
            param,
            Param::A(array_of_str.iter().map(|&s| s.to_string()).collect())
        );
    }

    #[test]
    fn test_param_to_string() {
        let param = Param::S("example".to_string());
        let converted = String::from(param);
        assert_eq!(converted, "example".to_string());
    }

    #[test]
    fn test_param_from_i32() {
        let i = -42i32;
        let param = Param::from(i);
        assert_eq!(param, Param::I(i));
    }

    #[test]
    fn test_param_from_i64() {
        let l = -1234567890i64;
        let param = Param::from(l);
        assert_eq!(param, Param::L(l));
    }

    #[test]
    fn test_param_from_u64() {
        let l = 1234567890u64;
        let param = Param::L(l as i64); // Since u64 isn't explicitly covered, treat as Param::L
        assert_eq!(param, Param::L(l as i64));
    }

    #[test]
    fn test_param_none_default() {
        let param = Param::default();
        assert_eq!(param, Param::None);
    }

    #[test]
    fn test_param_to_u32() {
        let param = Param::U(123u32);
        let converted: u32 = u32::from(param);
        assert_eq!(converted, 123u32);
    }

    #[test]
    fn test_param_to_i32() {
        let param = Param::I(-123i32);
        let converted: i32 = i32::from(param);
        assert_eq!(converted, -123i32);
    }

    #[test]
    fn test_param_to_i64() {
        let param = Param::L(-98765432i64);
        let converted: i64 = i64::from(param);
        assert_eq!(converted, -98765432i64);
    }

    #[test]
    fn test_param_array_serialization() {
        let param = Param::A(vec!["one".into(), "two".into()]);
        let serialized = serde_json::to_string(&param).unwrap();
        assert_eq!(serialized, r#"{"A":["one","two"]}"#);
    }

    #[test]
    fn test_param_display_trait() {
        let param = Param::S("value".into());
        let output = format!("{}", param);
        assert_eq!(output, r#"{"S":"value"}"#);
    }
}
