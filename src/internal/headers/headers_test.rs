#[cfg(test)]
mod tests {
    use std::io;

    use crate::internal::headers::headers::Headers; 

    #[test]
    fn valid_single_header() -> Result<(), io::Error> {
        let mut headers = Headers::new();
        let data = b"Host: localhost:42069\r\n\r\n";

        let (n, done) = headers.parse(data)?;

        assert_eq!(headers.get("Host"), Some(&"localhost:42069".to_string()));
        assert_eq!(n, 23);
        assert!(!done);

        Ok(())
    }

    #[test]
    fn invalid_spacing_header() {
        let mut headers = Headers::new();
        let data = b"       Host : localhost:42069       \r\n\r\n";

        assert!(headers.parse(data).is_err());
    }
}