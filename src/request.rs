use anyhow::{bail, Result};
use crate::client::Client;

pub struct RequestBuilder<'rq> {
    /// Context is which part of the API we are targetting (`/probe/`, etc.)
    pub ctx: &'rq str,
    pub c: &'rq Client<'rq>,
    pub r: Result<reqwest::blocking::Request>,
}

impl<'rq> RequestBuilder<'rq> {
    pub fn new(ctx: &'rq str, c: &'rq Client<'rq>, r: Result<reqwest::blocking::Request>) -> Self {
        RequestBuilder {ctx, c, r}
    }

    pub fn call(self) -> Result<reqwest::blocking::Response> {
        let r = match self.r {
            Ok(r) => r,
            Err(e) => bail!("e"),
        };
        let resp = self.c.agent.as_ref().unwrap().clone()
            .get(r.url().as_str()).send()?;
        Ok(resp)
    }
}


