//! Structs and methods to deal with measurements
//!

// We have the following call tree:
//
// ----- /measurements
//      ----- /list
//      ----- /create
//      ----- /get
//      ----- /update
//      ----- /delete

// -------------------------------------------------------------------------
// Standard library
use std::fmt;
use std::fmt::Formatter;

// External crates
use serde::{Deserialize, Serialize};

// Our crates
use crate::probes::Geometry;
use crate::request::{Op, Param, RequestBuilder};

// -------------------------------------------------------------------------

/// All operations available
#[derive(Debug)]
pub enum Ops {
    Create,
    Delete,
    Get,
    List,
    Update,
}

/// Dispatch table for the measurements ops.
///
impl Ops {
    /// Generate the proper URL for the service we want in the given category
    ///
    pub fn set_url(self, op: Op, uuid: String) -> String {
        match self {
            Ops::Create => unimplemented!(),
            Ops::Delete => unimplemented!(),
            Ops::Get => format!("/measurements/{}/", uuid), // /get
            Ops::List => "/measurements/".to_string(),      // /list
            Ops::Update => unimplemented!(),
        }
    }
}

/// Struct describing all data about a given measurement
///
#[derive(Serialize, Deserialize, Debug)]
pub struct Measurement {

}
