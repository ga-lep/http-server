use server::Server;
use crate::website_handler::WebsiteHandler;

mod server;
mod http;
mod website_handler;

fn main() {
    let ip = String::from("127.0.0.1");
    let server = Server::new(ip, 8080);
    server.run(WebsiteHandler)
}

/*
GET / HTTP/1.1
Host: developer.mozilla.org
Accept-Language: fr
 */