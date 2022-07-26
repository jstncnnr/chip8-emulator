use std::io;
use std::io::{Error, ErrorKind};

pub struct MMU {
    memory: Vec<u8>,
    max_size: usize
}

impl MMU {
    pub fn new(max_size: usize) -> Self {
        Self {
            memory: vec![0; max_size],
            max_size
        }
    }

    pub fn write_to_mem(&mut self, buf: Vec<u8>, offset: usize) -> io::Result<usize> {
        if offset >= self.max_size {
            return Err(Error::new(ErrorKind::Other, "Index out of bounds"));
        }

        let mut bytes_written: usize = 0;
        for i in 0..buf.len() {
            if offset + i > self.max_size {
                continue;
            }

            self.memory[offset + i] = buf[i];
            bytes_written += 1;
        }

        Ok(bytes_written)
    }

    pub fn read(&self, address: usize) -> io::Result<u8> {
        if address >= self.max_size {
            return Err(Error::new(ErrorKind::Other, "Index out of bounds"));
        }

        Ok(self.memory[address])
    }
}