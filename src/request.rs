use anyhow::Result;
use crate::client::{Client, Cmd};
use crate::probes;
use crate::probes::Probe;

use serde::de;
use crate::errors::APIError;

#[derive(Clone, Copy, Debug)]
pub enum Param<'a> {
    I(u32),
    S(&'a str),
}

impl<'a> From<&'a str> for Param<'a> {
    fn from(p: &'a str) -> Self {
        Param::S(p)
    }
}

impl<'a> From<Param<'a>> for &'a str {
    fn from(p: Param<'a>) -> Self {
        p.into()
    }
}

impl<'a> From<u32> for Param<'a> {
    fn from(p: u32) -> Self {
        Param::I(p)
    }
}

impl<'a> From<Param<'a>> for u32 {
    fn from(p: Param<'a>) -> Self {
        p.into()
    }
}

#[derive(Debug)]
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


