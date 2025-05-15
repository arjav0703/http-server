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
    let mut lines = reader.lines();

    let request = lines.next().unwrap().unwrap();
    println!("request: {}", request);

    let path = request.split_whitespace().nth(1).unwrap();
    println!("path: {}", path);

    let mut headers = std::collections::HashMap::new();
    for line in lines {
        let line = line.unwrap();
        if line.is_empty() {
            // Empty line marks the end of headers
            break;
        }

        // Split header at first colon
        if let Some((key, value)) = line.split_once(": ") {
            headers.insert(key.to_string(), value.to_string());
        }
    }

    println!("Headers: {:?}", headers);

    let response = if path == "/" {
        String::from("HTTP/1.1 200 OK\r\n\r\nHello, world!")
    } else if path.starts_with("/echo/") {
        println!("echo detected in path");

        let echo = path.split("/").nth(2).unwrap();
        println!("{}", echo);

        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            echo.len(),
            echo
        )
    } else if path.starts_with("/user-agent") {
        println!("user-agent detected in path");
        format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: \r\n\r\n",)
    } else {
        String::from("HTTP/1.1 404 Not Found\r\n\r\n404 Not Found")
    };

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
