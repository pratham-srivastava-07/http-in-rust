use std::io::{self, Read};

pub struct RequestLine {
    http_version: String,
    request_target: String,
    method: String
}

pub struct Request {
    pub request_line: RequestLine
}


fn request_from_reader(reader: impl Read) -> Result<Request, io::Error> {
    todo!()
}