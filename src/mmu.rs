use std::io;
use std::io::{Error, ErrorKind};
use crate::framebuffer::Framebuffer;

pub struct MMU {
    memory: Vec<u8>,
    max_size: usize,
    framebuffer: Framebuffer
}

impl MMU {
    pub fn new(max_size: usize) -> Self {
        Self {
            memory: vec![0; max_size],
            max_size,
            framebuffer: Framebuffer::new(64, 32)
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

    pub fn read_short(&self, address: usize) -> io::Result<u16> {
        if address + 1 >= self.max_size {
            return Err(Error::new(ErrorKind::Other, "Index out of bounds"));
        }

        let short = (self.read(address)? as u16) << 8 | self.read(address + 1)? as u16;
        Ok(short)
    }

    pub fn fb_get(&self, x: usize, y: usize) -> u8 {
        return self.framebuffer.get(x, y);
    }

    pub fn fb_set(&mut self, x: usize, y: usize, val: u8) {
        self.framebuffer.set(x, y, val);
    }

    pub fn fb_get_data(&self) -> Vec<u8> {
        return self.framebuffer.get_data();
    }

    pub fn fb_needs_refresh(&self) -> bool {
        return self.framebuffer.needs_refresh;
    }

    pub fn fb_clear(&mut self) {
        self.framebuffer.clear();
    }
}