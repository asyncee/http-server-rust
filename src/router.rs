use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;

use regex::{Captures, Regex};

use crate::http_request::HttpRequest;
use crate::http_response::HttpResponse;
use crate::method::Method;

type HttpView = fn(HttpRequest) -> HttpResponse;

struct Route
{
    path: Regex,
    func: HttpView,
}

impl Route
{
    pub fn new(path: Regex, func: HttpView) -> Self {
        Self {
            path,
            func,
        }
    }

    pub fn matches(&self, path: &str) -> bool {
        return self.path.is_match(path);
    }

    pub fn handle(&self, mut request: HttpRequest, stream: &mut TcpStream) {
        let captures: Captures = self.path.captures(&request.path).unwrap();
        let mut path_params: HashMap<String, String> = HashMap::new();
        for group_name in self.path.capture_names() {
            if let Some(name) = group_name {
                path_params.insert(name.to_string(), captures[name].to_string());
            }
        }
        request.path_params = path_params;
        let response = (self.func)(request);
        stream.write_all(&response.to_bytes()).unwrap();
    }
}

pub struct Router
{
    routes: HashMap<Method, Vec<Route>>,
}

impl Router
{
    pub fn new() -> Self {
        Self {
            routes: HashMap::from_iter([
                (Method::GET, vec![]),
                (Method::POST, vec![]),
            ])
        }
    }

    pub fn get(&mut self, path: &str, func: HttpView) {
        let routes = self.routes.get_mut(&Method::GET).unwrap();
        routes.push(Route::new(Regex::new(path).unwrap(), func));
    }

    pub fn post(&mut self, path: &str, func: HttpView) {
        let routes = self.routes.get_mut(&Method::POST).unwrap();
        routes.push(Route::new(Regex::new(path).unwrap(), func));
    }

    pub fn route(&self, request: HttpRequest, stream: &mut TcpStream) {
        for route in self.routes[&request.method].iter() {
            if route.matches(&request.path) {
                route.handle(request, stream);
                break;
            }
        }
    }
}
