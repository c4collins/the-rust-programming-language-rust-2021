use std::fmt::{Display, Formatter, Result};

enum HttpMethod {
    GET,
    POST,
    PUT,
}

pub struct HttpStatus {
    code: u16,
    name: String,
}

impl HttpStatus {
    pub fn new(code: u16, name: &str) -> Self {
        Self {
            code,
            name: name.to_string(),
        }
    }
}

pub struct HttpHeader {
    name: String,
    value: String,
}

pub struct HttpRequest {
    status: HttpStatus,
    version: String,
    path: String,
    headers: Vec<HttpHeader>,
    method: HttpMethod,
}

pub struct HttpResponse {
    status: HttpStatus,
    length: usize,
    pub contents: String,
}

impl HttpResponse {
    pub fn new(status: HttpStatus, contents: String) -> Self {
        let length = contents.len();
        Self {
            status,
            contents,
            length,
        }
    }

    pub fn as_string(&self) -> String {
        let response = [
            format!("HTTP/1.1 {} {}", self.status.code, self.status.name),
            format!("Content-Length: {}", self.length),
            format!("Content-Type: text/html"),
            format!(""),
            self.contents.clone(),
        ];
        let string_response = response.join("\r\n");
        string_response
    }
}
