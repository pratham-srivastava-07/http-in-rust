use std::{io::{self, Read}, str::SplitWhitespace};

use crate::internal::requests::parse_stream::ChunkReader;

pub struct RequestLine {
    http_version: String,
    request_target: String,
    method: String
}

pub struct Request {
    pub request_line: RequestLine,
    state: ParseState
}

enum ParseState {
    Initialized,
    Done
}


impl Request {
    fn parse(&mut self, data: &[u8]) -> Result<usize,io::Error> {
        match self.state  {
            ParseState::Initialized => {
                let (request_line, bytes) = parse_request_line(&String::from_utf8_lossy(data))?;
               if bytes == 0 {
                    return Ok(0)
               } else {
                    self.request_line = request_line;
                    self.state = ParseState::Done;
                    return Ok(bytes);
               }
            },
            ParseState::Done => {
                return Ok(0);
            }
        }
    }
}

 pub fn request_from_reader(mut reader: impl Read) -> Result<Request, io::Error> {
    // first create a simple buffer
    let mut buffer = [0; 1024];

    // then read all bytes 
    let bytes_to_be_read = reader.read(&mut buffer)?;
    println!("{:?}", bytes_to_be_read);

    let request_string = String::from_utf8_lossy(&buffer[..bytes_to_be_read]);

    let mut request = Request {
        request_line: RequestLine { http_version: String::new(), request_target: String::new(), method: String::new() },
        state: ParseState::Initialized
    };

    let mut chunk_reader = ChunkReader::new(request_string.to_string(), 5);

    let mut read_into_buf: Vec<u8> = Vec::new();

    loop {
        let chunks = chunk_reader.read(&mut buffer)?;

        if chunks == 0 {
            break;
        }

        read_into_buf.extend_from_slice(&buffer[..chunks]);

        let consumed = request.parse(&read_into_buf)?;

        read_into_buf.drain(..consumed);

        if matches!(request.state, ParseState::Done) {
            break;
        }

    };

    Ok(request)
 }

fn parse_request_line(input: &str) -> Result<(RequestLine, usize), io::Error> {

    let line_end = match input.find("\r\n") {  // "\r\n"
        Some(index) => index,
        None => {
            return Ok((  
                    RequestLine 
                    {
                        http_version: String::new(),
                        request_target: String::new(), 
                        method: String::new() 
                    },
                0,
            ));
        }
    };

    let request_line_string = &input[..line_end];

    let mut parts = request_line_string.split_whitespace();

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

    // let request = Request { request_line, state};
    let bytes_consumed = line_end + 2; // for \r\n inclusive 

    Ok((request_line, bytes_consumed))
} 