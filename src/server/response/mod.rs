pub mod writer;

use crate::server::headers::Headers;

pub enum StatusCode {
    OK = 200,
    BadRequest = 400,
    InternalServerError = 500,
}

pub struct Response {
    pub status: StatusCode,
    pub headers: Headers,
    pub body: Vec<u8>,
    pub trailers: Option<Headers>,
}

impl Response {
    pub fn new(
        status: StatusCode,
        headers: Headers,
        body: Vec<u8>,
        trailers: Option<Headers>,
    ) -> Self {
        Response {
            status: status,
            headers: headers,
            body: body,
            trailers: trailers,
        }
    }
}
