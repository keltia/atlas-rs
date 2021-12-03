//! Structs and methods to deal with anchor measurements
//!

// We have the following call tree:
//
// Atlas API ----- /anchor-measurement     ----- /list  ----- List<AM>
//                                         ----- /get  ----- AM

// Our own crates
use crate::request::{Param, RequestBuilder};

// External crates
use serde::{Serialize, Deserialize};

/// All operations available
///
#[derive(Debug)]
pub enum Ops {
    Get,
    List,
}

/// Generate the proper URL for the service we want in the given category
///
fn set_url(ops: Ops, uuid: String) -> String {
    match ops {
        Ops::Get => format!("/anchor-measurements/{}/", uuid), // /get
        Ops::List => "/anchor-measurements/".to_string(),      // /list
    }
}

/// Struct describing all data about a given anchor targetted by a measurement
///
#[derive(Serialize, Deserialize, Debug)]
pub struct AnchorMeasurement {
    /// Creation date
    date_created: String,
    /// Last modification date
    date_modified: String,
    /// ID of the target Measurement
    id: u32,
    is_mesh: bool,
    /// URL of the actual measurement
    measurement: String,
    /// URL of the anchor to which the measurement is targeted
    target: String,
    /// Measurement type of the involved measurement
    #[serde(rename = "type")]
    mtype: String,
}

impl AnchorMeasurement {
    /// Set the parameters and return the RequestBuilder object
    ///
    pub fn dispatch<'a>(
        r: &'a mut RequestBuilder<'a>,
        ops: Ops,
        data: Param<'a>,
    ) -> &'a mut RequestBuilder<'a> {
        let opts = r.c.opts.clone();
        let add = set_url(ops, data.into());

        let url = reqwest::Url::parse_with_params(
            format!("{}{}", r.r.url().as_str(), add).as_str(),
            opts.iter(),
        )
            .unwrap();
        r.r = reqwest::blocking::Request::new(r.r.method().clone(), url);
        r
    }
}
