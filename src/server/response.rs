use tokio::io::AsyncWrite;
use tokio::io::AsyncWriteExt;
use anyhow::anyhow;

use crate::request::headers::Headers;

pub enum StatusCode {
    OK = 200,
    BadRequest = 400,
    InternalServerError = 500
}

pub struct Response {
    pub status: StatusCode,
    pub headers: Headers,
    pub body: Vec<u8>
}

impl Response {
    pub fn new(status: StatusCode, headers: Headers, body: Vec<u8>) -> Self {
        Response { 
            status: status, 
            headers: headers, 
            body: body 
        }
    }
}

// If we need this to not use generic but create a Trait object then we can use the following instead
// Pin<Box<dyn AsyncWrite + Send>>
pub struct Writer<W>
where
    W: AsyncWrite + Unpin, 
{
    writer: W
}

impl<W> Writer<W>
where
    W: AsyncWrite + Unpin,
{
    pub fn new(writer: W) -> Self {
        Writer { 
            writer: writer
        }
    }

    pub async fn shutdown(&mut self) -> Result<(), anyhow::Error> {
        self.writer.shutdown().await?;
        Ok(())
    }

    pub async fn write_status_line(&mut self, status_code: StatusCode) -> Result<(), anyhow::Error> {
        let start_line = match status_code {
            StatusCode::OK => b"HTTP/1.1 200 OK\r\n".as_slice(),
            StatusCode::BadRequest => b"HTTP/1.1 400 Bad Request\r\n".as_slice(),
            StatusCode::InternalServerError => b"HTTP/1.1 500 Internal Server Error\r\n".as_slice()
        };

        self.writer.write_all(&start_line).await?;

        Ok(())
    }

    pub async fn write_headers(&mut self, headers: Headers) -> Result<(), anyhow::Error> {
        let content_length;
        match headers.get(&"content-length".to_string()) {
            Some(c) => content_length = c,
            None => return Err(anyhow!(format!("No content length in headers")))
        }

        let connection;
        match headers.get(&"Connection".to_string()) {
            Some(c) => connection = c,
            None => return Err(anyhow!(format!("No Connection in headers")))
        }

        let content_type;
        match headers.get(&"Content-Type".to_string()) {
            Some(c) => content_type = c,
            None => return Err(anyhow!(format!("No content length in headers")))
        }

        let write_header = format!("Content-Length:{}\r\nConnection:{}\r\nContent_Type:{}\r\n\r\n", content_length, connection, content_type);
        self.writer.write_all(&write_header.as_bytes()).await?;
        Ok(())
    }

    pub async fn write_body(&mut self, body: Vec<u8>) -> Result<(), anyhow::Error> {
        self.writer.write_all(&body).await?;
        Ok(())
    }
}

