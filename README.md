# HTTP Server

This is a fast and lightweight (less than 1 MB) HTTP server written in Rust 🦀

--- 
## Usage
1. Download the binary from [github releases](https://github.com/arjav0703/http-server/releases/).
2. Run the following command to start the server:
```
http-server
```
Note: Make sure that port 8080 is not in use. You can change the port by using the `--port` flag.
3. Make your first request:
```bash
curl http://localhost:8080/echo/hello-world
```
### Options:
- `--port <port>`: Specify the port to run the server on. Default is 8080.
- `--directory <directory>`: Specify the directory to serve files from. Default is the current directory.
-- `--allow-write`: Allow writing to the specified directory. This will allow you to create and update files in the specified directory through sending a POST request to /files/<foobar> and along with content.
---

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

---

## Features
0. Port, directory, and write access configuration through command line arguments.
1. File serving:
    You can access any file in the specified directory using the URL `http://localhost:8080/files/<filename>`.

2. File Creation:
    You can create a file by sending a POST request with the file content to the URL `http://localhost:8080/files/<filename>`. The server will create a file with the specified name in the specified directory.

3. File Updation:
    You can update a file by sending a POST request with the new content to the URL `http://localhost:8080/files/<filename>`. The server will update the file with the specified name in the specified directory.

4. Route echo:
    You can echo any string using the URL `http://localhost:8080/echo/<string>`. The server will respond with the same string.

5. User agent:
    The server will respond with the user agent string on sending a request to `http://localhost:8080/user-agent`.

6. File Protection: 
    The server will not allow access to files beginning with a '.' or a '_'

