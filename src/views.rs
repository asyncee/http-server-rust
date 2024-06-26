use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use crate::http_request::HttpRequest;
use crate::http_response::HttpResponse;

pub fn index(http_request: HttpRequest) -> HttpResponse {
    HttpResponse::ok(&http_request)
}

pub fn not_found(http_request: HttpRequest) -> HttpResponse {
    HttpResponse::not_found(&http_request)
}

pub fn echo(http_request: HttpRequest) -> HttpResponse {
    let word = &http_request.path_params["echo"];
    let mut response = HttpResponse::ok(&http_request);
    response.set_body(word.as_str().to_owned(), "text/plain".to_owned());
    response
}

pub fn user_agent(http_request: HttpRequest) -> HttpResponse {
    let mut response = HttpResponse::ok(&http_request);
    response.set_body(http_request.headers.get("User-Agent").unwrap().to_owned(), "text/plain".to_owned());
    response
}

pub fn serve_file(http_request: HttpRequest) -> HttpResponse {
    let directory = crate::get_directory();
    match directory {
        Some(dir) => {
            let mut path = PathBuf::new();
            path.push(dir);
            path.push(&http_request.path_params["filename"]);
            let file = File::open(path);

            match file {
                Ok(mut f) => {
                    let mut response = HttpResponse::ok(&http_request);
                    let mut contents = String::new();
                    f.read_to_string(&mut contents).unwrap();
                    response.set_body(contents, "application/octet-stream".to_owned());
                    response
                }
                Err(_) => HttpResponse::not_found(&http_request)
            }
        }
        None => HttpResponse::not_found(&http_request)
    }
}


pub fn create_file(http_request: HttpRequest) -> HttpResponse {
    let directory = crate::get_directory();
    match directory {
        Some(dir) => {
            let mut path = PathBuf::new();
            path.push(dir);
            path.push(&http_request.path_params["filename"]);
            let file = File::create(path);
            match file {
                Ok(mut f) => {
                    match &http_request.body {
                        Some(body) => {
                            f.write_all(&body).unwrap();
                            HttpResponse::created(&http_request)
                        }
                        None => HttpResponse::ok(&http_request),
                    }
                }
                Err(_) => HttpResponse::ok(&http_request)
            }
        }
        None => HttpResponse::ok(&http_request)
    }
}