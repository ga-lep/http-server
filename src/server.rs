use std::net::TcpListener;
use std::io::Read;
use crate::http::{ParserError, Request, StatusCode};
use crate::http::response::Response;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParserError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

const MESSAGE_SIZE: usize = 65535;

pub struct Server {
    ip: String,
    port: u16,
}

impl Server {
    pub fn new(ip: String, port: u16) -> Server {
        Server {
            ip,
            port,
        }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}:{}", self.ip, self.port);

        let listener = TcpListener::bind(format!("{}:{}", self.ip, self.port)).unwrap();

        loop {
             match listener.accept() {
                Ok((mut tcp_stream, socket_addr)) => {
                    let mut data = [0u8; MESSAGE_SIZE];

                    tcp_stream.read(&mut data);

                    let response = match Request::try_from(&data[..]) {
                        Ok(request) => handler.handle_request(&request),
                        Err(e) => handler.handle_bad_request(&e),
                    };
                    if let Err(e) = response.send(&mut tcp_stream) {
                        println!("Failed to send response : {}", e);
                    }
                }
                Err(error) => println!("Failed to established connection: {}", error)
            }
        }
    }
}