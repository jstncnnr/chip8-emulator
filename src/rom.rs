use std::fs::File;
use std::io;
use std::io::Read;

pub struct ROM {
    pub path: &'static str,
    pub size: usize,
    data: Vec<u8>
}

impl ROM {
    pub fn from_file(path: &'static str) -> io::Result<Self> {
        let mut file = File::open(path)?;
        let mut buf = Vec::new();
        let size = file.read_to_end(&mut buf)?;

        Ok(Self {
            path,
            size,
            data: buf.clone()
        })
    }

    pub fn get_data(&self) -> Vec<u8> {
        return self.data.clone();
    }
}