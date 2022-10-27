//! Struct and methods to deal with keys
//!

// We have the following call-tree:
//
//           ----- /keys                   ----- /permissions
//                                         ----- /permissions  ----- P     ---- /targets
//                                         ----- /get
//                                         ----- /set
//                                         ----- /delete
//                                         ----- /list
//                                         ----- /create

// -------------------------------------------------------------------------
// Standard library
use std::fmt;
use std::fmt::{Display, Formatter};

// External crates
use serde::{Deserialize, Serialize};

// Our crates
use crate::param::Param;
use crate::request::Op;

// -------------------------------------------------------------------------

// -------------------------------------------------------------------------

/// This is the structure describing an API key with its validity, entitlements, etc.
///
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Key {
    /// Main ID is an uuid
    pub uuid: String,
    /// Key validity from...
    pub valid_from: Option<String>,
    /// Key validity to
    pub valid_to: Option<String>,
    /// Is this an usable key?
    pub enabled: bool,
    ///  Is it an active one?
    pub is_active: bool,
    /// Creation date
    pub created_at: String,
    /// Key label (see atlas.ripe.net)
    pub label: String,
    /// Entitlements for the key
    pub grants: Vec<Grant>,
    /// Key type
    #[serde(rename = "type")]
    pub ktype: String,
}

/// Implement the Display trait.
///
impl Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

// -------------------------------------------------------------------------

/// Each permission is for a given target
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Target {
    #[serde(rename = "type")]
    pub ttype: String,
    pub id: String,
}

/// This is to describe all the entitlements of a given key
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Grant {
    pub permission: String,
    pub target: Option<Target>,
}

// -------------------------------------------------------------------------

impl Key {
    /// Generate the proper URL for the service we want in the given category
    ///
    pub fn set_url(op: Op, uuid: Param) -> String {
        match op {
            Op::Permissions => "/keys/permissions/".to_string(), // /permissions
            Op::Targets => format!("/keys/permissions/{}/targets/", String::from(uuid)), // /get targets
            Op::Get => format!("/keys/{}/", String::from(uuid)),                         // /get
            Op::Set => format!("/keys/{}/", String::from(uuid)),                         // /set
            Op::Delete => format!("/keys/{}/", String::from(uuid)),                      // /delete
            Op::List => "/keys/".to_string(),                                            // /list
            Op::Create => "/keys/".to_string(),                                          // /create
            _ => panic!("not possible"),
        }
    }
}

// -------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_key() {}
}
