use http_server::handle_req;
use std::thread;
use std::net::{TcpListener};
use std::env;
use colored::Colorize;

fn main() {
    
    let (port, directory) = getargs();
    run(&port.expect("Using default port"), directory.as_deref());

}

fn getargs() -> (Option<String> , Option<String>) {
    let args: Vec<String> = env::args().collect();
    let mut directory = None;
    let default_dir = String::from(".");

    for i in 0..args.len() - 1 {
        if args[i] == "--directory" {
            directory = Some(&args[i + 1]);
        } else {
            directory = Some(&default_dir);
        }
    }

    let args: Vec<String> = env::args().collect();
    let mut port = None;
    let default_port = String::from("8080");

    for i in 0..args.len() - 1 {
        if args[i] == "--port" {
            port = Some(&args[i + 1]);
        } else {
            port = Some(&default_port);
        }
    }
    (port.cloned() , directory.cloned())
}

fn run(port:&String, directory: Option<&str>) {
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
