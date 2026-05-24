use std::fmt::Display;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Options,
}

impl From<&str> for HttpMethod {
    fn from(value: &str) -> Self {
        match value.to_uppercase().as_str() {
            "GET" => Self::Get,
            "POST" => Self::Post,
            "PUT" => Self::Put,
            "DELETE" => Self::Delete,
            "PATCH" => Self::Patch,
            "OPTIONS" => Self::Options,
            _ => Self::Get,
        }
    }
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Get => write!(f, "GET"),
            Self::Post => write!(f, "POST"),
            Self::Put => write!(f, "PUT"),
            Self::Patch => write!(f, "PATCH"),
            Self::Delete => write!(f, "DELETE"),
            Self::Options => write!(f, "OPTIONS"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::HttpMethod;

    #[test]
    fn test_method_from_str() {
        let strings = ["GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS"];

        for method_str in strings {
            let method = HttpMethod::from(method_str);

            match method {
                HttpMethod::Get => assert_eq!("GET", method_str),
                HttpMethod::Post => assert_eq!("POST", method_str),
                HttpMethod::Put => assert_eq!("PUT", method_str),
                HttpMethod::Delete => assert_eq!("DELETE", method_str),
                HttpMethod::Patch => assert_eq!("PATCH", method_str),
                HttpMethod::Options => assert_eq!("OPTIONS", method_str),
            };
        }
    }

    // TODO: Implement test for method to string
}
