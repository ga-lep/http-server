use std::str::FromStr;
use crate::http::parser_error::ParserError;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Method {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

impl FromStr for Method {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Method::GET),
            "HEAD" => Ok(Method::HEAD),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            "CONNECT" => Ok(Method::CONNECT),
            "OPTIONS" => Ok(Method::OPTIONS),
            "TRACE" => Ok(Method::TRACE),
            "PATCH" => Ok(Method::PATCH),
            _ => Err(ParserError::InvalidMethod)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::http::{Method, ParserError};

    #[test]
    fn test_convert_get() {
        assert_eq!("GET".parse(), Ok(Method::GET));
    }

    #[test]
    fn test_convert_head() {
        assert_eq!("HEAD".parse(), Ok(Method::HEAD));
    }


    #[test]
    fn test_convert_post() {
        assert_eq!("POST".parse(), Ok(Method::POST));
    }


    #[test]
    fn test_convert_put() {
        assert_eq!("PUT".parse(), Ok(Method::PUT));
    }


    #[test]
    fn test_convert_delete() {
        assert_eq!("DELETE".parse(), Ok(Method::DELETE));
    }


    #[test]
    fn test_convert_connect() {
        assert_eq!("CONNECT".parse(), Ok(Method::CONNECT));
    }


    #[test]
    fn test_convert_options() {
        assert_eq!("OPTIONS".parse(), Ok(Method::OPTIONS));
    }


    #[test]
    fn test_convert_trace() {
        assert_eq!("TRACE".parse(), Ok(Method::TRACE));
    }


    #[test]
    fn test_convert_patch() {
        assert_eq!("PATCH".parse(), Ok(Method::PATCH));
    }

    #[test]
    fn test_convert_error() {
        assert_eq!("POTATO".parse::<Method>(), Err(ParserError::InvalidMethod));
    }
}