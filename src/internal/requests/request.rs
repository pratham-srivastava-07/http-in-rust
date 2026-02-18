use std::{io::{self, Read}, str::SplitWhitespace};

pub struct RequestLine {
    http_version: String,
    request_target: String,
    method: String
}

pub struct Request {
    pub request_line: RequestLine
}


 pub fn request_from_reader(mut reader: impl Read) -> Result<Request, io::Error> {
    // first create a simple buffer
    let mut buffer = [0; 1024];

    // then read all bytes 
    let bytes_to_be_read = reader.read(&mut buffer)?;
    println!("{:?}", bytes_to_be_read);

    let request_string = String::from_utf8_lossy(&buffer[..bytes_to_be_read]);

    let first_line = request_string.lines().next()
    .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "No input received"))
    ?;

    let mut parts = first_line.split_whitespace();

    let result = parse_request_line(parts)?;

    Ok(result)

 }

fn parse_request_line(mut parts: SplitWhitespace<'_>) -> Result<Request, io::Error> {
    let method = parts
    .next()
    .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Method missing"))?
    .to_string();

    let request_target = parts
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Request target missing"))?
        .to_string();

    let http_version = parts
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Http version missing"))?
        .to_string();

    let request_line = RequestLine {
        http_version,
        request_target,
        method,
    };

    let request = Request { request_line };

    Ok(request)
} 