mod slow_reader;

#[cfg(test)]
mod tests {
    use crate::slow_reader::SlowReader;
    use anyhow::anyhow;
    use tcp_to_http::request::headers;
    use tcp_to_http::request::request_from_reader;

    #[tokio::main]
    #[test]
    async fn test_header() -> Result<(), anyhow::Error> {
        // Valid Header
        let mut http_headers = headers::Headers::new();
        let data = b"Host: localhost:42069\r\n\r\n";
        let n = http_headers.parse(data)?;
        assert_eq!(25, n.0);
        match http_headers.get(&"Host".to_string()) {
            Some(val) => assert_eq!(*val, "localhost:42069".to_string()),
            None => return Err(anyhow!(format!("Value not found"))),
        }
        assert_eq!(n.1, true);

        // Invalid Header with extra whitespaces
        let mut http_headers = headers::Headers::new();
        let data = b"       Host: localhost:42069\r\n\r\n";
        let n = http_headers.parse(data);
        assert!(n.is_err());

        // Valid header with OWS in field value
        let mut http_headers = headers::Headers::new();
        let data = b"Host:    localhost:42069   \r\n\r\n";
        let n = http_headers.parse(data)?;
        assert_eq!(31, n.0);
        match http_headers.get(&"Host".to_string()) {
            Some(val) => assert_eq!(*val, "localhost:42069".to_string()),
            None => return Err(anyhow!(format!("Value not found"))),
        }
        assert_eq!(n.1, true);

        // Valid header with multiple header values
        let mut http_headers = headers::Headers::new();
        let data = b"Host: localhost:42069\r\nUser-Agent: curl\r\n\r\n";
        let n = http_headers.parse(data)?;
        assert_eq!(43, n.0);
        match http_headers.get(&"Host".to_string()) {
            Some(val) => assert_eq!(*val, "localhost:42069".to_string()),
            None => return Err(anyhow!(format!("Value not found"))),
        }
        match http_headers.get(&"User-Agent".to_string()) {
            Some(val) => assert_eq!(*val, "curl".to_string()),
            None => return Err(anyhow!(format!("Value not found"))),
        }
        assert_eq!(n.1, true);

        // Invalid field name
        let mut http_headers = headers::Headers::new();
        let data = b"H@st: localhost:42069\r\n\r\n";
        let n = http_headers.parse(data);
        assert!(n.is_err());

        // Field names with multiple values
        let mut http_headers = headers::Headers::new();
        let data = b"Host:localhost:42069 \r\nHost:localhost:42069  \r\n\r\n";
        let n = http_headers.parse(data)?;
        assert_eq!(49, n.0);
        match http_headers.get(&"Host".to_string()) {
            Some(val) => assert_eq!(*val, "localhost:42069,localhost:42069".to_string()),
            None => return Err(anyhow!(format!("Value not found"))),
        }
        assert_eq!(n.1, true);

        // Tests from Request
        // Valid Header
        let mut reader = SlowReader {
            data: b"GET / HTTP/1.1\r\nHost: localhost:42069\r\nUser-Agent: curl/7.81.0\r\nAccept: */*\r\n\r\n".to_vec(),
            pos: 0,
            read_size: 3
        };

        let req = request_from_reader(&mut reader).await?;

        assert_eq!(req.request_line.method, "GET");
        assert_eq!(req.request_line.request_target, "/");
        assert_eq!(req.request_line.http_version, "1.1");
        match req.headers.get(&"Host".to_string()) {
            Some(val) => assert_eq!(val, "localhost:42069"),
            None => return Err(anyhow!("Value not found")),
        }
        match req.headers.get(&"user-agent".to_string()) {
            Some(val) => assert_eq!(val, "curl/7.81.0"),
            None => return Err(anyhow!("Value not found")),
        }
        match req.headers.get(&"accept".to_string()) {
            Some(val) => assert_eq!(val, "*/*"),
            None => return Err(anyhow!("Value not found")),
        }

        // No headers
        let mut reader = SlowReader {
            data: b"GET / HTTP/1.1\r\n\r\n".to_vec(),
            pos: 0,
            read_size: 3,
        };

        let req = request_from_reader(&mut reader).await?;

        assert_eq!(req.request_line.method, "GET");
        assert_eq!(req.request_line.request_target, "/");
        assert_eq!(req.request_line.http_version, "1.1");

        assert!(req.headers.get(&"host".to_string()).is_none());

        // Invalid header
        let mut reader = SlowReader {
            data: b"GET / HTTP/1.1\r\nHost localhost:42069\r\n\r\n".to_vec(),
            pos: 0,
            read_size: 3,
        };

        let result = request_from_reader(&mut reader).await;

        assert!(result.is_err());

        // Multiple values of a header
        let mut reader = SlowReader {
            data: b"GET / HTTP/1.1\r\nAccept: text/html\r\nAccept: application/json\r\n\r\n"
                .to_vec(),
            pos: 0,
            read_size: 3,
        };

        let req = request_from_reader(&mut reader).await?;

        match req.headers.get(&"accept".to_string()) {
            Some(val) => assert_eq!(val, "text/html,application/json"),
            None => return Err(anyhow!("Accept not found")),
        }

        // Case insensitive header
        let mut reader = SlowReader {
            data: b"GET / HTTP/1.1\r\nHoSt: localhost:42069\r\nUsEr-AgEnT: curl\r\n\r\n".to_vec(),
            pos: 0,
            read_size: 3,
        };

        let req = request_from_reader(&mut reader).await?;

        match req.headers.get(&"host".to_string()) {
            Some(val) => assert_eq!(val, "localhost:42069"),
            None => return Err(anyhow!("Host not found")),
        }

        match req.headers.get(&"user-agent".to_string()) {
            Some(val) => assert_eq!(val, "curl"),
            None => return Err(anyhow!("User-Agent not found")),
        }

        // Invalid ending
        let mut reader = SlowReader {
            data: b"GET / HTTP/1.1\r\nHoSt: localhost:42069\r\nUsEr-AgEnT: curl\r\n".to_vec(),
            pos: 0,
            read_size: 3,
        };

        let req = request_from_reader(&mut reader).await;
        assert!(req.is_err());

        Ok(())
    }
}
