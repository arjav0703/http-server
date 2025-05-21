use http_server::handle_req;
use std::thread;
use std::net::{TcpListener};
use std::env;
use colored::Colorize;

fn main() {
    
    let port = "8080";
    println!("🚀 Server started on port {}", port.yellow().bold());

    run(port);

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
            Ok(mut stream) => {
                let msg = "Accepted new connection";
                println!("{}", msg.green());

                let dir = directory.map(|s| s.to_string());
                thread::spawn(move || {
                loop {
                    let should_close = handle_req(&mut stream, &dir);
                    if should_close {
                    println!("[Connection] Closing stream as requested");
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
