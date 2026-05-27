mod http;

use std::{
    io::{BufReader, Write},
    net::{Shutdown, TcpListener},
};

pub use http::{
    headers::{ConnectionHeader, ContentTypeHeader, HttpHeaders},
    methods::HttpMethod,
    request::HttpRequest,
    response::HttpResponse,
    router::HttpRouter,
    status::{HttpStatus, ParseHttpStatusError},
};

pub fn run(addr: &str, router: &HttpRouter) {
    let listener = TcpListener::bind(addr).expect(&format!("Error binding to {addr}"));

    for stream in listener.incoming() {
        let mut stream = stream.expect("Error accepting client");

        let buffer = BufReader::new(&stream);

        let request = HttpRequest::from(buffer);
        let route = (request.method().clone(), request.route().to_string());

        let view_function = router.get_route(route);

        let response = match view_function {
            Some(function) => function(request),
            None => HttpResponse::build("", HttpStatus::not_found(), HttpHeaders::new()),
        };

        stream
            .write_all(response.to_string().as_bytes())
            .expect("Error writing into buffer");

        stream
            .shutdown(Shutdown::Both)
            .expect("Error while shutdown stream");
    }
}
