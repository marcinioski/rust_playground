use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;
use std::fs;

enum RequestType {
    GET {val: String},
    POST(String),
    NONE,
}

fn parse_request(stream: &mut TcpStream) ->  RequestType {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        return RequestType::GET { val: String::from_utf8_lossy(&buffer[get.len()..]).to_string()};
    }

    RequestType::NONE
}

fn handle_connection(mut stream: TcpStream) {

    let mut contents: String = String::from("");

    match parse_request(&mut stream) {
        RequestType::GET{val} => contents = fs::read_to_string("static/hello.html").unwrap(),
        _ => (),
    }

    let response = format!("HTTP/1.1 200 OK\r\n\n{}", contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn run() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");

        handle_connection(stream);
    }
}

fn main() {
    println!("Hello, world!");
}
