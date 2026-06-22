use std::fmt::Display;

use crate::{
    ConnectionHeader, ContentTypeHeader,
    http::{headers::HttpHeaders, status::HttpStatus},
};

#[derive(Debug)]
pub struct HttpResponse {
    status: HttpStatus,
    headers: HttpHeaders,
    body: String,
}

impl HttpResponse {
    pub fn build(body: &str, status: HttpStatus, mut headers: HttpHeaders) -> Self {
        headers.set_content_length(body.len());

        HttpResponse {
            status,
            headers,
            body: body.to_string(),
        }
    }

    pub fn new(body: &str, status: HttpStatus) -> Self {
        let headers =
            HttpHeaders::build(ConnectionHeader::Close, body.len(), ContentTypeHeader::Json);

        HttpResponse {
            status,
            headers,
            body: body.to_string(),
        }
    }

    pub fn headers_mut(&mut self) -> &mut HttpHeaders {
        &mut self.headers
    }
}

impl Display for HttpResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HTTP/1.1 {} {}\r\n{}\r\n{}",
            self.status.code(),
            self.status.phrase(),
            self.headers,
            self.body,
        )
    }
}
