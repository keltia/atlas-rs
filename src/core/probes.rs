//! Struct and methods to deal with probes
//!
//! Probes are one of the main objects you deal with in the API.  These devices are
//! in various places (homes, datacenters, etc.) and are used for measurements, some
//! initiated by the probes themselves and some user-generated ones.
//!
//! Measurements are not handled by this part of the API, please see `measurements.rs` for this.
//!

// We have the following call tree:
//
//           ----- /probes                 ----- /get/ID
//                                         ----- /list
//                                         ----- /set
//                                         ----- /update
//                                         ----- P             ----- /measurements
//                                         ----- /archive
//                                         ----- /rankings
//                                         ----- /tags
//                                         ----- /tags         ----- /slugs

// -------------------------------------------------------------------------

// std library
//
use std::fmt;
use std::fmt::{Display, Formatter};

// External crates
//
use serde::{Deserialize, Serialize};

// Our crates
//
use crate::core::param::Param;
use crate::request::Op;

// -------------------------------------------------------------------------

/// Geolocation as reported by the probe
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Geometry {
    #[serde(rename = "type")]
    pub gtype: String,
    /// GPS coordinates
    pub coordinates: Vec<f64>,
}

/// Describes the current status of the probe
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Status {
    /// Date
    pub since: Option<String>,
    /// Status ID
    pub id: u32,
    /// Status: connected, etc.
    pub name: String,
}

/// Tags about the probe, most generated by the API, some you can add
///
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Tag {
    /// free-text Name like "system: IPv4 works"
    pub name: String,
    /// Value like "system-ipv4-works"
    pub slug: String,
}

/// All information about a given probe
/// Some fields are `Option` either because in some cases (like without an API key), information
/// is masked or just could be empty and deserialisation would fail.
///
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Probe {
    /// IPv4 address
    pub address_v4: Option<String>,
    /// IPv6 address
    pub address_v6: Option<String>,
    /// IPv4 Autonomous System ID
    pub asn_v4: Option<u32>,
    /// IPv6 Autonomous System ID
    pub asn_v6: Option<u32>,
    /// ISO 3166 Country Code
    pub country_code: Option<String>,
    /// Free text description
    pub description: Option<String>,
    /// First connection
    pub first_connected: Option<u32>,
    /// Approx Position
    pub geometry: Option<Geometry>,
    /// Probe ID
    pub id: u32,
    /// Is it an Anchor?
    pub is_anchor: bool,
    /// Is it public?
    pub is_public: bool,
    /// POSIX time since last connect
    pub last_connected: Option<u32>,
    /// IPv4 Network Prefix
    pub prefix_v4: Option<String>,
    /// IPv6 Network Prefix
    pub prefix_v6: Option<String>,
    /// Probe Status
    pub status: Status,
    /// Integer time
    pub status_since: Option<u32>,
    /// System and User tags
    pub tags: Vec<Tag>,
    /// Total uptime
    pub total_uptime: u32,
    /// Probe Type
    #[serde(rename = "type")]
    pub ptype: String,
}

/// Implement fmt::Display for Probe
impl Display for Probe {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

// -------------------------------------------------------------------------

/// When asking for a list of probes, this struct is used for pagination
///
#[derive(Serialize, Deserialize, Debug)]
pub struct ProbeList {
    /// How many results in this block
    pub count: u32,
    /// URL to fetch the next block
    pub next: String,
    /// URL to fetch previous block
    pub previous: String,
    /// Current Probe Block
    pub probes: Vec<Probe>,
}

// -------------------------------------------------------------------------

impl Probe {
    /// Generate the proper URL for the service we want in the given category
    ///
    pub fn set_url(op: Op, p: Param) -> String {
        match op {
            // Get the parameter as a vec of string, transforming into string
            Op::List => {
                let qs = match p {
                    Param::A(v) => v.join("&"),
                    _ => unimplemented!(),
                };
                format!("{}?{}", "/probes/", qs)
            } // /list
            Op::Get => format!("/probes/{}/", u32::from(p)), // /get
            Op::Set => format!("/probes/{}/", u32::from(p)), // /set
            Op::Update => format!("/probes/{}/", u32::from(p)), // /update
            Op::Measurement => format!("/probes/{}/measurements/", u32::from(p)), // P/measurements
            Op::Archive => "/probes/archive/".to_string(),   // /archive
            Op::Rankings => "/probes/rankings/".to_string(), // rankings
            Op::Tags => "/probes/tags/".to_string(),         // /tags/
            Op::Slugs => format!("/probes/tags/{}/slugs", u32::from(p)), // /tags/T/slugs/
            _ => panic!("not possible"),
        }
    }
}
// -------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_get_probe() {}
}
