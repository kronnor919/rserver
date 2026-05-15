use std::{
    io::{BufRead, BufReader, Read},
    net::TcpStream,
};

use crate::{HttpHeaders, HttpMethod};

#[derive(Debug)]
pub struct HttpRequest {
    method: HttpMethod,
    route: String,
    headers: HttpHeaders,
    body: String,
}

impl HttpRequest {
    pub fn method(&self) -> &HttpMethod {
        &self.method
    }

    pub fn route(&self) -> &str {
        &self.route
    }

    pub fn headers(&self) -> &HttpHeaders {
        &self.headers
    }

    pub fn body(&self) -> &str {
        &self.body
    }
}

impl From<BufReader<&TcpStream>> for HttpRequest {
    fn from(mut value: BufReader<&TcpStream>) -> Self {
        let mut request_lines = Vec::new();

        loop {
            let mut line = String::new();
            value.read_line(&mut line).unwrap();

            if line.as_str() == "\r\n" {
                break;
            }

            request_lines.push(line.trim().to_string());
        }

        let request_line: Vec<&str> = request_lines[0].split(" ").collect();

        let method = HttpMethod::from(request_line[0]);
        let route = request_line[1].to_string();
        let _http_version = request_line[2];

        let headers_lines: &[String] = &request_lines[1..];

        let headers = HttpHeaders::from(headers_lines);

        let mut body = String::new();

        value
            .take(headers.content_length() as u64)
            .read_to_string(&mut body)
            .unwrap();

        HttpRequest {
            method,
            route,
            headers,
            body,
        }
    }
}
