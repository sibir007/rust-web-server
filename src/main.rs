// This is a simple web server that responds with a hello.html file when a GET request is made to the root path, and a 404.html file when a GET request is made to any other path.
use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    fs,
    thread,
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        pool.execute(|| {
            handle_connection(stream);
        });

        // thread::spawn(|| {
        //     handle_connection(stream);
        // });
        // handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, file_name) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK\r\n\r\n", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html"),
    };

    let response = format!("{}{}", status_line, fs::read_to_string(file_name).unwrap());
    stream.write_all(response.as_bytes()).unwrap();

}

// fn respond_with_hello(mut stream: TcpStream) {
//     let response = "HTTP/1.1 200 OK\r\n\r\n";
//     let body = fs::read_to_string("hello.html").unwrap();

//     let response = format!("{}{}", response, body);
//     stream.write_all(response.as_bytes()).unwrap();
// }

// fn respond_with_404(mut stream: TcpStream) {
//     let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
//     let body = fs::read_to_string("404.html").unwrap();

//     let response = format!("{}{}", response, body);
//     stream.write_all(response.as_bytes()).unwrap();
// }
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