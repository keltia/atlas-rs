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

/// Generate the proper URL for the service we want in the given category
///
pub fn set_url(op: Op, uuid: String) -> String {
    match op {
        Op::Create => unimplemented!(),
        Op::Delete => unimplemented!(),
        Op::Get => format!("/measurements/{}/", uuid), // /get
        Op::List => "/measurements/".to_string(),      // /list
        Op::Update => unimplemented!(),
        _ => panic!("not possible"),
    }
}

/// Struct describing all data about a given measurement
///
#[derive(Serialize, Deserialize, Debug)]
pub struct Measurement {}
