mod slow_reader;

#[cfg(test)]
mod tests {
    use crate::slow_reader::SlowReader;
    use tcp_to_http::request::request_from_reader;

    #[test]
    fn test_body() -> Result<(), anyhow::Error> {
        // Valid Body and content length
        let reader = SlowReader {
            data: b"POST /submit HTTP/1.1\r\nHost: localhost:42069\r\nContent-Length: 13\r\n\r\nhello world!\n".to_vec(),
            pos: 0,
            read_size: 3,
        };

        let req = request_from_reader(reader)?;
        assert_eq!(req.body.get_body(), b"hello world!\n");

        // Body length less than content length
        let reader = SlowReader {
            data: b"POST /submit HTTP/1.1\r\nHost: localhost:42069\r\nContent-Length: 20\r\n\r\npartial content\n".to_vec(),
            pos: 0,
            read_size: 3,
        };

        let req = request_from_reader(reader);
        assert!(req.is_err());

        // 0 content length
        let reader = SlowReader {
            data: b"POST /submit HTTP/1.1\r\nHost: localhost:42069\r\nContent-Length: 0\r\n\r\n"
                .to_vec(),
            pos: 0,
            read_size: 3,
        };

        let req = request_from_reader(reader)?;
        assert_eq!(req.body.get_body(), b"");

        Ok(())
    }
}
