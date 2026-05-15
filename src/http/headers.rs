use std::fmt::Display;

#[derive(Debug)]
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

#[derive(Debug)]
pub enum ContentTypeHeader {
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
            _ => Self::PlainText,
        }
    }
}

impl Display for ContentTypeHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PlainText => write!(f, "text/plain"),
            Self::Json => write!(f, "application/json"),
            Self::Html => write!(f, "text/html"),
        }
    }
}

#[derive(Debug)]
pub struct HttpHeaders {
    connection: ConnectionHeader,
    content_length: usize,
    content_type: Option<ContentTypeHeader>,
}

impl HttpHeaders {
    pub fn build(connection: ConnectionHeader, content_type: Option<ContentTypeHeader>) -> Self {
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
            content_type: None,
        }
    }

    pub fn connection(&self) -> &ConnectionHeader {
        &self.connection
    }

    pub fn content_type(&self) -> Option<&ContentTypeHeader> {
        self.content_type.as_ref()
    }

    pub fn content_length(&self) -> usize {
        self.content_length
    }

    pub fn set_content_length(&mut self, content_length: usize) {
        self.content_length = content_length;
    }
}

impl From<&[String]> for HttpHeaders {
    fn from(value: &[String]) -> Self {
        let mut connection = ConnectionHeader::Close;
        let mut content_length = 0;
        let mut content_type = Some(ContentTypeHeader::PlainText);

        for header in value {
            let header_parts: Vec<&str> = header.split(": ").collect();
            let header_name = header_parts[0];
            let header_value = header_parts[1];

            match header_name {
                "Connection" => connection = ConnectionHeader::from(header_value),
                "Content-Type" => content_type = Some(ContentTypeHeader::from(header_value)),
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

        if let Some(content_type) = &self.content_type {
            write!(f, "\r\nContent-Type: {}", content_type)?;
        }

        write!(f, "\r\n")
    }
}
