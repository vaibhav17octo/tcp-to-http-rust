use anyhow::anyhow;
use std::collections::HashMap;

pub struct Headers(HashMap<String, String>);

const SEPARATOR: &'static str = "\r\n";
const MALFORMED_HEADER: &'static str = "HEADER is Malformed";
const MALFORMED_FIELD_NAME: &'static str = "Field value is Malformed";

impl Headers {
    pub fn new() -> Self {
        Headers(HashMap::new())
    }

    fn add_header(&mut self, key: String, value: String) {
        self.0.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<&String> {
        let value = self.0.get(&key);
        return value;
    }

    pub fn parse(&mut self, buffer: &[u8]) -> Result<(usize, bool), anyhow::Error> {
        let data = str::from_utf8(buffer)?;
        println!("Data length{}", data.len());
        let mut idx: usize;
        let mut read = 0;
        loop {
            match data[read..].find(SEPARATOR) {
                Some(us) => idx = us,
                None => return Ok((0, false)), // If there was no separator that means the request line was not full
            }

            // if SEPERATOR is at the start of the data, you've found the end of the headers, so return the proper values immediately.
            if idx == 0 {
                read += SEPARATOR.len();
                return Ok((read, true));
            }

            // RFC 9112: field-line   = field-name ":" OWS field-value OWS
            match data[read..read + idx].split_once(":") {
                Some(parts) => {
                    let field_name = parts.0.to_string();
                    let field_value = parts.1.trim().to_string(); // RFC 9112 Field value can have any number of whitespaces

                    // RFC 9112 field name should not have any whitespaces before or after
                    if field_name.ends_with(" ") || field_name.starts_with(" ") {
                        return Err(anyhow!(format!("{}", MALFORMED_FIELD_NAME)));
                    }

                    read += data[read..read + idx].len() + SEPARATOR.len();
                    println!("read at {}", read);

                    self.add_header(field_name, field_value);
                }
                None => {
                    return Err(anyhow!(format!("{}", MALFORMED_HEADER)));
                }
            }
        }
    }
}
