use crate::http::method::Method;
use crate::http::version::HttpVersion;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub version: HttpVersion,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

// impl Request {
//     fn parse(input: &[u8]) -> Result<Self, ParseError>
// }
