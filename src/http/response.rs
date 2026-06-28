use crate::http::status::StatusCode;
use crate::http::version::Version;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Response {
    pub version: Version,
    pub status: StatusCode,
    pub headers: HashMap<String, String>,
    body: Vec<u8>,
}
