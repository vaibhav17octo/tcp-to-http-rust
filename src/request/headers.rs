use anyhow::anyhow;
use core::fmt;
use std::collections::HashMap;

use crate::config::SEPARATOR;
use crate::config::MALFORMED_FIELD_NAME;
use crate::config::MALFORMED_HEADER;

pub struct Headers{
    headers: HashMap<String, String>
}

impl fmt::Display for Headers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output: String = "Headers:\n".to_string();
        for (key, value) in &self.headers {
            let add = format!("- {}, {}\n", key, value);
            output.push_str(&add[..]);
        }
        write!(f, "{}", output)
    }
}

impl Headers {
    pub fn new() -> Self {
        Headers{
            headers: HashMap::new()
        }
    }

    fn set(&mut self, key: String, value: String) {

        // RFC 9110 5.2 If field-name exists then we add the value with , separated
        self.headers
        .entry(key.to_lowercase())
        .and_modify(|current_value| {
            let add_to_current = String::from(",") + &value;
            current_value.push_str(&add_to_current);
        }).
        or_insert(value);
    }

    pub fn get(&self, key: &String) -> Option<&String> {
        let value = self.headers.get(&key.to_lowercase());
        return value;
    }

    pub fn is_valid_field_name(&self, field_name: &String) -> bool {
        // RFC 9112 field name should not have any whitespaces before or after
        if field_name.ends_with(" ") || field_name.starts_with(" ") {
            return false;
        }

        // RFC 9110 field name is a token
        // Uppercase letters: A-Z
        // Lowercase letters: a-z
        // Digits: 0-9
        // Special characters: !, #, $, %, &, ', *, +, -, ., ^, _, `, |, ~
        for ch in field_name.chars() {
            if !ch.is_alphanumeric() && !"!#$%&'*+-.^_`|~".contains(ch) {
                return false;
            }
        }

        return true;
    }

    pub fn parse(&mut self, buffer: &[u8]) -> Result<(usize, bool), anyhow::Error> {
        let data = str::from_utf8(buffer)?;
        let mut idx: usize;
        let mut read = 0;
        let mut done: bool = false;
        loop {
            match data[read..].find(SEPARATOR) {
                Some(us) => {
                    idx = us
                },
                None => break , // If there was no separator that means the request line was not full
            }

            // if SEPERATOR is at the start of the data, you've found the end of the headers and the request is done
            if idx == 0 {
                read += SEPARATOR.len();
                done = true;
                break;
            }

            // RFC 9112: field-line   = field-name ":" OWS field-value OWS
            match data[read..read + idx].split_once(":") {
                Some(parts) => {
                    let field_name = parts.0.to_string();
                    let field_value = parts.1.trim().to_string(); // RFC 9112 Field value can have any number of whitespaces

                    if !self.is_valid_field_name(&field_name) {
                        return Err(anyhow!(format!("{} Field name: {} ", MALFORMED_FIELD_NAME, field_name)));
                    }

                    read += data[read..read + idx].len() + SEPARATOR.len();

                    self.set(field_name, field_value);
                }
                None => {
                    return Err(anyhow!(format!("{}", MALFORMED_HEADER)));
                }
            }
        }

        return Ok((read, done));
    }
}
