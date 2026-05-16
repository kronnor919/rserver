# RServer

RServer is a single-threaded HTTP server implemented using only Rust's standard library, with zero external dependencies. It is capable of parsing HTTP requests and constructing HTTP responses. Additionally, it provides a crate-like interface that allows seamless integration into existing projects and the definition of custom endpoints.

## Installation

There is not yet an installation method as a crate, but you can clone the code and test the library by running the following commands:

```bash
git clone https://github.com/kronnor919/rserver.git
cd rserver
```

## Basic Usage

### Simple `GET` endpoint

```rust
use rserver::{ContentTypeHeader, HttpHeaders, HttpResponse, HttpRouter, HttpStatus, run};

fn main() {
    let mut router = HttpRouter::new();

    // GET /hello
    router.get(
        "/hello",
        Box::new(|| {
            let mut headers = HttpHeaders::new();
            headers.set_content_type(Some(ContentTypeHeader::PlainText));

            HttpResponse::build("Hello world!", HttpStatus::ok(), headers)
        }),
    );
```

This endpoint returns a response to the client with `Content-Type: text/plain` and a `200 OK` status code. Additionally, `HttpResponse::build` automatically computes the body length and sets the corresponding `Content-Length` header.

### `POST` route

```rust
    // POST /publish
    router.post(
        "/publish",
        Box::new(|| {
            let mut headers = HttpHeaders::new();
            headers.set_content_type(Some(ContentTypeHeader::Json));

            HttpResponse::build(
                "{\"message\": \"published\"}",
                HttpStatus::created(),
                headers,
            )
        }),
    );
```

All standard HTTP methods are supported. As illustrated in this example, no helper is currently provided for serializing JSONs from Rust structs... yet.

### Run server

```rust
    run("localhost:7364", &router);
}
```

Finally, the server can be executed and begin processing requests by invoking `rserver::run` with the configured `HttpRouter` as its argument. In this example, the server is bound to `localhost:7364`.

### Route not found

If RServer encounters an endpoint not registered in the router, it will return a response as follows:

```rust
HttpResponse::build(
    "",
    HttpStatus::not_found(),
    HttpHeaders::build(ConnectionHeader::Close, None),
)
```

## License

Project under [MIT License](./LICENSE)
