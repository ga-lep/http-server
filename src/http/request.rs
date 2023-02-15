use super::method::Method;
use super::parser_error::ParserError;
use std::convert::TryFrom;

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
        let mut url = iterator.next().ok_or(ParserError::InvalidRequest)?;
        let protocol = iterator.next().ok_or(ParserError::InvalidRequest)?;
        println!("method = {}\nurl = {}\nprotocol = {}", method, url, protocol);

        let method : Method = method.parse()?;
        let mut query_string = None;
        if let Some(i) = url.find('?') {
            query_string = Some(&url[i + 1..]);
            url = &url[..i];
        }

        unimplemented!()
    }
}

