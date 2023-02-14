use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FormatterResult};
use std::str::Utf8Error;

pub struct Request {
    method: Method,
    path: String,
    query_string: Option<String>,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParserError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let buffer: &str = std::str::from_utf8(value)?;
        let mut iterator = buffer.split_whitespace();
        let method = iterator.next().ok_or(ParserError::InvalidRequest)?;
        let url = iterator.next().ok_or(ParserError::InvalidRequest)?;
        let protocol = iterator.next().ok_or(ParserError::InvalidRequest)?;
        println!("method = {}\nurl = {}\nprotocal = {}", method, url, protocol);

        unimplemented!()
    }
}

pub enum ParserError {
    InvalidProtocol,
    InvalidRequest,
    InvalidEncoding,
}

impl ParserError {
    fn message(&self) -> &str {
        match self {
            ParserError::InvalidProtocol => "Invalid protocol",
            ParserError::InvalidRequest => "Invalid request",
            ParserError::InvalidEncoding => "Invalid encoding",
        }
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatterResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatterResult {
        write!(f, "{}", self.message())
    }
}

impl From<Utf8Error> for ParserError {
    fn from(_: Utf8Error) -> Self {
        self::ParserError::InvalidEncoding
    }
}

impl Error for ParserError {}