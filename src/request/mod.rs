use anyhow::anyhow;
use std::io::Read;
use std::str;
use crate::headers::Headers;

#[derive(Default)]
pub struct RequestLine {
    pub http_version: String,
    pub request_target: String,
    pub method: String,
}

#[derive(PartialEq)]
pub enum ParserState {
    StateInit,
    StateHeaders,
    StateDone,
    StateError,
}

pub struct Request {
    pub request_line: RequestLine,
    pub headers: Headers,
    pub state: ParserState,
}

impl Request {
    fn new() -> Self {
        Self {
            request_line: RequestLine::default(),
            headers: Headers::new(),
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
                    match parse_request_line(current_data) {
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
                            return Err(anyhow!(format!("{}: the request is in Error state", e)));
                        }
                    }
                },
                ParserState::StateHeaders => {
                    match self.headers.parse(current_data) {
                        Ok(r) => {
                            if r.0 == 0 {
                                break;
                            }

                            read += r.0; // No of bytes we have processed
                            if r.1 {
                                self.state = ParserState::StateDone;
                            }
                        },
                        Err(e) => {
                            self.state = ParserState::StateError;
                            return Err(anyhow!(format!("{}: the request is in Error state", e)));
                        }
                    }
                },
                ParserState::StateDone => break,
                ParserState::StateError => break,
            }
        }
        Ok(read)
    }
}

const SEPARATOR: &'static str = "\r\n";
const MALFORMED_REQUEST: &'static str = "Request is Malformed";

fn parse_request_line(bytes: &[u8]) -> Result<(RequestLine, usize), anyhow::Error> {
    let idx: usize;
    let data = str::from_utf8(bytes)?;

    match data.find(SEPARATOR) {
        Some(us) => idx = us,
        None => return Ok((RequestLine::default(), 0)), // If there was no separator that means the request line was not full
    }

    let r_line = &data[0..idx];
    let read = idx + SEPARATOR.len();

    let parts = r_line.split(" ").collect::<Vec<_>>(); // RFC 9112: request-line   = method SP request-target SP HTTP-version
    if parts.len() != 3 {
        return Err(anyhow!(format!(
            "{}: the request was incomplete",
            MALFORMED_REQUEST
        )));
    }

    let http_parts = parts[2].split("/").collect::<Vec<_>>(); // RFC 9112: HTTP-version  = HTTP-name "/" DIGIT "." DIGIT
    if http_parts.len() != 2 {
        return Err(anyhow!(format!(
            "{}: the HTTP method was incorrect",
            MALFORMED_REQUEST
        )));
    }

    if !parts[0].chars().all(|c| c.is_uppercase()) {
        // RFC 9112: HTTP-name     = %s"HTTP"
        return Err(anyhow!(format!(
            "{}: the Request method was incorrect",
            MALFORMED_REQUEST
        )));
    }

    let request_line = RequestLine {
        http_version: http_parts[1].to_string(),
        request_target: parts[1].to_string(),
        method: parts[0].to_string(),
    };

    Ok((request_line, read))
}

pub fn request_from_reader(mut f: impl Read) -> Result<Request, anyhow::Error> {
    let mut req = Request::new();
    let mut buffer = [0; 1024];
    let mut buffer_length = 0;

    while !req.done() {
        let n = f.read(&mut buffer[buffer_length..])?; // n is the number of bytes read from f i.e. our connection/file etc

        if n == 0 && !req.done() {
            return Err(anyhow!("Malformed request"));
        }
        
        buffer_length += n;

        // println!("Data to parse in request:{}", str::from_utf8(&buffer[..buffer_length])?);
        let read_n = req.parse(&buffer[..buffer_length])?; // read_n is how many bytes we have parsed

        buffer.copy_within(read_n..buffer_length, 0); // We don't need the bytes we have processed and hence we copy the remaining bytes to the starting of the buffer and reduce buffer length
        buffer_length -= read_n;
    }

    Ok(req)
}
