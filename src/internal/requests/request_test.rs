

#[cfg(test)]
mod tests {
    use crate::internal::requests::request::request_from_reader;

    use std::io::Cursor;

    pub fn add_number(a: usize, b: usize) -> usize {
    a +  b
}

    #[test]
    fn it_works() {
        let res = add_number(1, 3);
        assert_eq!(res, 4);
    }
    #[test]
    fn good_get_request_line() {
        let url = "GET / HTTP/1.1\r\nHost: localhost:42069\r\n\r\n";

        let reader = Cursor::new(url);

        let res = request_from_reader(reader);

        assert!(res.is_ok());
    }
}