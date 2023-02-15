use std::error::Error;
use std::fmt::{Debug, Display, Result as FormatterResult, Formatter};
use std::str::Utf8Error;

#[derive(PartialEq)]
pub enum ParserError {
    InvalidProtocol,
    InvalidMethod,
    InvalidRequest,
    InvalidEncoding,
}

impl ParserError {
    fn message(&self) -> &str {
        match self {
            ParserError::InvalidProtocol => "Invalid protocol",
            ParserError::InvalidMethod => "Invalid method",
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