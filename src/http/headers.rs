use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum ConnectionHeader {
    Close,
    KeepAlive,
}

impl From<&str> for ConnectionHeader {
    fn from(value: &str) -> Self {
        match value {
            "close" => Self::Close,
            "keep-alive" => Self::KeepAlive,
            _ => Self::Close,
        }
    }
}

impl Display for ConnectionHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Close => write!(f, "close"),
            Self::KeepAlive => write!(f, "keep-alive"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ContentTypeHeader {
    None,
    Json,
    Html,
    PlainText,
}

impl From<&str> for ContentTypeHeader {
    fn from(value: &str) -> Self {
        match value {
            "application/json" => Self::Json,
            "text/html" => Self::Html,
            "text/plain" => Self::PlainText,
            _ => Self::None,
        }
    }
}

impl Display for ContentTypeHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PlainText => write!(f, "text/plain"),
            Self::Json => write!(f, "application/json"),
            Self::Html => write!(f, "text/html"),
            Self::None => write!(f, ""),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct HttpHeaders {
    connection: ConnectionHeader,
    content_length: usize,
    content_type: ContentTypeHeader,
}

impl HttpHeaders {
    pub fn build(connection: ConnectionHeader, content_type: ContentTypeHeader) -> Self {
        HttpHeaders {
            connection,
            content_length: 0,
            content_type,
        }
    }

    pub fn new() -> Self {
        HttpHeaders {
            connection: ConnectionHeader::Close,
            content_length: 0,
            content_type: ContentTypeHeader::None,
        }
    }

    pub fn connection(&self) -> &ConnectionHeader {
        &self.connection
    }

    pub fn content_type(&self) -> &ContentTypeHeader {
        &self.content_type
    }

    pub fn content_length(&self) -> usize {
        self.content_length
    }

    pub fn set_content_length(&mut self, content_length: usize) {
        self.content_length = content_length;
    }

    pub fn set_content_type(&mut self, content_type: ContentTypeHeader) {
        self.content_type = content_type;
    }
}

impl From<&[String]> for HttpHeaders {
    fn from(value: &[String]) -> Self {
        let mut connection = ConnectionHeader::Close;
        let mut content_length = 0;
        let mut content_type = ContentTypeHeader::None;

        for header in value {
            let header_parts: Vec<&str> = header.split(": ").collect();
            let header_name = header_parts[0];
            let header_value = header_parts[1];

            match header_name {
                "Connection" => connection = ConnectionHeader::from(header_value),
                "Content-Type" => content_type = ContentTypeHeader::from(header_value),
                "Content-Length" => content_length = header_value.parse().unwrap(),
                _ => (),
            }
        }

        HttpHeaders {
            connection,
            content_length,
            content_type,
        }
    }
}

impl Display for HttpHeaders {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Connection: {}\r\nContent-Length: {}",
            self.connection, self.content_length,
        )?;

        if self.content_type != ContentTypeHeader::None {
            write!(f, "\r\nContent-Type: {}", self.content_type)?;
        }

        write!(f, "\r\n")
    }
}

#[cfg(test)]
mod tests {
    use crate::{ConnectionHeader, ContentTypeHeader, HttpHeaders};

    #[test]
    fn test_headers_parsing() {
        let raw_headers: &str = "Content-Type: application/json\r\n\
                     Content-Length: 1024\r\n\
                     Connection: keep-alive\r\n";

        let header_lines: Vec<String> = raw_headers[..].lines().map(|l| l.to_string()).collect();
        let headers = HttpHeaders::from(&header_lines[..]);

        let mut expected = HttpHeaders::build(ConnectionHeader::KeepAlive, ContentTypeHeader::Json);
        expected.set_content_length(1024);

        assert_eq!(headers, expected);
    }

    #[test]
    fn test_headers_dumping() {
        let mut headers = HttpHeaders::build(ConnectionHeader::Close, ContentTypeHeader::Json);
        headers.set_content_length(1024);

        let expected: &str = "Connection: close\r\n\
                     Content-Length: 1024\r\n\
                     Content-Type: application/json\r\n";

        let headers_dumped = headers.to_string();

        assert_eq!(expected, headers_dumped);
    }
}
