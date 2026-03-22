use std::collections::HashMap;

pub struct Response {
    pub version: String,
    pub status_code: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Response {
    pub fn new(status_code: u16, status_text: &str) -> Self {
        Self {
            version: "HTTP/1.1".to_string(),
            status_code,
            status_text: status_text.to_string(),
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }

    pub fn set_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    pub fn set_body(&mut self, body: Vec<u8>) {
        self.set_header("Content-Length", &body.len().to_string());
        self.body = body;
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut response = Vec::new();
        
        let status_line = format!("{} {} {}\r\n", self.version, self.status_code, self.status_text);
        response.extend_from_slice(status_line.as_bytes());

        for (k, v) in &self.headers {
            let header_line = format!("{}: {}\r\n", k, v);
            response.extend_from_slice(header_line.as_bytes());
        }

        response.extend_from_slice(b"\r\n");
        response.extend_from_slice(&self.body);

        response
    }
}
