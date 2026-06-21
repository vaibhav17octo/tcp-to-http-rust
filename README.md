# TCP-To-Http in Rust

## Description
Based on [ThePrimeagen's TCP to HTTP](https://www.boot.dev/lessons/b0cebf37-7151-48db-ad8a-0f9399f94c58) course in Go

I made this repository in order to learn Rust and understand the workings of HTTP protocol from the Layer 4 TCP protocol perspective.

## Usage
After cloning the repository, you can use the following command to run the HTTP Server which parses HTTP requests

The command runs a HTTP Server on your localhost on port 42069.
```bash
cargo run --bin httpserver
```

Open another terminal and try to make an HTTP request using Curl the HTTP Server:
```bash
curl http://localhost:42069/httpbin/stream/10
``` 

You should see 10 Json objects returned to you.

## How it works

### Server
We have a [Server](./src/server/mod.rs) which accepts a PORT and a handler function to handle your connection. (Check [httpserver](./src/bin/httpserver/main.rs) for example use case).
You can then send HTTP requests to the mentioned port.

- The server does support chunked encoding. However, will require `"transfer-encoding": "chunked"` header. (Example can found in [my_handler](./src/bin/httpserver/main.rs))
- The handler can handle async functions like querying a database, a website etc.

#### Example handler
```
fn my_handler(request: Request) -> HandlerFuture {
    Box::pin(async move {
        let mut status = StatusCode::OK;
        let mut body = b"Hello hello".to_vec();
        let mut headers = Headers::default_response_headers(body.len());
        let mut trailers: Option<Headers> = None;

        Ok(Response::new(status, headers, body, trailers))
    })
}
```

### TCPListener
We have the [tcplistener](./src/bin/tcplistener/main.rs) which creates a Tcp listener and passes the TCP stream to a function `request_from_reader` defined in [request module](./src/request.rs) which returns a Request sruct.

`request_from_reader` function reads from the TCP stream at most 1024 bytes at a time and parses the request with the parse function defined in Request struct.

## Async Handlers

Initially, request handlers were synchronous functions returning a Response directly. This worked for simple request processing, but became limiting when a handler needed to perform asynchronous operations such as making HTTP requests with reqwest, querying a database, or reading files asynchronously.

To support asynchronous handlers, the server stores handlers as functions that return a Future. Because every async fn generates a unique anonymous future type, the concrete future type is erased behind a dyn Future trait object. The future is heap allocated using Box, pinned using Pin so it can be safely polled by the async runtime, and marked Send so Tokio can move it between worker threads if necessary.

This allows handlers to perform non-blocking I/O while integrating cleanly with the Tokio runtime and the server's concurrent request processing model.

## References
- [Message format RFC 9112](https://datatracker.ietf.org/doc/html/rfc9112#name-message-format)
- [Parsing the request line RFC 9110](https://datatracker.ietf.org/doc/html/rfc9112#name-message-parsing)
- [Headers/field-line parsing RFC 9112](https://datatracker.ietf.org/doc/html/rfc9112#name-field-syntax)