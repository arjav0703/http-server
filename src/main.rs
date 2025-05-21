use http_server::handle_req;
use http_server::getargs;
use std::thread;
use std::net::{TcpListener};
use colored::Colorize;

fn main() {
    
    let (port, directory, allow_write) = getargs();
    run(&port.expect("Using default port"), directory.as_deref(), allow_write);

}

fn run(port:&String, directory: Option<&str>, allow_write: bool) {
    println!("🚀 Starting server on port: {}", port);

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let msg = "Accepted new connection";
                println!("{}", msg.green());

                let dir = directory.map(|s| s.to_string());
                thread::spawn(move || {
                loop {
                    let should_close = handle_req(&mut stream, &dir, allow_write);
                        
                    if should_close {
                        // println!("[Connection] Closing stream as requested");
                        break;
                    }
                }       
});

            }
            Err(e) => {
                eprintln!("error: {}", e);
            }
        }
    }

}
