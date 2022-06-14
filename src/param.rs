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

/// This enum is for passing the right kind of parameter to `get()`,
/// there might be a better way for this.
///
#[derive(Clone, Debug, Serialize, PartialEq)]
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
    None,
}

impl Default for Param {
    fn default() -> Self {
        Param::None
    }
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
}
