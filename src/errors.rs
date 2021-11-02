//! Definitions and traits about `APIError`
//!
use std::fmt;
use std::io;

use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct AErr {
    status: u32,
    code: u32,
    detail: String,
    title: String,
    errors: Vec<AError>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Source {
    pointer: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AError {
    source: Source,
    detail: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct APIError {
    err: AErr,
}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?})", self.err.title)
    }
}

impl From<io::Error> for APIError {
    fn from(error: io::Error) -> Self {
        APIError{
            err: AErr {
                status: 500,
                code: 500,
                detail: error.to_string(),
                title: "APIError/std::io".to_string(),
                errors: vec!(
                    AError {
                        detail: error.to_string(),
                        source: Source {
                            pointer: "".to_string(),
                        },
                    },
                ),
            },
        }
    }
}

impl From<serde_json::Error> for APIError {
    fn from(error: serde_json::Error) -> Self {
        APIError{
            err: AErr {
                status: 500,
                code: 500,
                detail: error.to_string(),
                title: "APIError/serde".to_string(),
                errors: vec!(
                    AError {
                        detail: error.to_string(),
                        source: Source {
                            pointer: "".to_string(),
                        },
                    },
                ),
            },
        }
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
