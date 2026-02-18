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

        let end_index = (self.pos + self.num_bytes_per_read).min(self.data.len());

        let bytes = self.data.as_bytes();
        let chunk =   &bytes[self.pos..end_index];

        let n = chunk.len().min(buf.len());
        buf[..n].copy_from_slice(&chunk[..n]);

        self.pos += n;

        Ok(n)
    }
}