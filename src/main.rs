use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("0.0.0.0:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Accepted new connection");

                // Spawn a new thread to handle each connection
                thread::spawn(|| {
                    handle_req(stream);
                });
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
            break;
        }

        // Split header at first colon
        if let Some((key, value)) = line.split_once(": ") {
            headers.insert(key.to_string(), value.to_string());
        }
    }

    println!("Headers: {:?}", headers);

    let response = if path == "/" {
        String::from("HTTP/1.1 200 OK\r\n\r\n")
    } else if path.starts_with("/echo/") {
        echo_handler(path)
    } else if path.starts_with("/user-agent") {
        agent_handler(headers)
    } else {
        String::from("HTTP/1.1 404 Not Found\r\n\r\n404 Not Found")
    };

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn echo_handler(path: &str) -> String {
    println!("echo detected in path");
    let echo = path.split("/").nth(2).unwrap();
    println!("{}", echo);

    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        echo.len(),
        echo
    )
}

fn agent_handler(headers: HashMap<String, String>) -> String {
    println!("User-Agent detected in path");
    let user_agent = headers.get("User-Agent").unwrap();

    println!("User-Agent: {}", user_agent);
    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        user_agent.len(),
        user_agent
    )
}
