use crate::h_res::HttpResponse;
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::path::Path;
pub fn handle_req(stream: &mut TcpStream, directory: &Option<String>, allow_write: bool) -> bool {
    let reader = BufReader::new(&mut *stream);
    let (path, method, headers, body) = reqreader(reader);

    let mut response = if path == "/" {
        landing_page()
    } else if path.starts_with("/echo/") {
        echo_handler(&path)
    } else if path.starts_with("/user-agent") {
        agent_handler(&headers)
    } else if path.starts_with("/") && directory.is_some() {
        file_handler(
            &path,
            &method,
            &directory.clone().unwrap(),
            body,
            allow_write,
        )
    } else {
        HttpResponse::new("404 Not Found")
    };

    if let Some(encoding) = headers.get("Accept-Encoding") {
        response.add_header("Accept-Encoding", encoding);
        if encoding.contains("gzip") {
            response.add_header("Content-Encoding", "gzip");
        }
    }

    let connection_close = headers
        .get("Connection")
        .map(|v| v.to_lowercase() == "close")
        .unwrap_or(false);

    if connection_close {
        response.add_header("Connection", "close");
    } else {
        response.add_header("Connection", "keep-alive");
    }

    let response_bytes = response.as_bytes();

    stream.write_all(&response_bytes).unwrap();
    stream.flush().unwrap();

    connection_close
}

fn landing_page() -> HttpResponse {
    let mut response = HttpResponse::new("200 OK");
    response.add_header("Content-Type", "text/html");

    let lander = Path::new("index.html");
    let indexpage = fs::read_to_string(lander).unwrap_or_else(|_| {
        eprintln!("[landing_page] Error reading index.html");
        "<html><body><h1>Make sure that that index.html exists</h1></body></html>".to_string()
    });

    response.set_body(indexpage.as_bytes());
    response
}

fn echo_handler(path: &str) -> HttpResponse {
    let echo = path.split('/').nth(2).unwrap_or("");

    let mut response = HttpResponse::new("200 OK");
    response.add_header("Content-Type", "text/plain");
    response.set_body(echo.as_bytes());

    response
}

fn agent_handler(headers: &HashMap<String, String>) -> HttpResponse {
    let unknown = "Unknown".to_string();
    let user_agent = headers.get("User-Agent").unwrap_or(&unknown);

    let mut response = HttpResponse::new("200 OK");
    response.add_header("Content-Type", "text/plain");
    response.set_body(user_agent.as_bytes());

    response
}

fn file_handler(
    path: &str,
    method: &str,
    directory: &String,
    body: Vec<u8>,
    allow_write: bool,
) -> HttpResponse {
    let file_path = Path::new(&directory).join(path.trim_start_matches('/'));
    if let Err(resp) = file_restrictor(path) {
        return resp;
    }

    match method {
        "GET" => match fs::read(&file_path) {
            Ok(contents) => {
                let mut response = HttpResponse::new("200 OK");
                response.add_header("Content-Type", "application/octet-stream");
                response.set_body(&contents);
                response
            }
            Err(_) => {
                eprintln!("[file_handler] File not found");
                HttpResponse::new("404 Not Found")
            }
        },

        "POST" => {
            if !allow_write {
                eprintln!("[file_handler] Write access denied");
                return HttpResponse::new("403 Forbidden");
            }

            if let Some(parent) = file_path.parent() {
                if let Err(e) = fs::create_dir_all(parent) {
                    eprintln!("[file_handler] Failed to create directories: {}", e);
                    return HttpResponse::new("500 Internal Server Error");
                }
            }

            match fs::write(&file_path, &body) {
                Ok(_) => HttpResponse::new("201 Created"),
                Err(e) => {
                    eprintln!("[file_handler] Error writing file: {}", e);
                    HttpResponse::new("500 Internal Server Error")
                }
            }
        }

        _ => {
            eprintln!("[file_handler] Method not allowed: {}", method);
            HttpResponse::new("405 Method Not Allowed")
        }
    }
}

fn file_restrictor(filename: &str) -> Result<(), HttpResponse> {
    if filename.starts_with('.') || filename.starts_with('_') {
        eprintln!(
            "[file_restrictor] Request Denied for: {} as it begins with a '.'",
            filename
        );
        return Err(HttpResponse::new("403 Forbidden"));
    }
    Ok(())
}

fn reqreader<R: BufRead + Read>(
    mut reader: R,
) -> (String, String, HashMap<String, String>, Vec<u8>) {
    let mut request_line = String::new();
    reader.read_line(&mut request_line).unwrap();
    request_line = request_line.trim().to_string();
    println!("request: {}", request_line);

    let method = request_line.split_whitespace().next().unwrap().to_string();

    let path = request_line.split_whitespace().nth(1).unwrap().to_string();

    let mut headers = HashMap::new();
    let mut content_length = 0;

    loop {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        let line = line.trim();

        if line.is_empty() {
            break;
        }

        if let Some((key, value)) = line.split_once(": ") {
            if key.to_lowercase() == "content-length" {
                content_length = value.parse::<usize>().unwrap_or(0);
            }
            headers.insert(key.to_string(), value.to_string());
        }
    }

    let mut body = Vec::new();
    if content_length > 0 {
        let mut buffer = vec![0; content_length];
        if reader.read_exact(&mut buffer).is_ok() {
            body = buffer;
        }
    }

    (path, method, headers, body)
}
