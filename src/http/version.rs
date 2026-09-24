#[derive(Debug, PartialEq, Clone)]
pub enum HttpVersion {
    Http10,
    Http11,
}

impl HttpVersion {
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        match bytes {
            b"HTTP/1.0" => Some(HttpVersion::Http10),
            b"HTTP/1.1" => Some(HttpVersion::Http11),
            _ => None,
        }
    }
}