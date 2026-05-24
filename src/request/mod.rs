use anyhow::anyhow;
use std::io::Read;
struct RequestLine {
    http_version: String,
    request_target: String,
    method: String,
}

struct Request {
    request_line: RequestLine,
}

const SEPARATOR: &'static str = "\r\n";
const MALFORMED_REQUEST: &'static str = "Request is Malformed";

fn get_request_line(s: String) -> Result<(RequestLine, String), anyhow::Error> {
    let idx: usize;
    match s.find(SEPARATOR) {
        Some(us) => idx = us,
        None => return Err(anyhow!(MALFORMED_REQUEST)),
    }

    let r_line = &s[0..idx];
    let rest_string = s[idx + SEPARATOR.len()..].to_string();

    let parts = r_line.split(" ").collect::<Vec<_>>();
    if parts.len() != 3 {
        return Err(anyhow!(format!(
            "{}: the request was incomplete",
            MALFORMED_REQUEST
        )));
    }

    let http_parts = parts[2].split("/").collect::<Vec<_>>();
    if http_parts.len() != 2 {
        return Err(anyhow!(format!(
            "{}: the HTTP method was incorrect",
            MALFORMED_REQUEST
        )));
    }

    if !parts[0].chars().all(|c| c.is_uppercase()) {
        return Err(anyhow!(format!(
            "{}: the Request method was incorrect",
            MALFORMED_REQUEST
        )));
    }

    let request_line = RequestLine {
        http_version: http_parts[1].to_string(),
        request_target: parts[1].to_string(),
        method: parts[0].to_string(),
    };

    Ok((request_line, rest_string))
}

pub fn request_from_reader(mut f: impl Read) -> Result<Request, anyhow::Error> {
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    let parsed_request_line = get_request_line(buffer)?;

    let request = Request {
        request_line: parsed_request_line.0,
    };

    Ok(request)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::request::request_from_reader;

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

        Ok(())
    }
}
