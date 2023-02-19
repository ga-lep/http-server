use std::fmt::{Debug, Display, Formatter, Result as FormatterResult};

#[derive(Copy, Clone, Debug)]
pub enum StatusCode {
    Ok = 200,
    Created = 201,
    NoContent = 204,
    BadRequest = 400,
    NotFound = 404
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> FormatterResult {
        write!(f, "{} {}", *self as u16, match self {
            StatusCode::Ok => "OK",
            StatusCode::Created => "CREATED",
            StatusCode::NoContent => "NO CONTENT",
            StatusCode::BadRequest => "BAD REQUEST",
            StatusCode::NotFound => "NOT FOUND",
        })
    }
}