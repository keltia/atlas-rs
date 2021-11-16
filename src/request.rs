use reqwest::Request;

use crate::client::Client;

pub struct RequestBuilder<'rq> {
    pub c: Client<'rq>,
    pub r: Request,
}

