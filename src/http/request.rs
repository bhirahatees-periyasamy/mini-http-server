use crate::http::method::Method;
use crate::http::version::Version;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub version: Version,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}
