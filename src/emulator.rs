use crate::cpu::CPU;
use crate::framebuffer::Framebuffer;
use crate::mmu::MMU;
use crate::rom::ROM;

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
        return _emulator;
    }

    pub fn start(&mut self) {
        self.running = true;
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn cycle(&mut self) {
        while self.running {
            self.cpu.cycle(&mut self.mmu);
        }
    }

    fn load_rom_into_memory(&mut self) {
        self.mmu.write_to_mem(self.rom.get_data(), 0x200)
            .expect("Tried to load ROM outside of memory block");
    }

    pub fn get_fb_data(&self) -> Vec<u8> {
        return self.mmu.fb_get_data();
    }

    pub fn fb_needs_refresh(&self) -> bool {
        return self.mmu.fb_needs_refresh();
    }
}