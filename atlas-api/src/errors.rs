//! Definitions and traits about `APIError`
//!

/// Standard library
use std::fmt;
use std::io;

/// External crates
use anyhow::anyhow;
use serde::{Deserialize, Serialize};

/// `APIError` is used to report API errors but we use it for ourselves
#[derive(Deserialize, Serialize, Debug)]
pub struct APIError {
    /// Inner error struct.
    pub error: AErr,
}

/// Container for errors
#[derive(Deserialize, Serialize, Debug)]
pub struct AErr {
    /// HTTP status code.
    pub status: u16,
    /// Error code.
    pub code: u16,
    /// Detailed error message.
    pub detail: String,
    /// Error short title.
    pub title: String,
    /// We might have more detail messages here.
    pub errors: Option<Vec<AError>>,
}

/// We can have several more specialized messages
#[derive(Deserialize, Serialize, Debug)]
pub struct AError {
    /// Source pointer.
    pub source: Source,
    /// Detailed error.
    pub detail: String,
}

/// We used it to say where the `APIError` is generated
#[derive(Deserialize, Serialize, Debug)]
pub struct Source {
    /// Error "location" whatever that means.
    pub pointer: String,
}

/// Just in case, define our default
impl Default for APIError {
    fn default() -> Self {
        APIError::new(500, "Default", "def", "default")
    }
}

impl APIError {
    /// Generate a properly formatted `APIError`
    ///
    /// Examples:
    /// ```no_run
    /// use atlas_api::errors::APIError;
    ///
    /// let e = APIError::new(501, "NotFound", "some error", "get_probe");
    /// ```
    ///
    #[inline]
    pub fn new(code: u16, title: &str, descr: &str, loc: &str) -> Self {
        APIError {
            error: AErr {
                status: code,
                code,
                detail: descr.to_string(),
                title: title.to_string(),
                errors: Some(vec![AError {
                    detail: descr.to_string(),
                    source: Source {
                        pointer: loc.to_string(),
                    },
                }]),
            },
        }
    }
}

/// Used to display a text version of the error (for `println!` and co)
///
impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?})", self.error.title)
    }
}

/// Convert a regular `std::io::error` into `APIError`
///
impl From<io::Error> for APIError {
    #[inline]
    fn from(error: io::Error) -> Self {
        APIError::new(500, "I/O error", &error.to_string(), "std::io::error")
    }
}

/// Convert a deserialize error from `serde`
///
impl From<serde_json::Error> for APIError {
    #[inline]
    fn from(error: serde_json::Error) -> Self {
        APIError::new(500, "json/decode", &error.to_string(), "serde")
    }
}

/// Convert a deserialize error from `anyhow`
impl From<anyhow::Error> for APIError {
    #[inline]
    fn from(error: anyhow::Error) -> Self {
        APIError::new(500, "json/decode", &error.to_string(), "anyhow")
    }
}

/// Convert a deserialize error from `reqwest`
impl From<reqwest::Error> for APIError {
    #[inline]
    fn from(error: reqwest::Error) -> Self {
        APIError::new(500, "reqwest", &error.to_string(), "reqwest")
    }
}

/// Convert our APIError into an anyhow one
impl From<APIError> for anyhow::Error {
    #[inline]
    fn from(aerr: APIError) -> Self {
        anyhow!(aerr)
    }
}

#[cfg(test)]
mod tests {}
