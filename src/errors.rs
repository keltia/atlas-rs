//! Definitions and traits about `APIError`
//!

/// Standard library
use std::fmt;
use std::io;

/// External crates
use serde::{Deserialize, Serialize};

/// `APIError` is used to report API errors but we use it for ourselves
#[derive(Deserialize, Serialize, Debug)]
pub struct APIError {
    pub err: AErr,
}

/// Container for errors
#[derive(Deserialize, Serialize, Debug)]
pub struct AErr {
    pub status: u32,
    pub code: u32,
    pub detail: String,
    pub title: String,
    pub errors: Vec<AError>,
}

/// We can have several more specialized messages
#[derive(Deserialize, Serialize, Debug)]
pub struct AError {
    pub source: Source,
    pub detail: String,
}

/// We used it to say where the `APIError` is generated
#[derive(Deserialize, Serialize, Debug)]
pub struct Source {
    pub pointer: String,
}

/// A few helpers for `APIError`
impl APIError {
    /// Generate a properly formatted `APIError`
    ///
    /// Examples:
    /// ```no_run
    /// use atlas_rs::errors::APIError;
    ///
    /// let e = APIError::new(501, "NotFound", "some error", "get_probe");
    /// ```
    ///
    pub fn new(code: u32, title: &str, descr: &str, loc: &str) -> Self {
        APIError {
            err: AErr {
                status: code,
                code: code,
                detail: descr.to_string(),
                title: title.to_string(),
                errors: vec![AError {
                    detail: descr.to_string(),
                    source: Source {
                        pointer: loc.to_string(),
                    },
                }],
            },
        }
    }
}

/// Used to display a text version of the error (for `println!` and co)
impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?})", self.err.title)
    }
}

/// Convert a regular `std::io::error` into `APIError`
impl From<io::Error> for APIError {
    fn from(error: io::Error) -> Self {
        APIError::new(500, "I/O error", &error.to_string(), "std::io::error")
    }
}

/// Convert a deserialize error from `serde`
impl From<serde_json::Error> for APIError {
    fn from(error: serde_json::Error) -> Self {
        APIError::new(500, "json/decode", &error.to_string(), "serde")
    }
}

/// Decode the body returned by the API into a proper `APIError`
pub fn decode_error(body: &str) -> Result<APIError, String> {
    let e: Result<APIError, String> = serde_json::from_str(&body).unwrap();
    match e {
        Ok(ae) => Ok(ae),
        Err(e) => {
            let s = format!("Error decoding {}", e);
            Err(s)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_decode_error_null() {
        let raw = "";
        assert!(decode_error(raw).is_err());
    }
    #[test]
    #[should_panic]
    fn test_decode_error_bad() {
        let raw = "error";
        assert!(decode_error(raw).is_err());
    }
}
