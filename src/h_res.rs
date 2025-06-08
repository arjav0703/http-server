use flate2::write::GzEncoder;
use flate2::Compression;
use std::collections::HashMap;
use std::io::Write;

pub struct HttpResponse {
    status: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl HttpResponse {
    pub fn new(status: &str) -> Self {
        HttpResponse {
            status: status.to_string(),
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }

    pub fn add_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    pub fn set_body(&mut self, body: &[u8]) {
        self.body = body.to_vec();
        self.headers
            .insert("Content-Length".to_string(), self.body.len().to_string());
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut body = self.body.clone();
        let mut headers = self.headers.clone();

        if let Some(encoding) = headers.get("Content-Encoding") {
            if encoding == "gzip" {
                let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
                encoder.write_all(&body).unwrap();
                body = encoder.finish().unwrap();
                headers.insert("Content-Length".to_string(), body.len().to_string());
            }
        }
        let mut response = format!("HTTP/1.1 {}\r\n", self.status);

        for (key, value) in &headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }

        response.push_str("\r\n");

        let mut bytes = response.into_bytes();
        bytes.extend(&body);
        bytes
    }
}
