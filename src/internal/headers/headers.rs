use std::{collections::HashMap, io};

pub struct Headers {
    pub map: HashMap<String, String>
}

impl Headers {
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.map.get(key)
    }

    pub fn parse(&mut self, data: &[u8]) -> Result<(usize, bool), io::Error> {
        let position = data.windows(2).position(|w| w==b"\r\n");

        let Some(crlf_pos) = position else {
            return Ok((0, false));
        };

        if crlf_pos == 0 {
            return Ok((2, false));
        }

        let line = &data[..crlf_pos];
        let line_str = std::str::from_utf8(line).map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Line not converted"))?;

        if line_str.starts_with(' ') {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid spaces"));
        }

        let colon_pos = line_str.find(":").ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "colon missing"))?;

        let key = &line_str[..colon_pos].to_lowercase();

        // TODO: keep a check for invalid character chk and that it should have atleast 1 char

        

        if key.ends_with(' ') {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid spaces in key"));
        }

        let value = &line_str[colon_pos+1..];

        // TODO for multiple headers with same key, keep a chk and then append values if it does exist
        // if self.map.contains_key(key) {
        //     self.map.insert(k, v)
        // }

        self.map.insert(key.to_string(), value.to_string());

        Ok((colon_pos +2, true))
    }
}