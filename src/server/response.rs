use tokio::io::AsyncWrite;
use tokio::io::AsyncWriteExt;

use crate::request::headers::Headers;

pub enum StatusCode {
    OK = 200,
    BadRequest = 400,
    InternalServerError = 500,
}

pub struct Response {
    pub status: StatusCode,
    pub headers: Headers,
    pub body: Vec<u8>,
    pub trailers: Option<Headers>
}

impl Response {
    pub fn new(status: StatusCode, headers: Headers, body: Vec<u8>, trailers: Option<Headers>) -> Self {
        Response {
            status: status,
            headers: headers,
            body: body,
            trailers: trailers
        }
    }
}

// If we need this to not use generic but create a Trait object then we can use the following instead
// Pin<Box<dyn AsyncWrite + Send>>
pub struct Writer<W>
where
    W: AsyncWrite + Unpin,
{
    writer: W,
}

impl<W> Writer<W>
where
    W: AsyncWrite + Unpin,
{
    pub fn new(writer: W) -> Self {
        Writer { writer: writer }
    }

    pub async fn shutdown(&mut self) -> Result<(), anyhow::Error> {
        self.writer.shutdown().await?;
        Ok(())
    }

    pub async fn write_status_line(
        &mut self,
        status_code: StatusCode,
    ) -> Result<(), anyhow::Error> {
        let start_line = match status_code {
            StatusCode::OK => b"HTTP/1.1 200 OK\r\n".as_slice(),
            StatusCode::BadRequest => b"HTTP/1.1 400 Bad Request\r\n".as_slice(),
            StatusCode::InternalServerError => b"HTTP/1.1 500 Internal Server Error\r\n".as_slice(),
        };

        self.writer.write_all(&start_line).await?;

        Ok(())
    }

    pub async fn write_headers(&mut self, headers: &Headers) -> Result<(), anyhow::Error> {
        // TODO: Errors are not being shown in logs and hence couldn't figure out where we were failing
        // TODO: Think of the tests that we removed how can take care of them
        self.writer.write_all(&headers.write_headers()).await?;
        Ok(())
    }

    pub async fn write_body(&mut self, body: Vec<u8>) -> Result<(), anyhow::Error> {
        self.writer.write_all(&body).await?;
        Ok(())
    }
}
