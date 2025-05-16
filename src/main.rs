use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::thread;

fn main() {
    println!("Logs from your program will appear here!");

    // Parse CLI arguments
    let args: Vec<String> = env::args().collect();
    let mut directory = None;

    for i in 0..args.len() - 1 {
        if args[i] == "--directory" {
            directory = Some(&args[i + 1]);
        }
    }

    let listener = TcpListener::bind("0.0.0.0:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Accepted new connection");

                let dir = directory.map(|s| s.to_string());

                thread::spawn(move || {
                    handle_req(stream, dir);
                });
            }
            Err(e) => {
                eprintln!("error: {}", e);
            }
        }
    }
}

fn handle_req(mut stream: TcpStream, directory: Option<String>) {
    let mut reader = BufReader::new(&stream);
    let (path, method, headers) = reqreader(&mut reader);

    let response = if path == "/" {
        String::from("HTTP/1.1 200 OK\r\n\r\n")
    } else if path.starts_with("/echo/") {
        echo_handler(&path)
    } else if path.starts_with("/user-agent") {
        agent_handler(headers)
    } else if path.starts_with("/files/") && directory.is_some() {
        file_handler(&path, method, directory.unwrap())
    } else {
        String::from("HTTP/1.1 404 Not Found\r\n\r\n")
    };

    stream.write_all(response.as_bytes()).unwrap();
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

fn file_handler(path: &str, method: String, directory: String) -> String {
    println!("[file_handler] method: {}", &method);

    let filename = path.strip_prefix("/files/").unwrap();
    println!("[file_handler] Filename: {}", filename);

    let file_path = Path::new(&directory).join(filename);
    println!("[file_handler] file path: {:?}", file_path);

    // Check if the file exists and read it
    if let Ok(contents) = fs::read(&file_path) {
        println!("[file_handler] File found");
        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            String::from_utf8_lossy(&contents)
        )
    } else if method == String::from("POST") {
        println!("[file_handler] POST detected");
        String::from("HTTP/1.1 201 Created\r\n\r\n")
    } else {
        eprint!("[file_handler]: File not found");
        String::from("HTTP/1.1 404 Not Found\r\n\r\nThe following files are available:")
    }
}

fn reqreader<R: BufRead>(reader: &mut R) -> (String, String, HashMap<String, String>) {
    let mut lines = reader.lines();

    let request = lines.next().unwrap().unwrap();
    println!("request: {}", request);

    let method = request.split_whitespace().nth(0).unwrap().to_string();
    println!("method: {}", &method);

    let path = request.split_whitespace().nth(1).unwrap().to_string();
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

    (path, method, headers)
}
