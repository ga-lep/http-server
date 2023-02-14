use std::net::TcpListener;
use std::io::Read;
use crate::http::Request;

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

    pub fn run(self) {
        println!("Listening on {}:{}", self.ip, self.port);

        let listener = TcpListener::bind(format!("{}:{}", self.ip, self.port)).unwrap();

        loop {
             match listener.accept() {
                Ok((mut tcpStream, socketAddr)) => {
                    let mut data = [0u8; MESSAGE_SIZE];

                    tcpStream.read(&mut data);

                    let request = Request::try_from(&data[..]);
                    println!("{} {}",
                             socketAddr.ip(),
                             String::from_utf8_lossy(&data))
                },
                Err(error) => println!("Failed to established connection: {}", error)
            }
        }
    }
}