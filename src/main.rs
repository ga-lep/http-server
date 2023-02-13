use server::Server;

mod server;
mod http;

fn main() {
    let ip = String::from("0.0.0.0");
    let server = Server::new(ip, 8080);
    server.run()
}

/*
GET / HTTP/1.1
Host: developer.mozilla.org
Accept-Language: fr
 */