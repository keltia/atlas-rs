//! Struct and methods to deal with probes
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
use std::fmt;
use std::fmt::Formatter;

// External crates
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[cfg(feature = "flat-api")]
use reqwest::StatusCode;

// Our crates
use crate::client::Client;
use crate::common::{add_opts, List};
use crate::errors::*;
use crate::request::{Param, RequestBuilder};

// -------------------------------------------------------------------------

/// All operations available
#[derive(Debug)]
pub enum Ops {
    List = 1,
    Get,
    Set,
    Update,
    Measurement,
    Archive,
    Rankings,
    Tags,
    Slugs,
}

/// Generate the proper URL for the service we want in the given category
fn set_url(ops: Ops, p: u32) -> String {
    match ops {
        Ops::List => "/probes/".to_string(),      // /list
        Ops::Get => format!("/probes/{}/", p),    // /get
        Ops::Set => format!("/probes/{}/", p),    // /set
        Ops::Update => format!("/probes/{}/", p), // /update
        Ops::Measurement => format!("/probes/{}/measurements/", p), // P/measurements
        Ops::Archive => "/probes/archive/".to_string(), // /archive
        Ops::Rankings => "/probes/rankings/".to_string(), // rankings
        Ops::Tags => "/probes/tags/".to_string(), // /tags/
        Ops::Slugs => format!("/probes/tags/{}/slugs", p), // /tags/T/slugs/
    }
}

// -------------------------------------------------------------------------

/// Geolocation as reported by the probe
#[derive(Serialize, Deserialize, Debug)]
pub struct Geometry {
    #[serde(rename = "type")]
    pub gtype: String,
    /// GPS coordinates
    pub coordinates: Vec<f64>,
}

/// Describes the current status of the probe
#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    /// Date
    pub since: String,
    /// Status ID
    pub id: u32,
    /// Status: connected, etc.
    pub name: String,
}

/// Tags about the probe, most generated by the API, some you can add
#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    pub name: String,
    pub slug: String,
}

/// All information about a given probe
#[derive(Serialize, Deserialize, Debug)]
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
    pub country_code: String,
    /// Free text description
    pub description: String,
    /// First connection
    pub first_connected: u32,
    /// Approx Position
    pub geometry: Geometry,
    /// Probe ID
    pub id: u32,
    /// Is it an Anchor?
    pub is_anchor: bool,
    /// Is it public?
    pub is_public: bool,
    /// POSIX time since last connect
    pub last_connected: u32,
    /// IPv4 Network Prefix
    pub prefix_v4: Option<String>,
    /// IPv6 Network Prefix
    pub prefix_v6: Option<String>,
    /// Probe Status
    pub status: Status,
    /// Integer time
    pub status_since: u32,
    /// System and User tags
    pub tags: Vec<Tag>,
    /// Total uptime
    pub total_uptime: u32,
    /// Probe Type
    #[serde(rename = "type")]
    pub ptype: String,
}

/// Implement fmt::Display for Probe
impl fmt::Display for Probe {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

// -------------------------------------------------------------------------

/// When asking for a list of probes, this struct is used for pagination
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
    /// Main routing that build the URL for the request
    ///
    pub fn dispatch<'a>(
        mut r: RequestBuilder<'a>,
        ops: Ops,
        data: Param<'a>,
    ) -> RequestBuilder<'a> {
        // Get the parameter
        let add = set_url(ops, data.into());

        // Setup URL with potential parameters like `key`.
        let url = reqwest::Url::parse_with_params(
            format!("{}/{}", r.r.url().as_str(), add).as_str(),
            &r.c.opts,
        )
        .unwrap();

        r.r = reqwest::blocking::Request::new(r.r.method().clone(), url);
        r
    }

    /// Alternate API for probes
    ///
    /// Example:
    /// ```no_run
    /// # use atlas_rs::probes::Probe;
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
    /// # use atlas_rs::probes::Probe;
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
impl<'cl> Client<'cl> {
    /// Get information on a specific probe by ID
    ///
    /// Examples:
    ///
    /// ```no_run
    ///  # use atlas_rs::client::ClientBuilder;
    ///  # use atlas_rs::probes::Probe;
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

// -------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_url() {
        assert_eq!("/probes/666/measurements/", set_url(Ops::Measurement, 666));
    }

    #[test]
    fn test_get_probe() {}
}
