use std::io::{self, Read};

struct SlowReader {
    data: Vec<u8>,
    pos: usize,
    read_size: usize
}

impl Read for SlowReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.pos >= self.data.len() {
            return Ok(0); // Reached EOL
        }

        let idx = self.pos+self.read_size;
        buf[..self.read_size].copy_from_slice(&self.data[self.pos..idx]);

        self.pos += self.read_size;

        Ok(self.read_size)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use tcp_to_http::request::request_from_reader;

use crate::SlowReader;

    #[test]
    fn test_request_from_reader() -> Result<(), anyhow::Error> {
        let reader = Cursor::new("GET / HTTP/1.1\r\nHost: localhost:42069\r\nUser-Agent: curl/7.81.0\r\nAccept: */*\r\n\r\n");
        let req = request_from_reader(reader)?;

        assert_eq!(req.request_line.method, "GET");
        assert_eq!(req.request_line.request_target, "/");
        assert_eq!(req.request_line.http_version, "1.1");

        let reader = Cursor::new("GET /coffee HTTP/1.1\r\nHost: localhost:42069\r\nUser-Agent: curl/7.81.0\r\nAccept: */*\r\n\r\n");
        let req = request_from_reader(reader)?;

        assert_eq!(req.request_line.method, "GET");
        assert_eq!(req.request_line.request_target, "/coffee");
        assert_eq!(req.request_line.http_version, "1.1");

        let reader = Cursor::new("/coffee HTTP/1.1\r\nHost: localhost:42069\r\nUser-Agent: curl/7.81.0\r\nAccept: */*\r\n\r\n");
        let req = request_from_reader(reader);
        assert!(req.is_err());

        let reader = SlowReader {
            data: b"GET / HTTP/1.1\r\nHost: localhost:42069\r\nUser-Agent: curl/7.81.0\r\nAccept: */*\r\n\r\n".to_vec(),
            pos: 0,
            read_size: 3
        };
        println!("{}", reader.data.len());
        let req = request_from_reader(reader)?;

        assert_eq!(req.request_line.method, "GET");
        assert_eq!(req.request_line.request_target, "/");
        assert_eq!(req.request_line.http_version, "1.1");

        let reader = SlowReader {
            data: b"GET /coffee HTTP/1.1\r\nHost: localhost:42069\r\nUser-Agent: curl/7.81.0\r\nAccept: */*\r\n\r\n".to_vec(),
            pos: 0,
            read_size: 1
        };
        println!("{}", reader.data.len());
        let req = request_from_reader(reader)?;

        assert_eq!(req.request_line.method, "GET");
        assert_eq!(req.request_line.request_target, "/coffee");
        assert_eq!(req.request_line.http_version, "1.1");

        Ok(())
    }
}