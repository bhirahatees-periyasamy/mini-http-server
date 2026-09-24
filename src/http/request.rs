use crate::error::http::ParseError;
use crate::http::method::Method;
use crate::http::version::HttpVersion;
use core::result::Result::{self, Err, Ok};
use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::net::TcpStream;

#[derive(Debug, PartialEq, Clone)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub version: HttpVersion,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

pub struct RequestLine<'a> {
    pub method: &'a [u8],
    pub path: &'a [u8],
    pub version: &'a [u8],
}


impl Request {
    pub fn new(
        method: Method,
        path: String,
        version: HttpVersion,
        headers: HashMap<String, String>,
        body: Vec<u8>,
    ) -> Self {
        Request { method: method, path: path, version: version, headers: headers, body: body }
    }

    pub fn handle_client(mut stream: TcpStream) -> io::Result<()> {
        let mut data: Vec<u8> = Vec::new();
        let mut buffer = [0; 1024];

        loop {
            let n = match stream.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => n,
                Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e) => return Err(e),
            };

            data.extend_from_slice(&buffer[..n]);
            // End of headers reached
            if data.windows(4).any(|w| w == b"\r\n\r\n") {
                break;
            }
            // Guard against huge/malicious requests
            if data.len() > 8 * 1024 {
                break;
            }
        }

        let body = "Hello from mini-http-server";
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain\r\nConnection: close\r\n\r\n{}",
            body.len(),
            body
        );

        let request = Self::parse(data);

        println!("Request: {:?}", request);

        stream.write_all(response.as_bytes())?;
        stream.flush()?;
        Ok(())
    }

    fn parse_request_line<'a>(request_line: &'a[u8]) -> Result<RequestLine<'a>, ParseError> {
        let request_line = request_line.strip_suffix(b"\r").unwrap_or(request_line);

        let mut parts = request_line.split(|&byte| byte == b' ');

        let method = parts
            .next()
            .ok_or_else(|| ParseError::MissingMethodName)?;

        let path = parts
            .next()
            .ok_or_else(|| ParseError::MissingRoutePath)?;

        let version = parts
            .next()
            .ok_or_else(|| ParseError::MissingHTTPVersion)?;

        Ok(RequestLine { method, path, version })
    }

    fn parse_headers<'a, I>(lines: I) -> Result<HashMap<String, String>, ParseError> 
        where  I: Iterator<Item = &'a [u8]>,
     {
        let mut headers = HashMap::new();

        for line in lines {
            let line = line.strip_suffix(b"\r").unwrap_or(line);

            if line.is_empty() {
                continue;
            }

            let colon = line.iter().position(|&byte| byte == b':').ok_or_else(|| {
                ParseError::MissingHostHeader
            })?;

            let name = &line[..colon];
            let value = &line[colon + 1..];

            let value = value.strip_prefix(&[b' ']).unwrap_or(value);

            let name = String::from_utf8_lossy(name).to_string();
            let value = String::from_utf8_lossy(value).to_string();

            headers.insert(name, value);
        }

        Ok(headers)
    }

    pub fn parse(data: Vec<u8>) -> Result<Request, ParseError> {
        let header_end = data
            .windows(4)
            .position(|window| window == b"\r\n\r\n")
            .ok_or_else(|| ParseError::InvalidRequest)?;

        let header_section = &data[..header_end];
        let body_start = header_end + 4;
        let mut lines = header_section.split(|&byte| byte == b'\n');

        
        let request_line = lines
            .next()
            .ok_or_else(|| ParseError::InvalidRequestLine)?;

        let parsed_request_line = Request::parse_request_line(request_line)?;

        let headers = Request::parse_headers(lines)?;

        let body = data[body_start..].to_vec();

        let method = Method::from_bytes(parsed_request_line.method)
            .ok_or_else(|| ParseError::InvalidMethod)?;

        let version = HttpVersion::from_bytes(parsed_request_line.version)
            .ok_or_else(|| ParseError::InvalidVersion)?;

        Ok(Request {
            method: method,
            path: String::from_utf8_lossy(parsed_request_line.path).to_string(),
            version: version,
            headers,
            body,
        })
    }
}
