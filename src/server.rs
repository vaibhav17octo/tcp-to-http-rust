use anyhow::anyhow;
use tokio::net::{TcpListener, TcpStream};
pub mod response;

use response::{StatusCode, Writer, Response};
use crate::request::headers::Headers;
use crate::request::{Request, request_from_reader};

pub type Handler = fn(Request) -> Result<Response, HandlerError>;

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

        let mut writer = Writer::new(stream);

        match (self.handler)(request) {
            Ok(response) => {
                writer.write_status_line(response.status).await?;
                writer.write_headers(response.headers).await?;
                writer.write_body(response.body).await?;
            },
            Err(e) => {
                let headers = Headers::default_response_headers(e.message.len());
                writer.write_status_line(e.status).await?;
                writer.write_headers(headers).await?;
                writer.write_body(e.message).await?;
            }
        }

        writer.shutdown().await?;

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
