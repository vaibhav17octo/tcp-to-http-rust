use std::io::{self, Read};
pub struct SlowReader {
    pub data: Vec<u8>,
    pub pos: usize,
    pub read_size: usize,
}

impl Read for SlowReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.pos >= self.data.len() {
            return Ok(0); // Reached EOL
        }

        if self.read_size > self.data.len() - self.pos {
            self.read_size = self.data.len() - self.pos;
        }

        let idx = self.pos + self.read_size;
        buf[..self.read_size].copy_from_slice(&self.data[self.pos..idx]);

        self.pos += self.read_size;

        Ok(self.read_size)
    }
}
