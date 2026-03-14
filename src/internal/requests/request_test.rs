#[cfg(test)]
mod tests {
    use crate::internal::requests::request::request_from_reader;
    use std::io::{Cursor, Read};

    struct ChunkReader {
        data: Vec<u8>,
        num_bytes_per_read: usize,
        pos: usize,
    }

    impl Read for ChunkReader {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            if self.pos >= self.data.len() {
                return Ok(0);
            }
            let remaining = self.data.len() - self.pos;
            let to_read = std::cmp::min(self.num_bytes_per_read, remaining);
            let to_read = std::cmp::min(to_read, buf.len());
            
            buf[..to_read].copy_from_slice(&self.data[self.pos..self.pos + to_read]);
            self.pos += to_read;
            Ok(to_read)
        }
    }

    #[test]
    fn test_standard_body() {
        let reader = ChunkReader {
            data: b"POST /submit HTTP/1.1\r\nHost: localhost:42069\r\nContent-Length: 13\r\n\r\nhello world!\n".to_vec(),
            num_bytes_per_read: 3,
            pos: 0,
        };
        let req = request_from_reader(reader).expect("Failed to parse request");
        assert_eq!(req.body, b"hello world!\n");
    }

    #[test]
    fn test_body_shorter_than_reported() {
        let reader = ChunkReader {
            data: b"POST /submit HTTP/1.1\r\nHost: localhost:42069\r\nContent-Length: 20\r\n\r\npartial content".to_vec(),
            num_bytes_per_read: 3,
            pos: 0,
        };
        let res = request_from_reader(reader);
        assert!(res.is_err(), "Expected error for body shorter than content-length");
    }

    #[test]
    fn test_empty_body_0_content_length() {
        let reader = ChunkReader {
            data: b"POST /submit HTTP/1.1\r\nHost: localhost:42069\r\nContent-Length: 0\r\n\r\n".to_vec(),
            num_bytes_per_read: 3,
            pos: 0,
        };
        let req = request_from_reader(reader).expect("Failed to parse request");
        assert_eq!(req.body, b"");
    }

    #[test]
    fn test_empty_body_no_content_length() {
        let reader = ChunkReader {
            data: b"GET / HTTP/1.1\r\nHost: localhost:42069\r\n\r\n".to_vec(),
            num_bytes_per_read: 3,
            pos: 0,
        };
        let req = request_from_reader(reader).expect("Failed to parse request");
        assert_eq!(req.body, b"");
    }

    #[test]
    fn test_no_content_length_but_body_exists() {
        let reader = ChunkReader {
            data: b"POST /submit HTTP/1.1\r\nHost: localhost\r\n\r\nextra_data".to_vec(),
            num_bytes_per_read: 3,
            pos: 0,
        };
        let req = request_from_reader(reader).expect("Failed to parse request");
        // It shouldn't error, but it should ignore the body
        assert_eq!(req.body, b"");
    }

    #[test]
    fn good_get_request_line() {
        let url = "GET / HTTP/1.1\r\nHost: localhost:42069\r\n\r\n";
        let reader = Cursor::new(url);
        let res = request_from_reader(reader);
        assert!(res.is_ok());
    }

    #[test]
    fn test_standard_headers() {
        let data = b"GET / HTTP/1.1\r\nHost: localhost:42069\r\nUser-Agent: curl/7.81.0\r\nAccept: */*\r\n\r\n";
        let cursor = Cursor::new(data);
        let req = request_from_reader(cursor).unwrap();
        assert_eq!(req.headers.get("host"), Some(&"localhost:42069".to_string()));
        assert_eq!(req.headers.get("user-agent"), Some(&"curl/7.81.0".to_string()));
        assert_eq!(req.headers.get("accept"), Some(&"*/*".to_string()));
    }

    #[test]
    fn test_empty_headers() {
        let data = b"GET / HTTP/1.1\r\n\r\n";
        let cursor = Cursor::new(data);
        let req = request_from_reader(cursor).unwrap();
        assert!(req.headers.map.is_empty());
    }

    #[test]
    fn test_malformed_header() {
        let data = b"GET / HTTP/1.1\r\nHost localhost:42069\r\n\r\n";
        let cursor = Cursor::new(data);
        let result = request_from_reader(cursor);
        assert!(result.is_err());
    }

    #[test]
    fn test_duplicate_headers() {
        let data = b"GET / HTTP/1.1\r\nAccept: text/html\r\nAccept: application/xhtml+xml\r\n\r\n";
        let cursor = Cursor::new(data);
        let req = request_from_reader(cursor).unwrap();
        assert_eq!(req.headers.get("accept"), Some(&"text/html, application/xhtml+xml".to_string()));
    }

    #[test]
    fn test_case_insensitive_headers() {
        let data = b"GET / HTTP/1.1\r\nHOST: localhost:42069\r\n\r\n";
        let cursor = Cursor::new(data);
        let req = request_from_reader(cursor).unwrap();
        assert_eq!(req.headers.get("host"), Some(&"localhost:42069".to_string()));
    }

    #[test]
    fn test_missing_end_of_headers() {
        let data = b"GET / HTTP/1.1\r\nHost: localhost:42069\r\n";
        let cursor = Cursor::new(data);
        let req = request_from_reader(cursor).unwrap();
        assert_eq!(req.headers.get("host"), Some(&"localhost:42069".to_string()));
    }
}