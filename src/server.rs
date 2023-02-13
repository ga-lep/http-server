use std::net::TcpListener;

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
                Ok((tcpStreammm, socketAddr)) => println!("{}", socketAddr.ip()),
                Err(error) => println!("Failed to established connection: {}", error)
            }
        }
    }
}