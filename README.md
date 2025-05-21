# HTTP Server

This is a simple HTTP server written in Rust 🦀

--- 
## Usage
1. Download the binary from github releases.
2. Run the following command to start the server:
```
http-server --directory ./
```
Note: Make sure that port 8080 is not in use. (TODO) You can change the port by using the `--port` flag.
3. Make your first request:
```bash
curl http://localhost:8080/echo/hello-world
```

## Building from source:
0. Make sure you have 🦀 Rust installed. You can install it from [rustup.rs](https://rustup.rs/).
1. Clone the repository:
```bash
git clone https://github.com/arjav0703/http-server.git
```
2. Change to the directory:
```bash
cd http-server
```
3. Build the project:
```bash
cargo build --release
```
4. Run the server 🚀:
```bash
./target/release/http-server --directory ./
```
Refer to the usage section for more details.
