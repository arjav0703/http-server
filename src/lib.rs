use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::path::Path;
use colored::Colorize;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::env;

pub fn getargs() -> (Option<String>, Option<String>, bool) {
    let args: Vec<String> = env::args().collect();

    let mut port: Option<String> = None;
    let mut directory: Option<String> = None;
    let mut allow_write= false ;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--port" => {
                if i + 1 < args.len() {
                    port = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--directory" => {
                if i + 1 < args.len() {
                    directory = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--allow-write" => {
                allow_write = true;
            }
            _ => {}
        }
        i += 1;
    }

    if port.is_none() {
        port = Some("8080".to_string());
    }
    if directory.is_none() {
        directory = Some(".".to_string());
    }

    if !allow_write{
        println!("{}", String::from("NOTE: Write access is not allowed").red().bold());
    } else {
        println!("{}", String::from("NOTE: Write access is allowed").red().bold());
    }

    (port, directory, allow_write)
}

pub fn handle_req( stream: &mut TcpStream, directory: &Option<String>, allow_write:bool) -> bool {
    let reader = BufReader::new(&mut *stream);
    let (path, method, headers, body) = reqreader(reader);

    let mut response = if path == "/" {
        landing_page()
    } else if path.starts_with("/echo/") {
        echo_handler(&path)
    } else if path.starts_with("/user-agent") {
        agent_handler(&headers)
    } else if path.starts_with("/files/") && directory.is_some() {
        file_handler(&path, &method, &directory.clone().unwrap(), body, allow_write)
    } else {
        HttpResponse::new("404 Not Found")
    };

    if let Some(encoding) = headers.get("Accept-Encoding") {
        response.add_header("Accept-Encoding", encoding);
        if encoding.contains("gzip") {
        response.add_header("Content-Encoding", "gzip");
        }
    }
  
    let connection_close = headers.get("Connection")
        .map(|v| v.to_lowercase() == "close")
        .unwrap_or(false);

    if connection_close {
        response.add_header("Connection", "close");
    } else {
        response.add_header("Connection", "keep-alive");
    }
 
    let response_bytes = response.as_bytes();
    // println!("Response:\n{}", String::from_utf8_lossy(&response_bytes).green().bold());
    
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
    // println!("{} : {}", "[Echo handler]:".blue(), echo.yellow().bold());

    let mut response = HttpResponse::new("200 OK");
    response.add_header("Content-Type", "text/plain");
    response.set_body(echo.as_bytes());

    response
}

fn agent_handler(headers: &HashMap<String, String>) -> HttpResponse {
    let unknown = "Unknown".to_string();
    let user_agent = headers.get("User-Agent").unwrap_or(&unknown);

    println!("[Agent-handler] User-Agent: {}", user_agent.bright_yellow().bold());

    let mut response = HttpResponse::new("200 OK");
    response.add_header("Content-Type", "text/plain");
    response.set_body(user_agent.as_bytes());

    response
}

fn file_handler(path: &str, method: &str, directory: &String, body: Vec<u8>, allow_write:bool) -> HttpResponse {
    println!("[file_handler] method: {}", method.red().bold());

    let filename = match path.strip_prefix("/files/") {
        Some(f) => f,
        None => {
            eprintln!("[file_handler] Invalid path format: {}", path);
            return HttpResponse::new("400 Bad Request");
        }
    };
    // println!("[file_handler] Filename: {}", filename.yellow().italic());

    let file_path = Path::new(&directory).join(filename);
    // println!("[file_handler] File path: {:?}", file_path);

    match method {
        "GET" => {
            match fs::read(&file_path) {
                Ok(contents) => {
                    println!("[file_handler] File found");
                    let mut response = HttpResponse::new("200 OK");
                    response.add_header("Content-Type", "application/octet-stream");
                    response.set_body(&contents);
                    response
                }
                Err(_) => {
                    eprintln!("[file_handler] File not found");
                    HttpResponse::new("404 Not Found")
                }
            }
        }

        "POST" => {
            // println!("[file_handler] POST detected with body length: {}", body.len());
            dbg!(&allow_write);
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

fn reqreader<R: BufRead + Read>(
    mut reader:  R,
) -> (String, String, HashMap<String, String>, Vec<u8>) {
    let mut request_line = String::new();
    reader.read_line(&mut request_line).unwrap();
    request_line = request_line.trim().to_string();
    println!("request: {}", request_line);

    let method = request_line.split_whitespace().nth(0).unwrap().to_string();
    println!("method: {}", &method);

    let path = request_line.split_whitespace().nth(1).unwrap().to_string();
    println!("path: {}", path);

    let mut headers = HashMap::new();
    let mut content_length = 0;

    // Read headers
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

    println!("Headers: {:?}", headers);

    // Read body
    let mut body = Vec::new();
    if content_length > 0 {
        let mut buffer = vec![0; content_length];
        if reader.read_exact(&mut buffer).is_ok() {
            body = buffer;
        }
    }

    (path, method, headers, body)
}

struct HttpResponse {
    status: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl HttpResponse {
    fn new(status: &str) -> Self {
        HttpResponse {
            status: status.to_string(),
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }

    fn add_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    fn set_body(&mut self, body: &[u8]) {
        self.body = body.to_vec();
        self.headers
            .insert("Content-Length".to_string(), self.body.len().to_string());
    }

    fn as_bytes(&self) -> Vec<u8> {
        let mut body = self.body.clone();
        let mut headers = self.headers.clone();

        if let Some(encoding) = headers.get("Content-Encoding") {
            if encoding == "gzip" {
                let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
                encoder.write_all(&body).unwrap();
                body = encoder.finish().unwrap();
                headers.insert("Content-Length".to_string(), body.len().to_string());
            }
        }
        let mut response = format!("HTTP/1.1 {}\r\n", self.status);

        for (key, value) in &headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }

        response.push_str("\r\n");

        let mut bytes = response.into_bytes();
        bytes.extend(&body);
        bytes
    }
}

