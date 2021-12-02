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
use std::fmt::Formatter;

// External crates
use serde::{Deserialize, Serialize};

#[cfg(feature = "flat-api")]
use reqwest::StatusCode;

// Our crates
use crate::client::Client;
use crate::request::{Param, RequestBuilder};

// -------------------------------------------------------------------------

/// All operations available
#[derive(Debug)]
pub enum Ops {
    Permissions = 1,
    Targets,
    Get,
    Set,
    Delete,
    List,
    Create,
}

/// Generate the proper URL for the service we want in the given category
fn set_url(ops: Ops, uuid: String) -> String {
    match ops {
        Ops::Permissions => "/keys/permissions/".to_string(), // /permissions
        Ops::Targets => format!("/keys/permissions/{}/targets/", uuid), // /get targets
        Ops::Get => format!("/keys/{}/", uuid),               // /get
        Ops::Set => format!("/keys/{}/", uuid),               // /set
        Ops::Delete => format!("/keys/{}/", uuid),            // /delete
        Ops::List => "/keys/".to_string(),                    // /list
        Ops::Create => "/keys/".to_string(),                  // /create
    }
}

// -------------------------------------------------------------------------

/// This is the structure describing an API key with its validity, entitlements, etc.
///
#[derive(Serialize, Deserialize, Debug)]
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

impl Key {
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

/// Implement the Display trait.
///
impl fmt::Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

// -------------------------------------------------------------------------

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
    pub target: Option<Target>,
}

// -------------------------------------------------------------------------

/// When asking for a list of keys, this struct is used for pagination
#[derive(Serialize, Deserialize, Debug)]
pub struct KeyList {
    /// How many results in this block
    pub count: u32,
    /// URL to fetch the next block
    pub next: String,
    /// URL to fetch previous block
    pub previous: String,
    /// Current key block
    pub keys: Vec<Key>,
}

// -------------------------------------------------------------------------

/// Main API methods for `key` type
impl<'cl> Client<'cl> {
    /// Get information on a specific key by ID
    ///
    /// Examples:
    ///
    /// ```no_run
    ///  # use atlas_rs::client::Client;
    ///  # use atlas_rs::keys::Key;
    ///
    ///     let cl = Client::new().verbose(true);
    ///     let pi = cl.get_key("key-id").unwrap();
    ///
    ///     println!("key ID {}: {}", pi.uuid, pi.label);
    ///  ```
    ///
    #[cfg(feature = "flat-api")]
    pub fn get_key(&self, uuid: &str) -> Result<Key, APIError> {
        let opts = &self.opts.clone();
        let url = format!("{}/keys/{}/", self.endpoint, uuid);
        let url = add_opts(&url, opts);

        let resp = self.agent.as_ref().unwrap().get(&url).send();

        let resp = match resp {
            Ok(resp) => resp,
            Err(e) => {
                let aerr = APIError::new(
                    e.status().unwrap().as_u16(),
                    "Bad",
                    e.to_string().as_str(),
                    "get_key",
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
                let p: Key = serde_json::from_str(&r)?;
                Ok(p)
            }
            _ => {
                let aerr = resp.json::<APIError>()?;
                Err(aerr)
            }
        }
    }

    /// Get information about a set of keys according to parameters
    ///
    #[cfg(feature = "flat-api")]
    pub fn get_keys() -> Result<Vec<Key>, APIError> {
        unimplemented!()
    }
}

// -------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_key() {}
}
