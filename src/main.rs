use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use hello::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(move || {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let request_path = get_request_path(&mut stream);
    let (status_line, filename) = generate_response_content(request_path);
    send_response(&mut stream, status_line, filename);
}

fn get_request_path(stream: &mut TcpStream) -> String {
    let buf_reader = BufReader::new(stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    if let Some(first_line) = http_request.get(0) {
        first_line.split_whitespace().nth(1).unwrap_or("/").to_string()
    } else {
        "/".to_string()
    }
}

fn generate_response_content(request_path: String) -> (String, String) {
    if request_path == "/" {
        ("HTTP/1.1 200 OK".to_string(), "hello.html".to_string())
    } else if request_path == "/sleep" {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK".to_string(), "hello.html".to_string())
    } else {
        ("HTTP/1.1 404 NOT FOUND".to_string(), "404.html".to_string())
    }
}

fn send_response(stream: &mut TcpStream, status_line: String, filename: String) {
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
