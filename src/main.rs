use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
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
    let (path, method, headers, body) = reqreader(&mut reader);

    let response = if path == "/" {
        String::from("HTTP/1.1 200 OK\r\n\r\n")
    } else if path.starts_with("/echo/") {
        echo_handler(&path)
    } else if path.starts_with("/user-agent") {
        agent_handler(headers)
    } else if path.starts_with("/files/") && directory.is_some() {
        file_handler(&path, method, directory.unwrap(), body)
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

fn file_handler(path: &str, method: String, directory: String, body: Vec<u8>) -> String {
    println!("[file_handler] method: {}", &method);

    let filename = path.strip_prefix("/files/").unwrap();
    println!("[file_handler] Filename: {}", filename);

    let file_path = Path::new(&directory).join(filename);
    println!("[file_handler] file path: {:?}", file_path);

    // Check if the file exists and read it
    if method == "GET" {
        if let Ok(contents) = fs::read(&file_path) {
            println!("[file_handler] File found");
            format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}",
                contents.len(),
                String::from_utf8_lossy(&contents)
            )
        } else {
            eprint!("[file_handler]: File not found");
            String::from("HTTP/1.1 404 Not Found\r\n\r\n")
        }
    } else if method == "POST" {
        println!(
            "[file_handler] POST detected with body length: {}",
            body.len()
        );

        // Ensure parent directories exist
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).unwrap_or_default();
        }

        // Write the body to the file
        if let Err(e) = fs::write(&file_path, &body) {
            eprintln!("Error writing file: {}", e);
            return String::from("HTTP/1.1 500 Internal Server Error\r\n\r\n");
        }

        String::from("HTTP/1.1 201 Created\r\n\r\n")
    } else {
        String::from("HTTP/1.1 405 Method Not Allowed\r\n\r\n")
    }
}

fn reqreader<R: BufRead + Read>(
    reader: &mut R,
) -> (String, String, HashMap<String, String>, Vec<u8>) {
    let mut request_line = String::new();
    reader.read_line(&mut request_line).unwrap();
    request_line = request_line.trim().to_string();
    println!("request: {}", request_line);

    let method = request_line.split_whitespace().nth(0).unwrap().to_string();
    println!("method: {}", &method);

    let path = request_line.split_whitespace().nth(1).unwrap().to_string();
    println!("path: {}", path);

    let mut headers = std::collections::HashMap::new();
    let mut content_length = 0;

    // Read headers
    loop {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        let line = line.trim();

        if line.is_empty() {
            break;
        }

        // Split header at first colon
        if let Some((key, value)) = line.split_once(": ") {
            if key.to_lowercase() == "content-length" {
                content_length = value.parse::<usize>().unwrap_or(0);
            }
            headers.insert(key.to_string(), value.to_string());
        }
    }

    println!("Headers: {:?}", headers);

    // Read the body if content-length is specified
    let mut body = Vec::new();
    if content_length > 0 {
        // Read exact number of bytes as specified in content-length
        let mut buffer = vec![0; content_length];
        if let Ok(_) = reader.read_exact(&mut buffer) {
            body = buffer;
        }
    }

    (path, method, headers, body)
}
