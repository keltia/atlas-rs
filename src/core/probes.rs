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

#[cfg(feature = "flat-api")]
use reqwest::StatusCode;
// External crates
//
use serde::{Deserialize, Serialize};

// Our crates
//
use crate::client::Client;
use crate::core::param::Param;
use crate::request::Op;

// -------------------------------------------------------------------------

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

/// Methods associated with probes.
///
impl Probe {
    /// Alternate API for probes
    ///
    /// Example:
    /// ```no_run
    /// # use atlas_rs::core::probes::Probe;
    /// # use atlas_rs::client::ClientBuilder;
    ///
    /// let c = ClientBuilder::new().api_key("the-key")?;
    /// let p = Probe::get(cl, 666)?;
    /// ```
    ///
    #[cfg(feature = "alt-api")]
    pub fn get(cl: &Client, pn: u32) -> Result<Self, APIError> {
        Ok(cl.probe().get(pn).call()?)
    }

    /// Alternate API for probes
    ///
    /// Example:
    /// ```no_run
    /// # use atlas_rs::core::probes::Probe;
    /// # use atlas_rs::client::ClientBuilder;
    ///
    /// let c = ClientBuilder::new().api_key("the-key")?;
    /// let p = Probe::list(cl, opts)?;
    /// ```
    ///
    #[cfg(feature = "alt-api")]
    pub fn list(cl: &Client, opts: &HashMap<&str, &str>) -> Result<List<Self>, APIError> {
        Ok(cl.get_probes(opts)?)
    }
}

// -------------------------------------------------------------------------

/// Main API methods for `Probe` type
///
/// This is the code enabled by the `flat-api` feature.
///
/// XXX May just disappear as I do not see this as real idiomatic Rust code.
///
impl Client {
    /// Get information on a specific probe by ID
    ///
    /// Examples:
    ///
    /// ```no_run
    ///  # use atlas_rs::client::ClientBuilder;
    ///  # use atlas_rs::core::probes::Probe;
    ///
    ///     let cl = ClientBuilder::new().api_key("foo").verbose(true);
    ///     let pi = cl.get_probe(666)?;
    ///
    ///     println!("Probe ID {}: {}", 666, pi.description);
    ///  ```
    ///
    #[cfg(feature = "flat-api")]
    pub fn get_probe(&self, id: u32) -> Result<Probe, APIError> {
        let opts = &self.opts.clone();
        let url = format!("{}/probes/{}/", self.endpoint, id);
        let url = add_opts(&url, opts);

        let resp = self.agent.as_ref().unwrap().get(&url).send();

        let resp = match resp {
            Ok(resp) => resp,
            Err(e) => {
                let aerr = APIError::new(
                    e.status().unwrap().as_u16(),
                    "Bad",
                    e.to_string().as_str(),
                    "get_probe",
                );
                return Err(aerr);
            }
        };

        // Try to see if we got an error
        match resp.status() {
            StatusCode::OK => {
                // We could use Response::json() here but it consumes the body.
                let r = resp.text()?;
                println!("p={}", r);
                let p: Probe = serde_json::from_str(&r)?;
                Ok(p)
            }
            _ => {
                let aerr = resp.json::<APIError>()?;
                Err(aerr)
            }
        }
    }

    /// Get information about a set of probes according to parameters
    ///
    #[cfg(feature = "flat-api")]
    pub fn get_probes(&self, opts: &HashMap<&str, &str>) -> Result<List<Probe>, APIError> {
        let gopts = &self.opts.clone();
        let url = format!("{}/probes/", &self.endpoint);

        // Add global options
        let url = add_opts(&url, gopts);
        // Add our specific ones, like page=NN
        let url = add_opts(&url, opts);

        let res: List<Probe> = self.fetch_one_page(&url, 1)?;

        if res.count == 0 {
            return Err(APIError::new(500, "Empty list", "nothing", "get_probes"));
        }

        if res.next.is_empty() {
            // We have no pagination
        }
        Ok(res)
    }
}

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
            Op::Get => format!("/probes/{}/", p),    // /get
            Op::Set => format!("/probes/{}/", p),    // /set
            Op::Update => format!("/probes/{}/", p), // /update
            Op::Measurement => format!("/probes/{}/measurements/", p), // P/measurements
            Op::Archive => "/probes/archive/".to_string(), // /archive
            Op::Rankings => "/probes/rankings/".to_string(), // rankings
            Op::Tags => "/probes/tags/".to_string(), // /tags/
            Op::Slugs => format!("/probes/tags/{}/slugs", p), // /tags/T/slugs/
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
