use crate::http::status::StatusCode;
use crate::http::version::HttpVersion;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Response {
    pub version: HttpVersion,
    pub status: StatusCode,
    pub headers: HashMap<String, String>,
    body: Vec<u8>,
}
