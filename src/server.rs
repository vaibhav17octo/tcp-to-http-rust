use anyhow::anyhow;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

pub mod response;

use response::{write_headers, write_status_line, StatusCode};
use crate::request::headers::Headers;

pub struct Server {
    closed: bool,
}

// Whenever object of Server goes out of scope the drop function will be called
impl Drop for Server {
    fn drop(&mut self) {
        self.close();
    }
}

impl Server {
    fn new() -> Self {
        Server { closed: false }
    }

    pub fn close(&mut self) {
        println!("\n From close: Shutting down server gracefully");
        self.closed = true;
    }

    async fn listen(&self, mut stream: TcpStream) -> Result<(), anyhow::Error> {
        write_status_line(&mut stream, StatusCode::OK).await?;
        write_headers(&mut stream, Headers::default_response_headers(0)).await?;
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

pub async fn serve(port: u16) -> Result<Server, anyhow::Error> {
    match TcpListener::bind(format!("127.0.0.1:{port}")).await {
        Ok(listener) => {
            println!("Application starting at port {port}");
            let server = Server::new();
            server.run_server(listener).await?;
            Ok(server)
        }
        Err(e) => return Err(anyhow!(format!("{e}"))),
    }
}
