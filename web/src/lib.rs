use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::time::Duration;
use std::{fs, thread};

pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    println!("{}", request_line);

    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("hello.html").unwrap();
        let length = format!("Content-Length:{}\r\n\r\n", contents.len());

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n\n{contents}");
        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let length = format!("Content-Length:{}\r\n\r\n", contents.len());

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n\n{contents}");
        stream.write_all(response.as_bytes()).unwrap();
    }
}

pub fn handle_connection2(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = format!("Content-Length:{}\r\n\r\n", contents.len());

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}

// 模拟慢请求
pub fn handle_connection_slow(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(10));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = format!("Content-Length:{}\r\n\r\n", contents.len());

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
