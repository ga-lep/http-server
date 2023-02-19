use super::{Method, ParserError, QueryString};
use std::convert::TryFrom;

#[derive(Debug)]
pub struct Request<'raw_buffer> {
    pub method: Method,
    pub url: &'raw_buffer str,
    pub query_string: Option<QueryString<'raw_buffer>>,
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
            query_string = Some(QueryString::from(&url[i + 1..]));
            url = &url[..i];
        }

        Ok(Self {
            method,
            url,
            query_string
        })
    }
}


