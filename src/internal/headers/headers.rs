use std::{collections::HashMap, io::{self, Read}};

#[derive(Debug)]
pub struct Headers {
    map: HashMap<String, String>
}

impl Headers {
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.map.get(key)
    }

    pub fn parse(&mut self, data: &[u8]) -> Result<(usize, bool), io::Error> {
        let text = String::from_utf8_lossy(data);

        let mut bytes_consumed = 0;

        for line in text.split("\r\n") {
            bytes_consumed += line.len() + 2;

            if line.is_empty() {
                return Ok((bytes_consumed, false));
            }

             if line.starts_with(' ') {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid header format",
                ));
            }

            let (key, value) = line
                .split_once(": ")
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidData, "Malformed header line")
                })?;

            self.map.insert(key.to_string(), value.to_string());
        }
        Ok((bytes_consumed, true))
    }
}
