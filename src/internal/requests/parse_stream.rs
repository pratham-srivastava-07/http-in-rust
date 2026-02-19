use std::io::{self, Read};

struct ChunkReader {
    data: String,
    num_bytes_per_read: usize,
    pos: usize
}

impl ChunkReader {
    fn new(data: String, num_bytes_per_read: usize) -> Self {
        Self { data, num_bytes_per_read, pos: 0 }
    }
}

impl Read for ChunkReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.pos >= self.data.len() {
            return Ok(0);
        }
        // if pos = 3, num_bytes_per_read = 4, data.len() = 30 => min == 7
        let end_index = (self.pos + self.num_bytes_per_read).min(self.data.len());

        // 30
        let bytes = self.data.as_bytes();
        // chunk is from index 3 to 7 
        let chunk =   &bytes[self.pos..end_index];

        // chunk.len() = 4, buf.len() = 2 (lets say), n value is 2
        let n = chunk.len().min(buf.len());
        // copy chunk's size upto n into buf
        buf[..n].copy_from_slice(&chunk[..n]);

        self.pos += n;

        Ok(n)
    }
}