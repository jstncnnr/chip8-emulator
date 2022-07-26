
pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub needs_refresh: bool,
    data: Vec<u8>
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            needs_refresh: false,
            data: vec![0; width * height]
        }
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.needs_refresh = true;
    }

    pub fn set(&mut self, x: usize, y: usize, val: u8) {
        self.data[y * self.width + x] = val;
        self.needs_refresh = true;
    }

    pub fn get(&self, x: usize, y: usize) -> u8 {
        return self.data[y * self.width + x];
    }

    pub fn get_data(&self) -> Vec<u8> {
        return self.data.clone();
    }
}