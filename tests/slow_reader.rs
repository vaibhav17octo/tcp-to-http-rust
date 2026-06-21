use std::io::{self};
use tokio::io::AsyncRead;
pub struct SlowReader {
    pub data: Vec<u8>,
    pub pos: usize,
    pub read_size: usize,
}

impl AsyncRead for SlowReader {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<io::Result<()>> {
        if self.pos >= self.data.len() {
            return std::task::Poll::Ready(Ok(())); // Reached EOL
        }

        if self.read_size > self.data.len() - self.pos {
            self.read_size = self.data.len() - self.pos;
        }

        let idx = self.pos + self.read_size;
        buf.put_slice(&self.data[self.pos..idx]);

        self.pos += self.read_size;

        std::task::Poll::Ready(Ok(()))
    }
}
