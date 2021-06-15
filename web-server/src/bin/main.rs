use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use httparse::Request;
use webserver::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut request = httparse::Request::new(&mut headers);
    request.parse(&buffer).unwrap();

    match request.method {
        Some(x) => match x {
            "GET" => handle_get_request(request, stream),
            _ => {
                println!("We cant handle {} requests yet", x);
                handle_unknown_request(stream)
            }
        },
        None => handle_unknown_request(stream),
    }
}

fn handle_get_request(request: Request, stream: TcpStream) {
    match request.path {
        Some(x) => match x {
            "/" => {
                let response =
                    create_response_with_file_and_status("hello.html", "HTTP/1.1 200 OK");
                send_response(stream, response)
            }
            _ => {
                println!("No page for path {} found", x);
                handle_unknown_request(stream)
            }
        },
        None => handle_unknown_request(stream),
    }
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
