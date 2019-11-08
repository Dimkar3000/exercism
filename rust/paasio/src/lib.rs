use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    wrapped: R,
    operations: usize,
    bytes_read: usize,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            wrapped,
            operations: 0,
            bytes_read: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_read
    }

    pub fn reads(&self) -> usize {
        self.operations
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        match self.wrapped.read(buf) {
            Ok(x) => {
                self.bytes_read += x;
                self.operations += 1;
                Ok(x)
            }
            Err(e) => {
                self.operations += 1;
                Err(e)
            }
        }
    }
}

pub struct WriteStats<W> {
    wrapped: W,
    operations: usize,
    bytes_writen: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            wrapped,
            operations: 0,
            bytes_writen: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_writen
    }

    pub fn writes(&self) -> usize {
        self.operations
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        match self.wrapped.write(buf) {
            Ok(x) => {
                self.bytes_writen += x;
                self.operations += 1;
                Ok(x)
            }
            Err(e) => {
                self.operations += 1;
                Err(e)
            }
        }
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}
