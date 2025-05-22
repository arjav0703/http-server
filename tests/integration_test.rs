use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::{Command, Child};
use std::{thread, time::Duration};

fn start_server() -> Child {
    Command::new("cargo")
        .args(&["run", "--", "--port", "8081", "--directory", ".", "--allow-write", "--timeout", "5"])
        .spawn()
        .expect("Failed to start server")
}

fn send_request(request: &str) -> String {
    let mut stream = TcpStream::connect("127.0.0.1:8081").unwrap();
    stream.write_all(request.as_bytes()).unwrap();
    stream.shutdown(std::net::Shutdown::Write).unwrap();

    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();
    response
}

#[test]
fn test_landing_page() {
    let _server = start_server();
    thread::sleep(Duration::from_secs(2)); // Give server time to start

    let request = "GET / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n";
    let response = send_request(request);

    assert!(response.contains("HTTP/1.1 200 OK"));
}

#[test]
fn test_echo_handler() {
    let request = "GET /echo/hello HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n";
    let response = send_request(request);

    assert!(response.contains("HTTP/1.1 200 OK"));
    assert!(response.ends_with("hello"));
}

#[test]
fn test_user_agent() {
    let request = "GET /user-agent HTTP/1.1\r\nHost: localhost\r\nUser-Agent: TestClient\r\nConnection: close\r\n\r\n";
    let response = send_request(request);

    assert!(response.contains("HTTP/1.1 200 OK"));
    assert!(response.ends_with("TestClient"));
}

#[test]
fn test_file_write_and_read() {
    let _server = start_server();
    thread::sleep(Duration::from_secs(2));

    // POST (write)
    let post_body = "Sample file content";
    let post_request = format!(
        "POST /files/testfile.txt HTTP/1.1\r\nHost: localhost\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        post_body.len(),
        post_body
    );
    let post_response = send_request(&post_request);
    assert!(post_response.contains("HTTP/1.1 201 Created"));

    // GET (read)
    let get_request = "GET /files/testfile.txt HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n";
    let get_response = send_request(get_request);
    assert!(get_response.contains("HTTP/1.1 200 OK"));
    assert!(get_response.ends_with(post_body));
}
