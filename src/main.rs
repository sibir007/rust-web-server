use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;
// use std::io::prelude::*;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
    

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        // .map(|mut line| {line.push_str("\n-----------------------------------------------"); line})
        // .map(|mut line| line.write_str("\n-----------------------------------------------"))
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let content = fs::read_to_string("hello.html").unwrap();
    let length = content.len();
    let response =  format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, length, content);
    stream.write_all(response.as_bytes()).unwrap();
}

// fn handle_connection(mut stream: TcpStream) {
//     let buf_reader = BufReader::new(&stream);
//     let http_request: Vec<_> = buf_reader
//         .lines()
//         .map(|line| line.unwrap())
//         .take_while(|line| !line.is_empty())
//         // .map(|mut line| {line.push_str("\n-----------------------------------------------"); line})
//         // .map(|mut line| line.write_str("\n-----------------------------------------------"))
//         .collect();

//     println!("Request: {:?}", http_request);
// }