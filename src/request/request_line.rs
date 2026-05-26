use anyhow::anyhow;
use std::str;
use core::fmt;

use crate::config::SEPARATOR;
use crate::config::MALFORMED_REQUEST_LINE;
use crate::config::MALFORMED_HTTP_VERSION;
use crate::config::MALFORMED_METHOD;

#[derive(Default)]
pub struct RequestLine {
    pub http_version: String,
    pub request_target: String,
    pub method: String,
}

impl fmt::Display for RequestLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Request line:\n- Method: {}\n- Target: {}\n- Version: {}",
            self.method,
            self.request_target,
            self.http_version
        )
    }
}

impl RequestLine {
    pub fn parse_request_line(bytes: &[u8]) -> Result<(RequestLine, usize), anyhow::Error> {
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
                MALFORMED_REQUEST_LINE
            )));
        }

        let http_parts = parts[2].split("/").collect::<Vec<_>>(); // RFC 9112: HTTP-version  = HTTP-name "/" DIGIT "." DIGIT
        if http_parts.len() != 2 {
            return Err(anyhow!(format!(
                "{}: the HTTP method was incorrect",
                MALFORMED_HTTP_VERSION
            )));
        }

        if !parts[0].chars().all(|c| c.is_uppercase()) {
            // RFC 9112: HTTP-name     = %s"HTTP"
            return Err(anyhow!(format!(
                "{}: the Request method was incorrect",
                MALFORMED_METHOD
            )));
        }

        let request_line = RequestLine {
            http_version: http_parts[1].to_string(),
            request_target: parts[1].to_string(),
            method: parts[0].to_string(),
        };

        Ok((request_line, read))
    }
}







