use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::path::Path;

use crate::h_res::HttpResponse;

/// Top‐level request dispatcher.  
/// Returns true if the connection should close.
pub fn handle_req(stream: &mut TcpStream, directory: &Option<String>, allow_write: bool) -> bool {
    // parse request
    let mut reader = BufReader::new(stream.try_clone().expect("Failed to clone stream"));
    let request = match parse_request(&mut reader) {
        Ok(req) => req,
        Err(res) => {
            let bytes = res.as_bytes();
            stream.write_all(&bytes).ok();
            stream.flush().ok();
            return true;
        }
    };

    // route
    let mut response = route_request(&request, directory, allow_write);

    // encoding if requested
    if let Some(accept) = request.headers.get("Accept-Encoding") {
        response.add_header("Accept-Encoding", accept);
        if accept.contains("gzip") {
            response.add_header("Content-Encoding", "gzip");
        }
    }

    // persistence
    let close_conn = request
        .headers
        .get("Connection")
        .map(|v| v.eq_ignore_ascii_case("close"))
        .unwrap_or(false);
    response.add_header(
        "Connection",
        if close_conn { "close" } else { "keep-alive" },
    );

    // send res
    let resp_bytes = response.as_bytes();
    stream
        .write_all(&resp_bytes)
        .unwrap_or_else(|e| eprintln!("Write error: {}", e));
    stream
        .flush()
        .unwrap_or_else(|e| eprintln!("Flush error: {}", e));

    close_conn
}

/// represents the parsed HTTP request
struct Request {
    method: String,
    path: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

fn parse_request<R: BufRead + Read>(reader: &mut R) -> Result<Request, HttpResponse> {
    let mut request_line = String::new();
    reader
        .read_line(&mut request_line)
        .map_err(|_| HttpResponse::new("400 Bad Request"))?;
    let request_line = request_line.trim();

    let mut parts = request_line.split_whitespace();
    let method = parts
        .next()
        .ok_or_else(|| HttpResponse::new("400 Bad Request"))?
        .to_string();
    let path = parts
        .next()
        .ok_or_else(|| HttpResponse::new("400 Bad Request"))?
        .to_string();

    let mut headers = HashMap::new();
    let mut content_length = 0;

    loop {
        let mut line = String::new();
        reader
            .read_line(&mut line)
            .map_err(|_| HttpResponse::new("400 Bad Request"))?;
        let line = line.trim_end();
        if line.is_empty() {
            break;
        }
        if let Some((name, value)) = line.split_once(": ") {
            if name.eq_ignore_ascii_case("content-length") {
                content_length = value.parse().unwrap_or(0);
            }
            headers.insert(name.to_string(), value.to_string());
        }
    }

    let mut body = vec![0; content_length];
    if content_length > 0 {
        reader
            .read_exact(&mut body)
            .map_err(|_| HttpResponse::new("400 Bad Request"))?;
    }

    Ok(Request {
        method,
        path,
        headers,
        body,
    })
}

fn route_request(req: &Request, directory: &Option<String>, allow_write: bool) -> HttpResponse {
    match req.path.as_str() {
        "/" => landing_page(),

        "/user-agent" => agent_handler(&req.headers),

        p if p.starts_with("/user-agent") => agent_handler(&req.headers),

        p if p.starts_with("/echo/") => echo_handler(p),

        p if p.starts_with('/') && directory.is_some() => {
            let dir = directory.as_ref().unwrap();
            file_handler(p, &req.method, dir, &req.body, allow_write)
        }

        _ => HttpResponse::new("404 Not Found"),
    }
}

fn landing_page() -> HttpResponse {
    let mut res = HttpResponse::new("200 OK");
    res.add_header("Content-Type", "text/html");

    let content = fs::read_to_string("index.html").unwrap_or_else(|_| {
        eprintln!("Could not read index.html, using fallback");
        "<html><body><h1>index.html missing</h1></body></html>".into()
    });
    res.set_body(content.as_bytes());
    res
}

fn echo_handler(path: &str) -> HttpResponse {
    let echo = path.splitn(3, '/').nth(2).unwrap_or("");
    let mut res = HttpResponse::new("200 OK");
    res.add_header("Content-Type", "text/plain");
    res.set_body(echo.as_bytes());
    res
}

fn agent_handler(headers: &HashMap<String, String>) -> HttpResponse {
    let ua = headers
        .get("User-Agent")
        .cloned()
        .unwrap_or_else(|| "Unknown".into());

    let mut res = HttpResponse::new("200 OK");
    res.add_header("Content-Type", "text/plain");
    res.set_body(ua.as_bytes());
    res
}

/// Handles GET/POST for file‐backed resources under `directory`
fn file_handler(
    path: &str,
    method: &str,
    base_dir: &str,
    body: &[u8],
    allow_write: bool,
) -> HttpResponse {
    if let Err(resp) = restrict_path(path) {
        return resp;
    }

    let full_path = Path::new(base_dir).join(path.trim_start_matches('/'));

    match method {
        "GET" => serve_file(&full_path),
        "POST" => {
            if !allow_write {
                return HttpResponse::new("403 Forbidden");
            }
            write_file(&full_path, body)
        }
        _ => HttpResponse::new("405 Method Not Allowed"),
    }
}

// todo. implement this with cli args
fn restrict_path(p: &str) -> Result<(), HttpResponse> {
    if let Some(name) = Path::new(p).file_name().and_then(|n| n.to_str()) {
        if name.starts_with('.') || name.starts_with('_') {
            return Err(HttpResponse::new("403 Forbidden"));
        }
    }
    Ok(())
}

fn serve_file(path: &Path) -> HttpResponse {
    match fs::read(path) {
        Ok(contents) => {
            let mut res = HttpResponse::new("200 OK");
            res.add_header("Content-Type", "application/octet-stream");
            res.set_body(&contents);
            res
        }
        Err(_) => HttpResponse::new("404 Not Found"),
    }
}

fn write_file(path: &Path, content: &[u8]) -> HttpResponse {
    if let Some(parent) = path.parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            eprintln!("mkdir failed: {}", e);
            return HttpResponse::new("500 Internal Server Error");
        }
    }
    match fs::write(path, content) {
        Ok(_) => HttpResponse::new("201 Created"),
        Err(e) => {
            eprintln!("File write error: {}", e);
            HttpResponse::new("500 Internal Server Error")
        }
    }
}
