use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let request_path = get_request_path(&mut stream);
    let (status_line, contents) = generate_response_content(request_path);

    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}

fn get_request_path(stream: &mut TcpStream) -> String {
    let buf_reader = BufReader::new(stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    http_request[0].split_whitespace().nth(1).unwrap_or("/").to_string()
}

fn generate_response_content(request_path: String) -> (String, String) {
    if request_path == "/" {
        ("HTTP/1.1 200 OK".to_string(), fs::read_to_string("hello.html").unwrap())
    } else {
        ("HTTP/1.1 404 Not Found".to_string(), fs::read_to_string("404.html").unwrap())
    }
}