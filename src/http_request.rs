use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::http_version::HttpVersion;
use crate::method::Method;

#[derive(Debug)]
struct HttpRequestParseError;

impl Display for HttpRequestParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to parse http request")
    }
}

impl Error for HttpRequestParseError {}

#[derive(Debug)]
pub struct HttpRequest {
    pub http_version: HttpVersion,
    pub method: Method,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
    pub path_params: HashMap<String, String>,
}

impl HttpRequest {

    pub fn parse(buffer: &mut BufReader<&TcpStream>) -> anyhow::Result<Self> {
        let mut buf_line = String::new();
        buffer.read_line(&mut buf_line)?;

        // Parse request line: GET /user-agent HTTP/1.1\r\n
        let mut it = buf_line.split_terminator("\r\n");
        let mut request_line = it.next().unwrap().split_whitespace();
        let method: Method = request_line.next().ok_or(HttpRequestParseError)?.parse()?;
        let path = request_line.next().ok_or(HttpRequestParseError)?.to_owned();
        let http_version: HttpVersion = request_line.next().ok_or(HttpRequestParseError)?.parse()?;

        // Parse headers
        let mut headers: HashMap<String, String> = HashMap::new();

        loop {
            buf_line.clear();
            buffer.read_line(&mut buf_line)?;
            let header = &buf_line;
            if header == "\r\n" || header.is_empty() {
                break
            }
            let (header, value) = header.split_once(":").unwrap();
            headers.insert(header.trim().to_owned(), value.trim().to_owned());
        }

        // Parse body.
        let mut body = None;
        if method == Method::POST && headers["Content-Type"] == "application/octet-stream" {
            let size: usize = headers["Content-Length"].parse().unwrap();
            let mut buf_body: Vec<u8> = Vec::with_capacity(size);
            unsafe { buf_body.set_len(size); }
            buffer.read_exact(&mut buf_body).unwrap();
            body = Some(buf_body);
        }

        Ok(Self {
            http_version,
            method,
            headers,
            body,
            path,
            path_params: HashMap::new(),
        })
    }
}
