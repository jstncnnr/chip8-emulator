use crate::cpu::CPU;
use crate::mmu::MMU;
use crate::rom::ROM;

const FONT_DATA: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub struct Emulator<'a> {
    rom: &'a ROM,
    mmu: MMU,
    cpu: CPU,
    running: bool
}

impl<'a> Emulator<'a> {
    pub fn new(rom: &'a ROM) -> Self {
        let mut _emulator = Self {
            rom,
            mmu: MMU::new(4096),
            cpu: CPU::new(0x200),
            running: false
        };

        _emulator.load_rom_into_memory();
        _emulator.load_font_map_into_memory();
        return _emulator;
    }

    pub fn start(&mut self) {
        self.running = true;
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn cycle(&mut self) {
        if self.running {
            self.cpu.cycle(&mut self.mmu);
        }
    }

    fn load_rom_into_memory(&mut self) {
        self.mmu.write_to_mem(self.rom.get_data(), 0x200)
            .expect("Tried to load ROM outside of memory block");
    }

    fn load_font_map_into_memory(&mut self) {
        self.mmu.write_to_mem(FONT_DATA.to_vec(), 0x50)
            .expect("Tried to load font map outside of memory block");
    }

    pub fn get_fb_data(&self) -> Vec<u8> {
        return self.mmu.fb_get_data();
    }

    pub fn fb_needs_refresh(&self) -> bool {
        return self.mmu.fb_needs_refresh();
    }
}