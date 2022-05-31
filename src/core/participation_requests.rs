//! Structs and methods to deal with participation requests
//!

// We have the following call tree:
//
// ----- /participation-requests ----- /list

// -------------------------------------------------------------------------
// Standard library
use std::fmt;
use std::fmt::{Display, Formatter};

// External crates
use serde::{Deserialize, Serialize};

// Our crates
use crate::core::param::Param;
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

impl ParticipationRequests {
    /// Generate the proper URL for the service we want in the given category
    ///
    pub fn set_url(op: Op, data: Param) -> String {
        match op {
            Op::Get => format!("/participation-requests/{}/", data), // /list
            _ => panic!("not possible"),
        }
    }
}

/// Implement fmt::Display for ParticipationRequests
impl Display for ParticipationRequests {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

