use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use anyhow::anyhow;

use crate::request::headers::Headers;

pub enum StatusCode {
    OK = 200,
    BadRequest = 400,
    InternalServerError = 500
}

pub async fn write_status_line(stream: &mut TcpStream, statusCode: StatusCode) -> Result<(), anyhow::Error> {
    let start_line: Vec<u8>;
    match statusCode {
        StatusCode::OK => start_line = b"HTTP/1.1 200 OK\r\n".to_vec(),
        StatusCode::BadRequest => start_line = b"HTTP/1.1 400 Bad Request\r\n".to_vec(),
        StatusCode::InternalServerError => start_line = b"HTTP/1.1 500 Internal Server Error\r\n".to_vec()
    }

    stream.write(&start_line).await?;

    Ok(())
}

pub async fn write_headers(stream: &mut TcpStream, headers: Headers) -> Result<(), anyhow::Error> {
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

    let write_header = format!("Content-Length:{}\r\nConnection:{}\r\nContent_Type:{}\r\n\r\n", content_length, connection, content_type).into_bytes();
    stream.write(&write_header).await?;
    Ok(())
}

