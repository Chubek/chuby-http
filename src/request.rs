use crate::{common::*, parser::RequestParser};

pub struct Request {
    pub headers: Vec<Header>,
    pub body: String,
    pub uri: String,
    pub uri_params: Vec<Params>,
    pub uri_paths: Vec<String>,
    pub bare_uri: String,
    pub method: Method,
    pub port: u32,
    pub userinfo: UserInfo,
    pub host: String,
    pub scheme: String,
    pub referer: String,
    pub content_type: MimeType,
}


impl Request {
    pub fn from(s: String) -> Self {
        let req = RequestParser::parse_and_create(s);

        req
    }
}