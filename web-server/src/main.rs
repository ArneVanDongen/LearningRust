use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        handle_get_request(stream);
    } else {
        println!("We dont know how to handle the following request yet:\n{}", 
            String::from_utf8_lossy(&buffer[..]));
        handle_unknown_request(stream);
    }
}

fn handle_get_request(stream: TcpStream) {
    let response = create_response_with_file_and_status("hello.html", "HTTP/1.1 200 OK");
    send_response(stream, response);
}

fn handle_unknown_request(stream: TcpStream) {
    let response = create_response_with_file_and_status("404.html", "HTTP/1.1 404 NOT FOUND");
    send_response(stream, response);
}

fn send_response(mut stream: TcpStream, response: String) {
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn create_response_with_file_and_status(file_name: &str, status_line: &str) -> String {
    let contents = fs::read_to_string(file_name).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    response
}
