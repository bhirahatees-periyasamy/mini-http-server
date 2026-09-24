#[derive(Debug, PartialEq, Clone)]
pub enum Method {
    GET,
    POST,
    PATCH,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    CONNECT,
    TRACT,
}

impl Method {
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        match bytes {
            b"GET" => Some(Method::GET),
            b"POST" => Some(Method::POST),
            b"PUT" => Some(Method::PUT),
            b"DELETE" => Some(Method::DELETE),
            b"PATCH" => Some(Method::PATCH),
            b"HEAD" => Some(Method::HEAD),
            b"OPTIONS" => Some(Method::OPTIONS),
            b"CONNECT" => Some(Method::CONNECT),
            b"TRACT" => Some(Method::TRACT),
            _ => None,
        }
    }
}
