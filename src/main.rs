pub mod error;
pub mod http;
use std:: net::TcpListener;
use crate::http::request::{Request};


fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:1234")?;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(e) = Request::handle_client(stream){
                    eprintln!("connection error:: {e}");
                }
            },
            Err(e) => eprintln!("accept failed: {e}"),
        }
    }
    Ok(())
}
