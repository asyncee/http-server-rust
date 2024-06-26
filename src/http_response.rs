use std::collections::HashMap;
use std::fmt::Display;
use crate::http_version::HttpVersion;
use crate::http_request::HttpRequest;

enum HttpStatus {
    OK,
    NotFound,
    Created,
}

impl Display for HttpStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            HttpStatus::OK => "200 OK".to_owned(),
            HttpStatus::NotFound => "404 Not Found".to_owned(),
            HttpStatus::Created => "201 Created".to_owned(),
        };
        write!(f, "{}", str)
    }
}

pub struct HttpResponse {
    http_version: HttpVersion,
    status: HttpStatus,
    headers: HashMap<String, String>,
    body: Option<String>
}

impl HttpResponse {
    pub fn ok(http_request: &HttpRequest) -> Self {
        Self {
            http_version: http_request.http_version.clone(),
            status: HttpStatus::OK,
            headers: HashMap::new(),
            body: None,
        }
    }

    pub fn not_found(http_request: &HttpRequest) -> Self {
        Self {
            http_version: http_request.http_version.clone(),
            status: HttpStatus::NotFound,
            headers: HashMap::new(),
            body: None,
        }
    }

    pub fn created(http_request: &HttpRequest) -> Self {
        Self {
            http_version: http_request.http_version.clone(),
            status: HttpStatus::Created,
            headers: HashMap::new(),
            body: None,
        }
    }

    pub fn set_body(&mut self, body: String, content_type: String) {
        self.headers.insert("Content-Type".to_string(), content_type);
        self.headers.insert("Content-Length".to_string(), body.len().to_string());
        self.body = Some(body);
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut headers = String::new();
        for (k, v) in &self.headers {
            headers.push_str(format!("{k}: {v}\r\n").as_str());
        }
        let mut response = format!("{} {}\r\n{headers}\r\n", self.http_version, self.status);
        if let Some(body) = &self.body {
            response.push_str(body)
        }
        response.into()
    }
}
