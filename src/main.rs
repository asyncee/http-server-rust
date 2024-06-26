use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use std::thread;

use http_request::HttpRequest;
use router::Router;

mod http_response;
mod router;
mod http_request;
mod http_version;
mod views;
mod method;

fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&stream);
    let http_request = HttpRequest::parse(&mut buf_reader).expect("invalid http request");

    println!("{http_request:?}");

    let mut router = Router::new();

    router.get(r"^/$", views::index);
    router.get(r"^/echo/(?P<echo>.*)$", views::echo);
    router.get(r"^/user-agent$", views::user_agent);
    router.get(r"^/files/(?P<filename>.*)$", views::serve_file);
    router.post(r"^/files/(?P<filename>.*)$", views::create_file);
    router.get(r"^.*$", views::not_found);

    router.route(http_request, &mut stream);
}

fn get_directory() -> Option<String> {
    let args: Vec<String> = std::env::args().collect();
    let directory_index = args.iter().position(|r| r == "--directory");
    directory_index.map(|idx| args[idx + 1].to_owned())
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                thread::spawn(move || handle_connection(s));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
