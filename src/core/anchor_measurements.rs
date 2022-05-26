//! Structs and methods to deal with anchor measurements
//!

// We have the following call tree:
//
// Atlas API ----- /anchor-measurement     ----- /list  ----- List<AM>
//                                         ----- /get  ----- AM

// Std library
use std::fmt;
use std::fmt::{Display, Formatter};

// External crates
use serde::{Deserialize, Serialize};

// Our own crates
use crate::common::Routing;
use crate::request::{Op, Param, RequestBuilder};

// -------------------------------------------------------------------------

// -------------------------------------------------------------------------

/// Struct describing all data about a given anchor targeted by a measurement
///
#[derive(Serialize, Deserialize, Debug)]
pub struct AnchorMeasurement {
    /// Creation date
    date_created: String,
    /// Last modification date
    date_modified: String,
    /// ID of the target Measurement
    id: u32,
    is_mesh: bool,
    /// URL of the actual measurement
    measurement: String,
    /// URL of the anchor to which the measurement is targeted
    target: String,
    /// Measurement type of the involved measurement
    #[serde(rename = "type")]
    mtype: String,
}

/// Implement the Display trait.
///
impl Display for AnchorMeasurement {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

impl<T> Routing<T> for AnchorMeasurement {
    /// Generate the proper URL for the service we want in the given category
    ///
    fn set_url(op: Op, uuid: T) -> String
    where T: Display,
    {
        match op {
            Op::Get => format!("/anchor-measurements/{}/", uuid), // /get
            Op::List => "/anchor-measurements/".to_string(),      // /list
            _ => panic!("not possible"),
        }
    }
}