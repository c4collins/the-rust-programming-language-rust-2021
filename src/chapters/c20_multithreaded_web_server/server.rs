use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
};

use crate::chapters::c20_multithreaded_web_server::http;
use crate::chapters::c20_multithreaded_web_server::routing;
use crate::chapters::c20_multithreaded_web_server::templates;
use crate::chapters::c20_multithreaded_web_server::threading;

pub fn run(host: &str, port: u16) {
    let listener = get_listener(host, port);
    let pool = threading::ThreadPool::build(6).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| handle_connection(stream));
    }

    println!("Server is shutting down!");
}

fn get_listener(host: &str, port: u16) -> TcpListener {
    let address = format!("{host}:{port}");
    println!("Starting server at {address}");
    TcpListener::bind(address).unwrap()
}

/// Turns a request into a response
fn handle_connection(mut stream: TcpStream) {
    let request = process_request(&mut stream);
    let response = process_response(&request);
    stream.write_all(response.as_string().as_bytes()).unwrap();
}

fn process_request(stream: &mut TcpStream) -> String {
    let buf_reader = BufReader::new(stream);
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect(); // TODO: consume headers
    let request = http_request[0].clone();

    // println!("Request: {:#?}, {}", http_request, request);

    request
}

fn process_response(request: &str) -> http::HttpResponse {
    let (_, filename) = routing::get(&request[..]);
    let status = http::HttpStatus::new(200, "OK");
    let (contents, _) = templates::get(filename);
    let response = http::HttpResponse::new(status, contents);

    // println!("Response:\n\n{}", response.as_string());

    response
}
