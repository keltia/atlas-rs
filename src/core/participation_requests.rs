//! Structs and methods to deal with participation requests
//!

// We have the following call tree:
//
// ----- /participation-requests ----- /list

// -------------------------------------------------------------------------
// Standard library
use std::fmt::Display;

// External crates
use serde::{Deserialize, Serialize};

// Our crates
use crate::common::Routing;
use crate::request::Op;

// -------------------------------------------------------------------------

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

impl<T: Display> Routing<T> for ParticipationRequests {
    /// Generate the proper URL for the service we want in the given category
    ///
    fn set_url(op: Op, data: T) -> String {
        match op {
            Op::Get => format!("/participation-requests/{}/", data),      // /list
            _ => panic!("not possible"),
        }
    }
}
