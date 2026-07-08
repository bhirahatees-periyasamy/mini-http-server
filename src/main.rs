pub mod error;
pub mod http;

use std::{io::Read, net::TcpListener};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:1234")?;
    for stream in listener.incoming() {
        let mut stream = stream?;

        println!("Client connected!");

        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer)?;

        println!("Read {} bytes", bytes_read);

        println!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
    }
    Ok(())
}
