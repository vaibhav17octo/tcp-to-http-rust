pub mod body;
pub mod request_line;

use anyhow::anyhow;
use core::fmt;
use tokio::io::{AsyncRead, AsyncReadExt};

use crate::server::Headers;
use body::Body;
use request_line::RequestLine;

use crate::config::ERROR_STATE;
use crate::config::MALFORMED_REQUEST;

#[derive(PartialEq)]
pub enum ParserState {
    StateInit,
    StateHeaders,
    StateDone,
    StateBody,
    StateError,
}

pub struct Request {
    pub request_line: RequestLine,
    pub headers: Headers,
    pub body: Body,
    pub state: ParserState,
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\n{}\n{}",
            &self.request_line, &self.headers, &self.body
        )
    }
}

impl Request {
    fn new() -> Self {
        Self {
            request_line: RequestLine::default(),
            headers: Headers::new(),
            body: Body::new(),
            state: ParserState::StateInit,
        }
    }

    fn done(&self) -> bool {
        return self.state == ParserState::StateDone || self.state == ParserState::StateError;
    }

    fn parse(&mut self, data: &[u8]) -> Result<usize, anyhow::Error> {
        let mut read = 0;
        loop {
            let current_data = &data[read..];
            match self.state {
                ParserState::StateInit => {
                    match RequestLine::parse_request_line(current_data) {
                        Ok(r) => {
                            if r.1 == 0 {
                                break; // If we don't have a request line that means we need more data
                            }

                            read += r.1; // No of bytes we have processed

                            self.request_line = r.0;
                            self.state = ParserState::StateHeaders;
                        }
                        Err(e) => {
                            self.state = ParserState::StateError;
                            return Err(anyhow!(format!("{}: {}", ERROR_STATE, e)));
                        }
                    }
                }
                ParserState::StateHeaders => {
                    match self.headers.parse(current_data) {
                        Ok(r) => {
                            if r.0 == 0 {
                                break;
                            }

                            read += r.0; // No of bytes we have processed
                            if r.1 {
                                self.state = ParserState::StateBody;
                            }
                        }
                        Err(e) => {
                            self.state = ParserState::StateError;
                            return Err(anyhow!(format!("{}: {}", ERROR_STATE, e)));
                        }
                    }
                }
                ParserState::StateBody => {
                    match self.headers.get(&"content-length".to_string()) {
                        Some(n) => self.body.set_content_length(
                            n.parse().expect("Invalid value for content length"),
                        ),
                        None => {
                            break // There's no body to be read
                            self.state = ParserState::StateDone;
                        }
                    }

                    match self.body.parse_body(current_data) {
                        Ok(n) => {
                            read += n.0;
                            if n.1 {
                                self.state = ParserState::StateDone;
                            }
                        }
                        Err(e) => {
                            self.state = ParserState::StateError;
                            return Err(e);
                        }
                    }

                    if current_data == b"" {
                        break;
                    }
                }
                ParserState::StateDone => break,
                ParserState::StateError => break,
            }
        }
        Ok(read)
    }
}

pub async fn request_from_reader(
    f: &mut (impl AsyncRead + Unpin),
) -> Result<Request, anyhow::Error> {
    let mut req = Request::new();
    let mut buffer = [0; 1024];
    let mut buffer_length = 0;

    while !req.done() {
        let n = f.read(&mut buffer[buffer_length..]).await?; // n is the number of bytes read from f i.e. our connection/file etc

        if n == 0 && !req.done() {
            return Err(anyhow!("{}", MALFORMED_REQUEST));
        }

        buffer_length += n;

        // println!("Data to parse in request:{}", str::from_utf8(&buffer[..buffer_length])?);
        let read_n = req.parse(&buffer[..buffer_length])?; // read_n is how many bytes we have parsed

        buffer.copy_within(read_n..buffer_length, 0); // We don't need the bytes we have processed and hence we copy the remaining bytes to the starting of the buffer and reduce buffer length
        buffer_length -= read_n;
    }

    Ok(req)
}
