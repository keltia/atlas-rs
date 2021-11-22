use anyhow::Result;
use crate::client::{Client, Cmd};
use crate::probes;
use crate::probes::Probe;

use serde::de;
use crate::errors::APIError;

// ------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub enum Param<'a> {
    I(u32),
    S(&'a str),
}

// Implement From: for our enum

/// From &str to Param
impl<'a> From<&'a str> for Param<'a> {
    fn from(s: &'a str) -> Self {
        Param::S(s)
    }
}

/// From Param to &str
impl<'a> From<Param<'a>> for &'a str {
    fn from(p: Param<'a>) -> Self {
        return match p {
            Param::S(s) => s,
            _ => "",
        }
    }
}

/// From u32 to Param
impl<'a> From<u32> for Param<'a> {
    fn from(p: u32) -> Self {
        Param::I(p)
    }
}

/// From Param to u32
impl<'a> From<Param<'a>> for u32 {
    fn from(p: Param<'a>) -> Self {
        return match p {
            Param::I(v) => v,
            _ => 0,
        }
    }
}

// ------------------------------------------------------------
// RequestBuilder itself

#[derive(Debug)]
pub struct RequestBuilder<'rq> {
    /// Context is which part of the API we are targetting (`/probe/`, etc.)
    pub ctx: Cmd,
    pub c: Client<'rq>,
    pub r: reqwest::blocking::Request,
}

/// Add methods for chaining and keeping state
impl<'rq> RequestBuilder<'rq> {
    pub fn new(ctx: Cmd, c: Client<'rq>, r: reqwest::blocking::Request) -> Self {
        RequestBuilder {ctx, c, r}
    }

    pub fn get(self, data: Param<'rq>) -> Self
    {
        // Main routing
        match self.ctx {
            Cmd::Probes => {
                return Probe::dispatch(self, probes::Ops::Get, data);
            },
            Cmd::Measurements => unimplemented!(),
            Cmd::AnchorMeasurements => unimplemented!(),
            Cmd::Credits => unimplemented!(),
            Cmd::Anchors => unimplemented!(),
            Cmd::Keys => unimplemented!(),
            Cmd::ParticipationRequests => unimplemented!(),
        }
    }

    /// Finalize the chain and call the real API
    ///
    pub fn call<T>(self) -> Result<T, APIError>
        where T: de::Deserialize<'rq>
    {
        let r = match self.r {
            Ok(r) => r,
            Err(e) => return Err(APIError::new(500,
                                               "bad request",
                                               e.to_string().as_ref(),
                                               "")),
        };

        let resp = self.c.agent.as_ref().unwrap()
            .get(r.url().as_str()).send()?;

        let txt = match resp.text() {
            Ok(t) => t,
            Err(e) => return Err(APIError::new(500,
                                               "no body",
                                               e.to_string().as_ref(),
                                               ""))
        }.as_str();

        let r: T = serde_json::from_str(txt)?;
        Ok(r)
    }
}


