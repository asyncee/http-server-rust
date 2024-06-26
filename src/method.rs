use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::error::Error;

#[derive(Debug)]
pub struct UnsupportedMethod {}

impl Display for UnsupportedMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} method is not supported", self)
    }
}

impl Error for UnsupportedMethod {}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Method {
    GET,
    POST,
}

impl FromStr for Method {
    type Err = UnsupportedMethod;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            _ => Err(UnsupportedMethod {})
        }
    }
}
