use std::collections::HashMap;

/// We target the v2 API
const ENDPOINT: &str = "https://atlas.ripe.net/api/v2";

#[derive(Debug)]
pub enum AF {
    V4,
    V6,
    V46,
}

/// Hold the client data
#[derive(Debug)]
pub struct Client {
    /// Mandatory
    pub(crate) api_key: &'static str,

    /// Optional
    pub(crate) endpoint: &'static str,
    pub(crate) default_probe: u32,
    pub(crate) area_type: &'static str,
    pub(crate) area_value: &'static str,
    pub(crate) is_oneoff: bool,
    pub(crate) pool_size: usize,
    pub(crate) want_af: AF,
    pub(crate) verbose: bool,
    pub(crate) tags: &'static str,

    /// Default options
    pub(crate) opts: HashMap<&'static str, &'static str>,
}

impl Client {
    /// Set default values
    pub fn new() -> Self {
        Client {
            api_key: "<CHANGEME>",
            endpoint: ENDPOINT,
            default_probe: 0,
            area_type: "area",
            area_value: "WW",
            is_oneoff: true,
            pool_size: 10,
            want_af: AF::V46,
            verbose: false,
            tags: "",
            opts: HashMap::new(),
        }
    }

    /// Sets the value of the required API key
    ///
    /// Examples
    ///
    /// ```no_run
    /// # use atlas_rs::client::Client;
    /// Client::new()
    ///     .api_key("FOO")
    /// # ;
    /// ```
    pub fn api_key<S: Into<&'static str>>(mut self, k: S) ->  Self {
        self.api_key = k.into();
        self
    }
/*
    pub fn endpoint(&mut self, v: &str) -> Self {
        self.endpoint = v;
        self
    }

    pub fn default_probe(&mut self, v: u32) -> Self {
        self.default_probe = v;
        self
    }

    pub fn area_type(&mut self, v: &str) -> Self {
        self.area_type = v;
        self
    }

    pub fn area_value(&mut self, v: &str) -> Self {
        self.area_value = v;
        self
    }
*/
    pub fn onoff<S: Into<bool>>(mut self, v: S) -> Self {
        self.is_oneoff = v.into();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let c = Client::new();

        assert_eq!("<CHANGEME>", c.api_key);
    }

    #[test]
    fn test_api_key() {
        let c= Client::new()
            .api_key("FOO")
            .onoff(true);
        assert_eq!("FOO", c.api_key);
        assert_eq!(ENDPOINT, c.endpoint);
        assert!(c.is_oneoff);
        println!("{:#?}", c);
    }
}