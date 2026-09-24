use std::fmt;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ParseError {
    InvalidRequest,

    InvalidMethod,

    InvalidVersion,

    InvalidHeader,

    InvalidRequestLine,

    MissingHostHeader,

    MissingMethodName,

    MissingHTTPVersion,
    
    MissingRoutePath,

    UnexpectedEOF,
}



impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidRequest => write!(f, "invalid HTTP request"),
            ParseError::InvalidMethod => write!(f, "invalid HTTP method"),

            ParseError::InvalidVersion => write!(f, "invalid HTTP version"),

            ParseError::InvalidHeader => write!(f, "invalid header"),

            ParseError::InvalidRequestLine => write!(f, "invalid request line"),

            ParseError::MissingHostHeader => write!(f, "missing Host header"),

            ParseError::UnexpectedEOF => write!(f, "unexpected end of request"),

            ParseError::MissingRoutePath => write!(f,"missing path"),

            ParseError::MissingMethodName => write!(f,"missing method name"),

            ParseError::MissingHTTPVersion => write!(f,"missing HTTP version"),
        }
    }
}
