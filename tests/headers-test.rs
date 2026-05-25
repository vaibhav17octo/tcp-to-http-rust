#[cfg(test)]
mod tests {
    use anyhow::anyhow;
    use tcp_to_http::headers;

    #[test]
    fn test_parse() -> Result<(), anyhow::Error> {
        let mut http_headers = headers::Headers::new();

        // // Valid Header
        // let data = b"Host: localhost:42069\r\n\r\n";
        // let n = http_headers.parse(data)?;
        // assert_eq!(25, n.0);
        // match http_headers.get("Host".to_string()) {
        //     Some(val) => assert_eq!(*val, "localhost:42069".to_string()),
        //     None => return Err(anyhow!(format!("Value not found")))
        // }
        // assert_eq!(n.1, true);

        // // Invalid Header with extra whitespaces
        // let data = b"       Host: localhost:42069\r\n\r\n";
        // let n = http_headers.parse(data);
        // assert!(n.is_err());

        //
        // let data = b"Host2:    localhost:42069   \r\n\r\n";
        // let n = http_headers.parse(data)?;
        // assert_eq!(32, n.0);
        // match http_headers.get("Host2".to_string()) {
        //     Some(val) => assert_eq!(*val, "localhost:42069".to_string()),
        //     None => return Err(anyhow!(format!("Value not found")))
        // }
        // assert_eq!(n.1, true);

        //
        let data = b"Host3: localhost:42069\r\nUser-Agent: curl\r\n\r\n";
        let n = http_headers.parse(data)?;
        assert_eq!(44, n.0);
        match http_headers.get("Host3".to_string()) {
            Some(val) => assert_eq!(*val, "localhost:42069".to_string()),
            None => return Err(anyhow!(format!("Value not found"))),
        }
        match http_headers.get("User-Agent".to_string()) {
            Some(val) => assert_eq!(*val, "curl".to_string()),
            None => return Err(anyhow!(format!("Value not found"))),
        }
        assert_eq!(n.1, true);

        Ok(())
    }
}
