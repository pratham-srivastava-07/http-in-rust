

// #[cfg(test)]
// mod tests {
//     use crate::internal::requests::request::request_from_reader;

//     use std::io::Cursor;

//     pub fn add_number(a: usize, b: usize) -> usize {
//         a +  b
//     }

//     #[test]
//     fn it_works() {
//         let res = add_number(1, 3);
//         assert_eq!(res, 4);
//     }
//     #[test]
//     fn good_get_request_line() {
//         let url = "GET / HTTP/1.1\r\nHost: localhost:42069\r\n\r\n";

//         let reader = Cursor::new(url);

//         let res = request_from_reader(reader);

//         assert!(res.is_ok());
//     }

//     #[test]
//     fn test_standard_headers() {
//         let data = b"GET / HTTP/1.1\r\nHost: localhost:42069\r\nUser-Agent: curl/7.81.0\r\nAccept: */*\r\n\r\n";
//         let cursor = Cursor::new(data);
        
//         let req = request_from_reader(cursor).unwrap();
        
//         assert_eq!(req.headers.get("host"), Some(&"localhost:42069".to_string()));
//         assert_eq!(req.headers.get("user-agent"), Some(&"curl/7.81.0".to_string()));
//         assert_eq!(req.headers.get("accept"), Some(&"*/*".to_string()));
//     }

//     #[test]
//     fn test_empty_headers() {
//         let data = b"GET / HTTP/1.1\r\n\r\n";
//         let cursor = Cursor::new(data);
        
//         let req = request_from_reader(cursor).unwrap();
        
//         assert!(req.headers.map.is_empty());
//     }

//     #[test]
//     fn test_malformed_header() {
//         let data = b"GET / HTTP/1.1\r\nHost localhost:42069\r\n\r\n";
//         let cursor = Cursor::new(data);
        
//         let result = request_from_reader(cursor);
//         assert!(result.is_err());
//     }

//     #[test]
//     fn test_duplicate_headers() {
//         let data = b"GET / HTTP/1.1\r\nAccept: text/html\r\nAccept: application/xhtml+xml\r\n\r\n";
//         let cursor = Cursor::new(data);
        
//         let req = request_from_reader(cursor).unwrap();
        
//         assert_eq!(req.headers.get("accept"), Some(&"text/html, application/xhtml+xml".to_string()));
//     }

//     #[test]
//     fn test_case_insensitive_headers() {
//         let data = b"GET / HTTP/1.1\r\nHOST: localhost:42069\r\n\r\n";
//         let cursor = Cursor::new(data);
        
//         let req = request_from_reader(cursor).unwrap();
        
//         assert_eq!(req.headers.get("host"), Some(&"localhost:42069".to_string()));
//     }

//     #[test]
//     fn test_missing_end_of_headers() {
//         let data = b"GET / HTTP/1.1\r\nHost: localhost:42069\r\n";
//         let cursor = Cursor::new(data);
        
//         let req = request_from_reader(cursor).unwrap();
//         // Since we don't have \r\n\r\n, it shouldn't reach Done state
//         // and shouldn't complete the full parse loop cleanly
//         // Alternatively, it might just return with what it has when reader returns 0
        
//         assert_eq!(req.headers.get("host"), Some(&"localhost:42069".to_string()));
//     }
// }