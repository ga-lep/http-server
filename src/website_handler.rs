use crate::http::{Method, Request, StatusCode};
use crate::http::response::Response;
use crate::server::Handler;

pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        Response::new(StatusCode::Ok, Some("patate".to_string()))

        match request.method {
            Method::GET => match request.path {
                _ => Response::new(StatusCode::NotFound, None)
            }
            _ => Response::new(StatusCode::NotFound, None)
        }
    }
}