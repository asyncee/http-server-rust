use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct UnsupportedHttpVersion;

impl Display for UnsupportedHttpVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unsupported HTTP version")
    }
}

impl Error for UnsupportedHttpVersion {}

#[derive(Debug, Clone)]
pub enum HttpVersion {
    Http11
}

impl Display for HttpVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpVersion::Http11 => write!(f, "HTTP/1.1")
        }
    }
}

impl FromStr for HttpVersion {
    type Err = UnsupportedHttpVersion;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HTTP/1.1" => Ok(HttpVersion::Http11),
            _ => Err(UnsupportedHttpVersion)
        }
    }
}
