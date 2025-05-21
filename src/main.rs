use http_server::handle_req;
use std::thread;
use std::net::{TcpListener};
use std::env;
use colored::Colorize;

fn main() {
    
    let (port, directory) = getargs();
    run(&port.expect("Using default port"), directory.as_deref());

}

fn getargs() -> (Option<String>, Option<String>) {
    let args: Vec<String> = env::args().collect();

    let mut port: Option<String> = None;
    let mut directory: Option<String> = None;

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

    (port, directory)
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
