use codecrafters_http_server::handle_req;
use std::thread;
use std::net::{TcpListener};
use std::env;

fn main() {
    let port = "8080";
    run(port);

    println!("Server started on port {}", port);
}

fn run(port:&str) {
    let args: Vec<String> = env::args().collect();
    let mut directory = None;

    for i in 0..args.len() - 1 {
        if args[i] == "--directory" {
            directory = Some(&args[i + 1]);
        }
    }

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();

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
