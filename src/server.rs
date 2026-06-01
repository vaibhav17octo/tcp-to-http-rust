use anyhow::anyhow;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use std::io::Write;
pub mod response;

use response::{write_headers, write_status_line, write_body, StatusCode};
use crate::request::headers::Headers;
use crate::request::{Request, request_from_reader};

pub type Handler = fn(&mut dyn Write, Request) -> Result<(), HandlerError>;

pub struct Server {
    closed: bool,
    handler: Handler
}

pub struct HandlerError {
    pub status: StatusCode,
    pub message: Vec<u8>
}

// Whenever object of Server goes out of scope the drop function will be called
impl Drop for Server {
    fn drop(&mut self) {
        self.close();
    }
}

impl Server {
    fn new(handler: Handler) -> Self {
        Self { 
            closed: false,
            handler: handler
        }
    }

    pub fn close(&mut self) {
        println!("\nFrom close: Shutting down server gracefully");
        self.closed = true;
    }

    async fn listen(&self, mut stream: TcpStream) -> Result<(), anyhow::Error> {
        let request = request_from_reader(&mut stream).await?;
        println!("{}", request);
        let mut body: Vec<u8>= vec![];

        match (self.handler)(&mut body, request) {
            Ok(()) => {
                let headers = Headers::default_response_headers(body.len());
                write_status_line(&mut stream, StatusCode::OK).await?;
                write_headers(&mut stream, headers).await?;
                write_body(&mut stream, body).await?;
            },
            Err(e) => {
                let headers = Headers::default_response_headers(e.message.len());
                write_status_line(&mut stream, e.status).await?;
                write_headers(&mut stream, headers).await?;
                write_body(&mut stream, e.message).await?;
            }
        }
        stream.shutdown().await?;

        Ok(())
    }

    async fn run_server(&self, listener: TcpListener) -> Result<(), anyhow::Error> {
        while !self.closed {
            match listener.accept().await {
                Ok((stream, _addr)) => {
                    self.listen(stream).await?;
                    println!("Processed one connection");
                }
                Err(e) => return Err(anyhow!(format!("{e}"))),
            }
        }

        Ok(())
    }
}

pub async fn serve(port: u16, handler: Handler) -> Result<Server, anyhow::Error> {
    match TcpListener::bind(format!("127.0.0.1:{port}")).await {
        Ok(listener) => {
            println!("Application starting at port {port}");
            let server = Server::new(handler);
            server.run_server(listener).await?;
            Ok(server)
        }
        Err(e) => return Err(anyhow!(format!("{e}"))),
    }
}
