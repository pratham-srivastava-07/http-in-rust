use std::{io::{self, Read}, fmt};

use crate::internal::headers::headers::Headers;

#[derive(Debug)]
pub struct RequestLine {
    http_version: String,
    request_target: String,
    method: String
}

impl fmt::Display for RequestLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Request line:\n- Method: {}\n- Target: {}\n- Version: {}",
            self.method, self.request_target, self.http_version
        )
    }
}

#[derive(Debug)]
pub struct Request {
    pub request_line: RequestLine,
    pub headers: Headers,
    state: ParseState
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.request_line)?;
        writeln!(f, "Headers:")?;
        for (key, value) in &self.headers.map {
            // Capitalize the first letter of each part of the key for display purposes
            // (e.g. user-agent -> User-Agent), or just display exactly as uppercase key 
            writeln!(f, "- {}: {}", key.to_uppercase(), value)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
enum ParseState {
    Initialized,
    ParsingHeader,
    Done
}


impl Request {
    fn parse(&mut self, data: &[u8]) -> Result<usize,io::Error> {
        match self.state  {
            ParseState::Initialized => {
                let (request_line, bytes) = parse_request_line(&String::from_utf8_lossy(data))?;
                println!("{:?}, {}", request_line, bytes);
               if bytes == 0 {
                    return Ok(0)
               } else {
                    self.request_line = request_line; 
                    self.state = ParseState::ParsingHeader;
                    return Ok(bytes);
               }
            },
            ParseState::ParsingHeader => {
                let (pos, val) = self.headers.parse(data)?;
                if val == true {
                    return Ok(pos);
                } else {
                    if pos > 0 {
                        self.state = ParseState::Done;
                    }
                    return Ok(pos);
                }
            }
            ParseState::Done => {
                return Ok(0);
            }
        }
    }
}

 pub fn request_from_reader(mut reader: impl Read) -> Result<Request, io::Error> {
    // first create a simple buffer
    let mut buffer = [0; 1024];

    let mut request = Request {
        request_line: RequestLine { http_version: String::new(), request_target: String::new(), method: String::new() },
        headers: Headers::new(),
        state: ParseState::Initialized
    };

    // println!("{}", request.request_line);

    let mut read_into_buf: Vec<u8> = Vec::new();

    loop {
        let chunks = reader.read(&mut buffer)?;  
        println!("{}", chunks);

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

    // println!("{}, {}, {}", http_version.clone(), method.clone(), request_target.clone());

    // let request = Request { request_line, state};
    let bytes_consumed = line_end + 2; // for \r\n inclusive 

    Ok((request_line, bytes_consumed))
}