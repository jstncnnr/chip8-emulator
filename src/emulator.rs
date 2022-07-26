use crate::mmu::MMU;
use crate::rom::ROM;

pub struct Emulator<'a> {
    rom: &'a ROM,
    mmu: MMU,
    running: bool
}

impl<'a> Emulator<'a> {
    pub fn new(rom: &'a ROM) -> Self {
        let mut _emulator = Self {
            rom,
            mmu: MMU::new(4096),
            running: false
        };

        _emulator.load_rom_into_memory();
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

    fn load_rom_into_memory(&mut self) {
        self.mmu.write_to_mem(self.rom.get_data(), 0x200)
            .expect("Tried to load ROM outside of memory block");
    }
}