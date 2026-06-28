use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Version {
    Http10,
    Http11,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Method {
    Get,
    Post,
    Patch,
    Put,
    Delete,
    Head,
    Options,
    Connect,
    Trace,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub version: Version,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}
