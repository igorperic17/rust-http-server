use crate::http::ParseError;
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    CONNECT,
    OPTION,
    TRACE,
    PATCH,
}

impl FromStr for Method {
    type Err = ParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let m = match value {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "DELETE" => Method::DELETE,
            "HEAD" => Method::HEAD,
            "CONNECT" => Method::CONNECT,
            "OPTION" => Method::OPTION,
            "TRACE" => Method::TRACE,
            "PATCH" => Method::PATCH,
            _ => {
                return Err(ParseError::InvalidMethod);
            }
        };
        Ok(m)
    }
}
