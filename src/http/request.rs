use std::{
    collections::HashMap,
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
    args: HashMap<String, String>,
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

    pub fn args(&self) -> &HashMap<String, String> {
        &self.args
    }
}

impl From<BufReader<&TcpStream>> for HttpRequest {
    fn from(mut value: BufReader<&TcpStream>) -> Self {
        // Stores all the lines of the request until
        // the body
        let mut request_lines = Vec::new();

        loop {
            let mut line = String::new();
            value.read_line(&mut line).unwrap();

            if line.as_str() == "\r\n" {
                break;
            }

            request_lines.push(line.trim().to_string());
        }

        // <METHOD> <ROUTE?ARGS> <HTTP_METHOD>
        let request_line: Vec<&str> = request_lines[0].split(" ").collect();

        let method = HttpMethod::from(request_line[0]);
        let route_with_args = request_line[1];
        let _http_version = request_line[2];

        // (route, arg1=val1&arg2=val2)
        let route_parts: Vec<&str> = route_with_args.split("?").collect();

        // ROUTE
        let route = route_parts[0].to_string();

        let mut args = HashMap::new();

        // arg1=val1&arg2=val2
        let args_str_option: Option<&&str> = route_parts.get(1);

        // If args is not empty
        if let Some(args_str) = args_str_option {
            // ...arg1=val1...
            for pair in args_str.split("&").collect::<Vec<&str>>() {
                let parts: Vec<&str> = pair.split("=").collect();

                // (arg1, val1)
                let (key, value) = (parts[0], parts[1]);

                args.insert(key.to_string(), value.to_string());
            }
        }

        // Headers
        let headers_lines: &[String] = &request_lines[1..];

        let headers = HttpHeaders::from(headers_lines);

        let mut body = String::new();

        // Extract the body with headers.content_length
        value
            .take(headers.content_length() as u64)
            .read_to_string(&mut body)
            .unwrap();

        HttpRequest {
            method,
            route,
            headers,
            body,
            args,
        }
    }
}
