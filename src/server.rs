use anyhow::anyhow;
use tokio::io::AsyncWrite;
use tokio::net::{TcpListener, TcpStream};
pub mod response;

use crate::request::headers::Headers;
use crate::request::{Request, request_from_reader};
use response::{Response, StatusCode, Writer};

use std::pin::Pin;

// Check Async Handlers section in README
pub type HandlerFuture = Pin<Box<dyn Future<Output = Result<Response, HandlerError>> + Send>>;

pub type Handler = fn(Request) -> HandlerFuture;

pub struct Server {
    closed: bool,
    handler: Handler,
}

pub struct HandlerError {
    pub status: StatusCode,
    pub message: Vec<u8>,
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
            handler: handler,
        }
    }

    pub fn close(&mut self) {
        println!("\nFrom close: Shutting down server gracefully");
        self.closed = true;
    }

    async fn chunked_encoding<W>(writer: &mut Writer<W>, body: Vec<u8>) -> Result<(), anyhow::Error>
    where
        W: AsyncWrite + Unpin,
    {
        let chunks = body.chunks(32);
        println!("Doing chunk encoding");
        for chunk in chunks {
            println!("Chunk:{:?}", chunk);
            writer
                .write_body(format!("{:X}\r\n", chunk.len()).into_bytes())
                .await?;
            writer.write_body(chunk.to_vec()).await?;
            writer.write_body(b"\r\n".to_vec()).await?;
        }
        writer.write_body(b"0\r\n\r\n".to_vec()).await?;
        Ok(())
    }

    async fn listen(handler: Handler, mut stream: TcpStream) -> Result<(), anyhow::Error> {
        let request = request_from_reader(&mut stream).await?;
        // println!("{}", request);

        let mut writer = Writer::new(stream);

        match (handler)(request).await {
            Ok(response) => {
                writer.write_status_line(response.status).await?;
                writer.write_headers(&response.headers).await?;

                match response.headers.get(&String::from("transfer-encoding")) {
                    Some(v) => {
                        if v == "chunked" {
                            Self::chunked_encoding(&mut writer, response.body).await?;
                        } else {
                            return Err(anyhow!("Chunked is the only supported transfer-encoding"));
                        }
                    }
                    None => {
                        println!("Body not chunked:{:?}", response.body);
                        writer.write_body(response.body).await?;
                    }
                }
            }
            Err(e) => {
                let headers = Headers::default_response_headers(e.message.len());
                writer.write_status_line(e.status).await?;
                writer.write_headers(&headers).await?;
                writer.write_body(e.message).await?;
            }
        }

        writer.shutdown().await?;

        Ok(())
    }

    async fn run_server(&self, listener: TcpListener) -> Result<(), anyhow::Error> {
        let handler = self.handler;
        while !self.closed {
            match listener.accept().await {
                Ok((stream, _addr)) => {
                    tokio::spawn(async move {
                        match Self::listen(handler, stream).await {
                            Ok(_) => Ok(()),
                            Err(e) => return Err(anyhow!(format!("{e}"))),
                        }
                    });
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
