mod slow_reader;

#[cfg(test)]
mod tests {
    use crate::slow_reader::SlowReader;
    use std::io::Cursor;
    use tcp_to_http::request::request_from_reader;

    #[tokio::main]
    #[test]
    async fn test_request_line() -> Result<(), anyhow::Error> {
        let mut reader = Cursor::new(
            "GET / HTTP/1.1\r\nHost: localhost:42069\r\nUser-Agent: curl/7.81.0\r\nAccept: */*\r\n\r\n",
        );
        let req = request_from_reader(&mut reader).await?;

        assert_eq!(req.request_line.method, "GET");
        assert_eq!(req.request_line.request_target, "/");
        assert_eq!(req.request_line.http_version, "1.1");

        let mut reader = Cursor::new(
            "GET /coffee HTTP/1.1\r\nHost: localhost:42069\r\nUser-Agent: curl/7.81.0\r\nAccept: */*\r\n\r\n",
        );
        let req = request_from_reader(&mut reader).await?;

        assert_eq!(req.request_line.method, "GET");
        assert_eq!(req.request_line.request_target, "/coffee");
        assert_eq!(req.request_line.http_version, "1.1");

        let mut reader = Cursor::new(
            "/coffee HTTP/1.1\r\nHost: localhost:42069\r\nUser-Agent: curl/7.81.0\r\nAccept: */*\r\n\r\n",
        );
        let req = request_from_reader(&mut reader).await;
        assert!(req.is_err());

        let mut reader = SlowReader {
            data: b"GET / HTTP/1.1\r\nHost: localhost:42069\r\nUser-Agent: curl/7.81.0\r\nAccept: */*\r\n\r\n".to_vec(),
            pos: 0,
            read_size: 3
        };
        println!("{}", reader.data.len());
        let req = request_from_reader(&mut reader).await?;

        assert_eq!(req.request_line.method, "GET");
        assert_eq!(req.request_line.request_target, "/");
        assert_eq!(req.request_line.http_version, "1.1");

        let mut reader = SlowReader {
            data: b"GET /coffee HTTP/1.1\r\nHost: localhost:42069\r\nUser-Agent: curl/7.81.0\r\nAccept: */*\r\n\r\n".to_vec(),
            pos: 0,
            read_size: 1
        };
        println!("{}", reader.data.len());
        let req = request_from_reader(&mut reader).await?;

        assert_eq!(req.request_line.method, "GET");
        assert_eq!(req.request_line.request_target, "/coffee");
        assert_eq!(req.request_line.http_version, "1.1");

        Ok(())
    }
}
