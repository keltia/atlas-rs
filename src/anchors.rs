//! Structs and methods to deal with anchors
//!

// We have the following call tree:
//
//           ----- /anchors                ----- /list  ----- List<A>
//                                         ----- /get  ----- A

// -------------------------------------------------------------------------
// Standard library
use std::fmt;
use std::fmt::Formatter;

// External crates
use serde::{Serialize, Deserialize};

// Our crates
use crate::client::Client;
use crate::probes::Geometry;
use crate::request::{Param, RequestBuilder};

// -------------------------------------------------------------------------

/// All operations available
#[derive(Debug)]
pub enum Ops {
    Get,
    List,
}

/// Generate the proper URL for the service we want in the given category
fn set_url(ops: Ops, id: i64) -> String {
    match ops {
        Ops::Get => format!("/anchors/{}/", id), // /get
        Ops::List => "/anchors/".to_string(),      // /list
    }
}

/// Struct describing all data about a given anchor
///
#[derive(Serialize, Deserialize, Debug)]
pub struct Anchor {
    /// The id of the anchor, XXX
    pub id: i64,
    /// The type of the object,
    #[serde(rename = "type")]
    pub atype: String,
    /// The fully qualified domain name of the anchor,
    pub fqdn:String,
    /// The id of the probe that is hosted on this anchor,
    pub probe: i64,
    /// Is it IPv4-only?
    pub is_ipv4_only: bool,
    /// The IPv4 address (if any) of this anchor,
    pub ip_v4: Option<String>,
    /// The IPv4 AS this anchor belongs to,
    pub as_v4: i64,
    /// The IPv4 gateway address of this anchor,
    pub ip_v4_gateway: String,
    /// The IPv4 netmask for the IP address of this anchor,
    pub ip_v4_netmask: String,
    /// The IPv6 address (if any) of this anchor,
    pub ip_v6: Option<String>,
    /// The IPv6 AS this anchor belongs to,
    pub as_v6: i64,
    /// The IPv6 gateway address of this anchor,
    pub ip_v6_gateway: String,
    /// The IPv6 prefix of this anchor, XXX
    pub ip_v6_prefix: i64,
    /// The city this anchor is located in,
    pub city: String,
    /// An ISO-3166-1 alpha-2 code indicating the country that this probe is located in, as
    /// derived from the user supplied longitude and latitude,
    pub country: String,
    /// A GeoJSON point object containing the location of this anchor. The longitude and latitude
    /// are contained within the `coordinates` array, XXX
    pub geometry: Geometry,
    /// Installed TLSA DNS resource record on this anchor,
    pub tlsa_record: String,
    /// Is it disabled?
    pub is_disabled: bool,
    /// Date the achor went live,
    pub date_live: String,
    /// Version [ 0, 1, 2, 99 ]
    pub hardware_version: i32,
}

impl Anchor {
    pub fn dispatch<'a>(
        r: &'a mut RequestBuilder<'a>,
        ops: Ops,
        data: Param<'a>,
    ) -> &'a mut RequestBuilder<'a> {
        let opts = r.c.opts.clone();
        let add = set_url(ops, data.into());

        let url = reqwest::Url::parse_with_params(
            format!("{}/{}", r.r.url().as_str(), add).as_str(),
            opts.iter(),
        )
            .unwrap();
        r.r = reqwest::blocking::Request::new(r.r.method().clone(), url);
        r
    }
}

/// Implement the Display trait.
///
impl fmt::Display for Anchor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

