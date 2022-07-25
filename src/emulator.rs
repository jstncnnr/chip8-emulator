use crate::rom::ROM;

pub struct Emulator<'a> {
    rom: &'a ROM,
    running: bool
}

impl<'a> Emulator<'a> {
    pub fn new(rom: &ROM) -> Self {
        let mut _emulator = Self {
            rom,
            running: false
        };

        return _emulator;
    }

    pub fn start(&mut self) {
        self.running = true;
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn cycle(&mut self) {

    }
}