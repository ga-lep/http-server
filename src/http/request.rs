use super::method::Method;
use super::parser_error::ParserError;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter, Result as FormatterResult};

pub struct Request<'raw_buffer> {
    method: Method,
    url: &'raw_buffer str,
    query_string: Option<&'raw_buffer str>,
}

impl<'raw_buffer> TryFrom<&'raw_buffer [u8]> for Request<'raw_buffer> {
    type Error = ParserError;

    fn try_from(raw_buffer: &'raw_buffer [u8]) -> Result<Self, Self::Error> {
        let buffer: &str = std::str::from_utf8(raw_buffer)?;
        let mut iterator = buffer.split_whitespace();
        let method = iterator.next().ok_or(ParserError::InvalidRequest)?;
        let mut url = iterator.next().ok_or(ParserError::InvalidRequest)?;
        //let protocol = iterator.next().ok_or(ParserError::InvalidRequest)?;

        let method : Method = method.parse()?;
        let mut query_string = None;
        if let Some(i) = url.find('?') {
            query_string = Some(&url[i + 1..]);
            url = &url[..i];
        }

        Ok(Self {
            method,
            url,
            query_string
        })
    }
}
impl<'raw_buffer> Display for Request<'raw_buffer> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatterResult {
        write!(f, "Request : {{ {}, {}, {} }}", self.method, self.url, self.query_string.unwrap_or(""))
    }
}


