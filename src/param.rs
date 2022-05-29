//! Module to manage API calls parameters and conversions.
//!

use std::fmt::{Display, Formatter};

use serde::Serialize;

/// This enum is for passing the right kind of parameter to `get()`,
/// there might be a better way for this.
///
#[derive(Clone, Debug, Serialize)]
pub enum Param {
    /// Represents the most usual 32-bit integer
    I(i32),
    /// Represents an unsigned 32-bit integer
    U(u32),
    /// Represents the long aka 64-bit integer
    L(i64),
    /// Represents the string pointer aka `str`
    S(String),
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

/// From Param to String
///
impl<'a> From<Param> for String {
    fn from(p: Param) -> Self {
        match p {
            Param::S(s) => s,
            _ => "".to_string(),
        }
    }
}

/// From u32 to Param
///
impl<'a> From<u32> for Param {
    fn from(p: u32) -> Self {
        Param::U(p)
    }
}

/// From Param to u32
///
impl<'a> From<Param> for u32 {
    fn from(p: Param) -> Self {
        match p {
            Param::U(v) => v,
            _ => 0,
        }
    }
}

/// From i64 to Param
///
impl<'a> From<i64> for Param {
    fn from(p: i64) -> Self {
        Param::L(p)
    }
}

/// From i32 to Param
///
impl<'a> From<i32> for Param {
    fn from(p: i32) -> Self {
        Param::I(p)
    }
}

/// From Param to i32
///
impl<'a> From<Param> for i32 {
    fn from(p: Param) -> Self {
        match p {
            Param::I(v) => v,
            _ => 0,
        }
    }
}

/// From Param to i64
///
impl<'a> From<Param> for i64 {
    fn from(p: Param) -> Self {
        match p {
            Param::L(v) => v,
            _ => 0,
        }
    }
}
