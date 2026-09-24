use crate::http::method::Method;
use crate::http::version::HttpVersion;
use core::result::Result::Err;
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

impl Request {

    pub fn new(method: Method, path: String, version:HttpVersion, headers: HashMap<String, String>, body: Vec<u8>) -> Self {
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

        println!("--- raw request ---\n{}", String::from_utf8_lossy(&data));

        let body = "Hello from mini-http-server";
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain\r\nConnection: close\r\n\r\n{}",
            body.len(),
            body
        );
        stream.write_all(response.as_bytes())?;
        stream.flush()?;
        Ok(())
    }

    // pub fn parse(self, data: Vec<u8>) -> Request {
        
    // }
}
