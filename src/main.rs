use colored::Colorize;
use std::net::TcpListener;
use std::thread;
use std::time::Duration;
mod argsparser;
mod h_res;
mod req;

fn main() {
    let (port, directory, allow_write, timeout) = argsparser::getargs();
    run(port, directory.as_deref(), allow_write, timeout);
}

fn run(port: u16, directory: Option<&str>, allow_write: bool, timeout: u64) {
    println!("🚀 Starting server on port: {}", port);

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let msg = "Accepted new connection";
                println!("{}", msg.green());

                let timeout = Duration::from_secs(timeout);
                stream.set_read_timeout(Some(timeout)).unwrap();
                stream.set_write_timeout(Some(timeout)).unwrap();

                let dir = directory.map(|s| s.to_string());
                thread::spawn(move || loop {
                    let should_close = req::handle_req(&mut stream, &dir, allow_write);

                    if should_close {
                        break;
                    }
                });
            }
            Err(e) => {
                eprintln!("error: {}", e);
            }
        }
    }
}
