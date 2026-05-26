#[cfg(test)]
mod tests {
    use anyhow::anyhow;
    use tcp_to_http::headers;

    #[test]
    fn test_parse() -> Result<(), anyhow::Error> {
        // Valid Header
        let mut http_headers = headers::Headers::new();
        let data = b"Host: localhost:42069\r\n\r\n";
        let n = http_headers.parse(data)?;
        assert_eq!(25, n.0);
        match http_headers.get(&"Host".to_string()) {
            Some(val) => assert_eq!(*val, "localhost:42069".to_string()),
            None => return Err(anyhow!(format!("Value not found")))
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
            None => return Err(anyhow!(format!("Value not found")))
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
            None => return Err(anyhow!(format!("Value not found")))
        }
        assert_eq!(n.1, true);
        
        Ok(())
    }
}
