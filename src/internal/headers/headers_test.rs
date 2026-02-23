#[cfg(test)]
mod tests {
    use crate::internal::headers::headers::Headers;

    use super::*;

    #[test]
    fn valid_single_header() {
        let mut headers = Headers::new();
        let data = b"Host: localhost:42069\r\n\r\n";

        let (n, done) = headers.parse(data).unwrap();

        assert_eq!(headers.get("Host"), Some(&"localhost:42069".to_string()));
        assert_eq!(n, 23);
        assert!(!done);
    }

    #[test]
    fn invalid_spacing_header() {
        let mut headers = Headers::new();
        let data = b"       Host : localhost:42069       \r\n\r\n";

        assert!(headers.parse(data).is_err());
    }
}