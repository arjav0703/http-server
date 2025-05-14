use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("0.0.0.0:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                handle_req(stream);
            }
            Err(e) => {
                eprintln!("error: {}", e);
            }
        }
    }
}

fn handle_req(mut stream: TcpStream) {
    let reader = BufReader::new(&stream);

    let request = reader.lines().next().unwrap().unwrap();
    println!("request: {}", request);

    let path = request.split_whitespace().nth(1).unwrap();
    println!("path: {}", path);

    let response = if path == "/" {
        String::from("HTTP/1.1 200 OK\r\n\r\nHello, world!")
    } else if path.starts_with("/echo/") {
        println!("echo detected in path");
        let echo = path.split("/");
        println!("{:?}", echo);
        String::from("HTTP/1.1 404 NOT FOUND\r\n\r\n404 Not Found")
    } else {
        String::from("HTTP/1.1 404 NOT FOUND\r\n\r\n404 Not Found")
    };

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
