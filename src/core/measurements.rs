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
use std::fmt::{Display, Formatter};

// External crates
use serde::{Deserialize, Serialize};

// Our crates
use crate::common::Routing;
use crate::request::Op;

// -------------------------------------------------------------------------

/// Struct describing all data about a given measurement
///
#[derive(Serialize, Deserialize, Debug)]
pub struct Measurement {}

/// Implement the Display trait.
///
impl Display for Measurement {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

impl<T: Display> Routing<T> for Measurement {
    /// Generate the proper URL for the service we want in the given category
    ///
    fn set_url(op: Op, uuid: T) -> String
    {
        match op {
            Op::Create => unimplemented!(),
            Op::Delete => unimplemented!(),
            Op::Get => format!("/measurements/{}/", uuid), // /get
            Op::List => "/measurements/".to_string(),      // /list
            Op::Update => unimplemented!(),
            _ => panic!("not possible"),
        }
    }
}