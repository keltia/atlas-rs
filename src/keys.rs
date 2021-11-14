//! Struct and methods to deal with probes
//!

use reqwest::StatusCode;
/// External crates
use serde::{Deserialize, Serialize};

/// Our crates
use crate::client::Client;
use crate::common::add_opts;
use crate::errors::*;

/// This is the structure describing an API key with its validity, entitlements, etc.
///
#[derive(Serialize, Deserialize, Debug)]
pub struct Key {
    /// Main ID is an uuid
    pub uuid: String,
    /// Key validity from...
    pub valid_from: String,
    /// Key validity to
    pub valid_to: String,
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

/// Each permission is for a given target
#[derive(Serialize, Deserialize, Debug)]
pub struct Target {
    #[serde(rename = "type")]
    pub ttype: String,
    pub id: String,
}

/// This is to describe all the entitlements of a given key
#[derive(Serialize, Deserialize, Debug)]
pub struct Grant {
    pub permission: String,
    pub target: Target,
}

/// When asking for a list of keys, this struct is used for pagination
#[derive(Serialize, Deserialize, Debug)]
pub struct KeyList {
    /// How many results in this block
    pub count: u32,
    /// URL to fetch the next block
    pub next: String,
    /// URL to fetch previous block
    pub previous: String,
    /// Current Probe Block
    pub keys: Vec<Key>,
}
