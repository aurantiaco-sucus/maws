use std::io;
use std::io::Read;

pub struct StreamBuffer<R: Read> {
    pub inner: R,
    pub buf: Vec<u8>,
    pub len: usize,
    pub len_old: usize,
}

impl <R: Read> StreamBuffer<R> {
    pub fn new(inner: R, cap_initial: usize) -> Self {
        Self { inner, buf: vec![0; cap_initial], len: 0, len_old: 0 }
    }

    /// Reads from stream to scratch buffer. It doubles the size of the buffer if needed.
    pub fn read(&mut self) -> io::Result<()> {
        if self.buf.len() == self.len {
            self.buf.resize(self.buf.len() * 2, 0);
        }
        let len_add = self.inner.read(&mut self.buf[self.len..])?;
        self.len_old = self.len;
        self.len += len_add;
        Ok(())
    }

    /// Meaningful slice of buffer.
    pub fn buf_eff(&self) -> &[u8] {
        &self.buf[..self.len]
    }

    /// Drops the earliest `n` effective elements from the buffer, best used for session separation.
    /// This also affects the `len_old` field.
    pub fn drop_earliest(&mut self, n: usize) {
        assert!(self.len >= n);
        self.buf.copy_within(n..self.len, 0);
        self.len -= n;
        self.len_old = self.len_old.saturating_sub(n);
    }

    /// Resize the buffer to at most `f` times the current effective size.
    pub fn fit_factor(&mut self, f: usize) {
        if self.len * f < self.buf.len() {
            return;
        }
        self.buf.resize(self.len * f, 0);
    }
}