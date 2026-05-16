use std::fmt::Display;

pub enum ParseHttpStatusError {
    UnkownStatusCode,
}

#[derive(Debug)]
pub struct HttpStatus {
    code: u16,
    phrase: &'static str,
}

impl HttpStatus {
    fn build(code: u16, phrase: &'static str) -> Self {
        HttpStatus {
            code,
            phrase: phrase,
        }
    }

    // 1xx: Informational
    pub fn continue_status() -> Self {
        Self::build(100, "Continue")
    }

    pub fn switching_protocols() -> Self {
        Self::build(101, "Switching Protocols")
    }

    pub fn processing() -> Self {
        Self::build(102, "Processing")
    }

    pub fn early_hints() -> Self {
        Self::build(103, "Early Hints")
    }

    // 2xx: Success
    pub fn ok() -> Self {
        Self::build(200, "OK")
    }

    pub fn created() -> Self {
        Self::build(201, "Created")
    }

    pub fn accepted() -> Self {
        Self::build(202, "Accepted")
    }

    pub fn non_authoritative_information() -> Self {
        Self::build(203, "Non-Authoritative Information")
    }

    pub fn no_content() -> Self {
        Self::build(204, "No Content")
    }

    pub fn reset_content() -> Self {
        Self::build(205, "Reset Content")
    }

    pub fn partial_content() -> Self {
        Self::build(206, "Partial Content")
    }

    pub fn multi_status() -> Self {
        Self::build(207, "Multi-Status")
    }

    pub fn already_reported() -> Self {
        Self::build(208, "Already Reported")
    }

    pub fn im_used() -> Self {
        Self::build(226, "IM Used")
    }

    // 3xx: Redirection
    pub fn multiple_choices() -> Self {
        Self::build(300, "Multiple Choices")
    }

    pub fn moved_permanently() -> Self {
        Self::build(301, "Moved Permanently")
    }

    pub fn found() -> Self {
        Self::build(302, "Found")
    }

    pub fn see_other() -> Self {
        Self::build(303, "See Other")
    }

    pub fn not_modified() -> Self {
        Self::build(304, "Not Modified")
    }

    pub fn use_proxy() -> Self {
        Self::build(305, "Use Proxy")
    }

    pub fn temporary_redirect() -> Self {
        Self::build(307, "Temporary Redirect")
    }

    pub fn permanent_redirect() -> Self {
        Self::build(308, "Permanent Redirect")
    }

    // 4xx: Client Errors
    pub fn bad_request() -> Self {
        Self::build(400, "Bad Request")
    }

    pub fn unauthorized() -> Self {
        Self::build(401, "Unauthorized")
    }

    pub fn payment_required() -> Self {
        Self::build(402, "Payment Required")
    }

    pub fn forbidden() -> Self {
        Self::build(403, "Forbidden")
    }

    pub fn not_found() -> Self {
        Self::build(404, "Not Found")
    }

    pub fn method_not_allowed() -> Self {
        Self::build(405, "Method Not Allowed")
    }

    pub fn not_acceptable() -> Self {
        Self::build(406, "Not Acceptable")
    }

    pub fn proxy_authentication_required() -> Self {
        Self::build(407, "Proxy Authentication Required")
    }

    pub fn request_timeout() -> Self {
        Self::build(408, "Request Timeout")
    }

    pub fn conflict() -> Self {
        Self::build(409, "Conflict")
    }

    pub fn gone() -> Self {
        Self::build(410, "Gone")
    }

    pub fn length_required() -> Self {
        Self::build(411, "Length Required")
    }

    pub fn precondition_failed() -> Self {
        Self::build(412, "Precondition Failed")
    }

    pub fn payload_too_large() -> Self {
        Self::build(413, "Payload Too Large")
    }

    pub fn uri_too_long() -> Self {
        Self::build(414, "URI Too Long")
    }

    pub fn unsupported_media_type() -> Self {
        Self::build(415, "Unsupported Media Type")
    }

    pub fn range_not_satisfiable() -> Self {
        Self::build(416, "Range Not Satisfiable")
    }

    pub fn expectation_failed() -> Self {
        Self::build(417, "Expectation Failed")
    }

    pub fn teapot() -> Self {
        Self::build(418, "I'm a teapot")
    }

    pub fn misdirected_request() -> Self {
        Self::build(421, "Misdirected Request")
    }

    pub fn unprocessable_entity() -> Self {
        Self::build(422, "Unprocessable Entity")
    }

    pub fn locked() -> Self {
        Self::build(423, "Locked")
    }

    pub fn failed_dependency() -> Self {
        Self::build(424, "Failed Dependency")
    }

    pub fn too_early() -> Self {
        Self::build(425, "Too Early")
    }

    pub fn upgrade_required() -> Self {
        Self::build(426, "Upgrade Required")
    }

    pub fn precondition_required() -> Self {
        Self::build(428, "Precondition Required")
    }

    pub fn too_many_requests() -> Self {
        Self::build(429, "Too Many Requests")
    }

    pub fn request_header_fields_too_large() -> Self {
        Self::build(431, "Request Header Fields Too Large")
    }

    pub fn unavailable_for_legal_reasons() -> Self {
        Self::build(451, "Unavailable For Legal Reasons")
    }

    // 5xx: Server Errors
    pub fn internal_server_error() -> Self {
        Self::build(500, "Internal Server Error")
    }

    pub fn not_implemented() -> Self {
        Self::build(501, "Not Implemented")
    }

    pub fn bad_gateway() -> Self {
        Self::build(502, "Bad Gateway")
    }

    pub fn service_unavailable() -> Self {
        Self::build(503, "Service Unavailable")
    }

    pub fn gateway_timeout() -> Self {
        Self::build(504, "Gateway Timeout")
    }

    pub fn http_version_not_supported() -> Self {
        Self::build(505, "HTTP Version Not Supported")
    }

    pub fn variant_also_negotiates() -> Self {
        Self::build(506, "Variant Also Negotiates")
    }

    pub fn insufficient_storage() -> Self {
        Self::build(507, "Insufficient Storage")
    }

    pub fn loop_detected() -> Self {
        Self::build(508, "Loop Detected")
    }

    pub fn not_extended() -> Self {
        Self::build(510, "Not Extended")
    }

    pub fn network_authentication_required() -> Self {
        Self::build(511, "Network Authentication Required")
    }

    pub fn code(&self) -> u16 {
        self.code
    }

    pub fn phrase(&self) -> &'static str {
        self.phrase
    }
}

impl TryFrom<u16> for HttpStatus {
    type Error = ParseHttpStatusError;

    fn try_from(code: u16) -> Result<Self, Self::Error> {
        match code {
            // 1xx: Informational
            100 => Ok(Self::continue_status()),
            101 => Ok(Self::switching_protocols()),
            102 => Ok(Self::processing()),
            103 => Ok(Self::early_hints()),

            // 2xx: Success
            200 => Ok(Self::ok()),
            201 => Ok(Self::created()),
            202 => Ok(Self::accepted()),
            203 => Ok(Self::non_authoritative_information()),
            204 => Ok(Self::no_content()),
            205 => Ok(Self::reset_content()),
            206 => Ok(Self::partial_content()),
            207 => Ok(Self::multi_status()),
            208 => Ok(Self::already_reported()),
            226 => Ok(Self::im_used()),

            // 3xx: Redirection
            300 => Ok(Self::multiple_choices()),
            301 => Ok(Self::moved_permanently()),
            302 => Ok(Self::found()),
            303 => Ok(Self::see_other()),
            304 => Ok(Self::not_modified()),
            305 => Ok(Self::use_proxy()),
            307 => Ok(Self::temporary_redirect()),
            308 => Ok(Self::permanent_redirect()),

            // 4xx: Client Errors
            400 => Ok(Self::bad_request()),
            401 => Ok(Self::unauthorized()),
            402 => Ok(Self::payment_required()),
            403 => Ok(Self::forbidden()),
            404 => Ok(Self::not_found()),
            405 => Ok(Self::method_not_allowed()),
            406 => Ok(Self::not_acceptable()),
            407 => Ok(Self::proxy_authentication_required()),
            408 => Ok(Self::request_timeout()),
            409 => Ok(Self::conflict()),
            410 => Ok(Self::gone()),
            411 => Ok(Self::length_required()),
            412 => Ok(Self::precondition_failed()),
            413 => Ok(Self::payload_too_large()),
            414 => Ok(Self::uri_too_long()),
            415 => Ok(Self::unsupported_media_type()),
            416 => Ok(Self::range_not_satisfiable()),
            417 => Ok(Self::expectation_failed()),
            418 => Ok(Self::teapot()),
            421 => Ok(Self::misdirected_request()),
            422 => Ok(Self::unprocessable_entity()),
            423 => Ok(Self::locked()),
            424 => Ok(Self::failed_dependency()),
            425 => Ok(Self::too_early()),
            426 => Ok(Self::upgrade_required()),
            428 => Ok(Self::precondition_required()),
            429 => Ok(Self::too_many_requests()),
            431 => Ok(Self::request_header_fields_too_large()),
            451 => Ok(Self::unavailable_for_legal_reasons()),

            // 5xx: Server Errors
            500 => Ok(Self::internal_server_error()),
            501 => Ok(Self::not_implemented()),
            502 => Ok(Self::bad_gateway()),
            503 => Ok(Self::service_unavailable()),
            504 => Ok(Self::gateway_timeout()),
            505 => Ok(Self::http_version_not_supported()),
            506 => Ok(Self::variant_also_negotiates()),
            507 => Ok(Self::insufficient_storage()),
            508 => Ok(Self::loop_detected()),
            510 => Ok(Self::not_extended()),
            511 => Ok(Self::network_authentication_required()),

            _ => Err(ParseHttpStatusError::UnkownStatusCode),
        }
    }
}

impl Display for HttpStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.code, self.phrase)
    }
}
