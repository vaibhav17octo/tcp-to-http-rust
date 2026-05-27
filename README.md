# TCP-To-Http in Rust

## Description
Based on [ThePrimeagen's TCP to HTTP](https://www.boot.dev/lessons/b0cebf37-7151-48db-ad8a-0f9399f94c58) course in Go

I made this repository in order to learn Rust and understand the workings of HTTP protocol from the Layer 4 TCP protocol perspective.

## Usage
After cloning the repository, you can use the following command to run the TCP listener which parses a HTTP request

The command runs a Tcp listener on your localhost on port 42069.
```bash
cargo run --bin tcplistener
```

Open another terminal and try to make an HTTP request using Curl on the TCP listener:
```bash
curl http://localhost:42069/use-neovim-btw
``` 

You should see an empty response but in the tcplistener terminal you should see the parsed request.

## How it works
We have the [tcplistener](./src/bin/tcplistener/main.rs) which creates a Tcp listener and passes the TCP stream to a function `request_from_reader` defined in [request module](./src/request.rs) which returns a Request sruct.

`request_from_reader` function reads from the TCP stream at most 1024 bytes at a time and parses the request with the parse function defined in Request struct.

## References
- [Message format RFC 9112](https://datatracker.ietf.org/doc/html/rfc9112#name-message-format)
- [Parsing the request line RFC 9110](https://datatracker.ietf.org/doc/html/rfc9112#name-message-parsing)
- [Headers/field-line parsing RFC 9112](https://datatracker.ietf.org/doc/html/rfc9112#name-field-syntax)