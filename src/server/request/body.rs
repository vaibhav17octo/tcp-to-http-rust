use anyhow::anyhow;
use core::fmt;

pub struct Body {
    body: Vec<u8>,
    content_length: usize,
}

impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Body:\n{:?}", String::from_utf8_lossy(&self.body))
    }
}

impl Body {
    pub fn new() -> Self {
        Body {
            body: Vec::new(),
            content_length: 0,
        }
    }

    pub fn set_content_length(&mut self, content_length: usize) {
        self.content_length = content_length;
    }

    pub fn get_body(&self) -> &Vec<u8> {
        &self.body
    }

    pub fn parse_body(&mut self, data: &[u8]) -> Result<(usize, bool), anyhow::Error> {
        let mut temp = data.to_vec();
        self.body.append(&mut temp);

        if self.content_length == self.body.len() {
            return Ok((temp.len(), true));
        } else if self.content_length < self.body.len() {
            return Err(anyhow!(
                "Length of the body is larger than what was provided in content length"
            ));
        }

        Ok((data.len(), false))
    }
}
