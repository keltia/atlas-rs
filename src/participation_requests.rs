//! Structs and methods to deal with participation requests
//!

// We have the following call tree:
//
// ----- /participation-requests ----- /list

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
pub fn set_url(op: Op, data: u32) -> String {
    match op {
        Op::Get => format!("/participation-requests/{}/", data.into()),      // /list
        _ => panic!("not possible"),
    }
}

/// Struct describing all data about a given anchor
///
#[derive(Serialize, Deserialize, Debug)]
pub struct ParticipationRequests {
    pub requested: u32,
    #[serde(rename = "type")]
    pub rtype: String,
    pub value: String,
    pub action: String,
    pub tags_include: String,
    pub tags_exclude: String,
    pub id: u32,
    pub created_at: u32,
}


