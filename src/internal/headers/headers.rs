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

        let key = line_str[..colon_pos].to_lowercase();

        if !crate::utils::is_valid_token(&key) {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid characters in key"));
        }

        if key.ends_with(' ') {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid spaces in key"));
        }

        let value = line_str[colon_pos + 1..].trim().to_string();

        self.map
            .entry(key)
            .and_modify(|v| {
                v.push_str(", ");
                v.push_str(&value);
            })
            .or_insert(value);

        Ok((crlf_pos + 2, true))
    }
}