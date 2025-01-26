//! Definitions and traits about `APIError`
//!

/// Standard library
use std::fmt;
use std::io;

/// External crates
use anyhow::anyhow;
use serde::{Deserialize, Serialize};

/// Represents an API error returned from the RIPE Atlas API.
///
/// The `APIError` struct acts as a container for the error details returned by
/// the server. It provides metadata such as the HTTP status code, a short title for the error,
/// detailed descriptions, and optional extra details about the error.
///
/// This struct can also be used to create custom errors for internal usage in this library.
///
/// # Fields
///
/// - `error` - The inner `AErr` struct containing details about the error.
///
/// # Examples
///
/// Creating a new `APIError`:
///
/// ```rust
/// use atlas_api::errors::APIError;
///
/// let error = APIError::new(404, "Not Found", "Resource could not be found", "/resource");
/// println!("{:?}", error);
/// ```
///
/// Converting other errors into `APIError`:
///
/// ```rust
/// use std::io;
/// use atlas_api::errors::APIError;
///
/// let io_error = io::Error::new(io::ErrorKind::NotFound, "file not found");
/// let api_error: APIError = io_error.into();
/// println!("{:?}", api_error);
/// ```
///
#[derive(Deserialize, Serialize, Debug)]
pub struct APIError {
    /// Inner error struct.
    pub error: AErr,
}

/// Container struct for encapsulating API errors.
///
/// This struct represents the main structure for handling errors from the API and includes
/// details about the error such as status codes, titles, and additional messages.
///
/// # Fields
///
/// - `status` - The HTTP status code for the error (e.g., 404 for Not Found).
/// - `code` - Application-specific error code for further classification.
/// - `detail` - A detailed message describing the error.
/// - `title` - A short, human-readable title summarizing the error.
/// - `errors` - An optional list of additional error details, represented by `AError`.
///
/// # Example
///
/// ```rust
/// use atlas_api::errors::{AErr, AError, Source};
///
/// let a_err = AErr {
///     status: 404,
///     code: 1234,
///     detail: "The requested resource could not be found".to_string(),
///     title: "Not Found".to_string(),
///     errors: Some(vec![AError {
///         source: Source {
///             pointer: "/some/resource".to_string(),
///         },
///         detail: "Additional details about the error".to_string(),
///     }]),
/// };
///
/// println!("{:?}", a_err);
/// ```
///
#[derive(Deserialize, Serialize, Debug)]
pub struct AErr {
    /// HTTP status code representing the outcome of the request.
    pub status: u16,
    /// Application-specific error code for categorization.
    pub code: u16,
    /// A detailed description of what caused the error.
    pub detail: String,
    /// A concise error title summarizing the issue.
    pub title: String,
    /// Optional additional error details.
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
mod tests {
    use super::*;

    #[test]
    fn test_api_error_new() {
        let error = APIError::new(404, "Not Found", "Resource could not be found", "/resource");

        assert_eq!(error.error.status, 404);
        assert_eq!(error.error.code, 404);
        assert_eq!(error.error.title, "Not Found");
        assert_eq!(error.error.detail, "Resource could not be found");

        let a_error = error.error.errors.expect("Expected Some errors");
        assert_eq!(a_error.len(), 1);
        assert_eq!(a_error[0].detail, "Resource could not be found");
        assert_eq!(a_error[0].source.pointer, "/resource");
    }

    #[test]
    fn test_api_error_default() {
        let error = APIError::default();

        assert_eq!(error.error.status, 500);
        assert_eq!(error.error.code, 500);
        assert_eq!(error.error.title, "Default");
        assert_eq!(error.error.detail, "def");

        let a_error = error.error.errors.expect("Expected Some errors");
        assert_eq!(a_error.len(), 1);
        assert_eq!(a_error[0].detail, "def");
        assert_eq!(a_error[0].source.pointer, "default");
    }

    #[test]
    fn test_api_error_display() {
        let error = APIError::new(404, "Not Found", "Resource could not be found", "/resource");
        assert_eq!(format!("{}", error), "(\"Not Found\")");
    }

    #[test]
    fn test_api_error_from_io_error() {
        use std::io;

        let io_error = io::Error::new(io::ErrorKind::NotFound, "File not found");
        let api_error: APIError = io_error.into();

        assert_eq!(api_error.error.status, 500);
        assert_eq!(api_error.error.title, "I/O error");
        assert!(api_error.error.detail.contains("File not found"));
        assert_eq!(
            api_error.error.errors.unwrap()[0].source.pointer,
            "std::io::error"
        );
    }

    #[test]
    fn test_api_error_from_serde_error() {
        let serde_error = serde_json::from_str::<APIError>("invalid json").unwrap_err();
        let api_error: APIError = serde_error.into();

        assert_eq!(api_error.error.status, 500);
        assert_eq!(api_error.error.title, "json/decode");
        assert!(api_error.error.detail.contains("EOF while parsing"));
        assert_eq!(api_error.error.errors.unwrap()[0].source.pointer, "serde");
    }

    #[test]
    fn test_aerr_fields() {
        let a_err = AErr {
            status: 400,
            code: 1234,
            detail: "Some error occurred".to_string(),
            title: "Invalid Request".to_string(),
            errors: Some(vec![AError {
                detail: "Field X is invalid".to_string(),
                source: Source {
                    pointer: "/field_x".to_string(),
                },
            }]),
        };

        assert_eq!(a_err.status, 400);
        assert_eq!(a_err.code, 1234);
        assert_eq!(a_err.title, "Invalid Request");
        assert_eq!(a_err.detail, "Some error occurred");

        let additional_errors = a_err.errors.expect("Expected Some errors");
        assert_eq!(additional_errors.len(), 1);
        assert_eq!(additional_errors[0].detail, "Field X is invalid");
        assert_eq!(additional_errors[0].source.pointer, "/field_x");
    }

    #[test]
    fn test_a_error_fields() {
        let source = Source {
            pointer: "/some_field".to_string(),
        };
        let a_error = AError {
            detail: "An error occurred.".to_string(),
            source,
        };

        assert_eq!(a_error.detail, "An error occurred.");
        assert_eq!(a_error.source.pointer, "/some_field");
    }
}
