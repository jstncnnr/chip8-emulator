use std::io;
use crate::mmu::MMU;

pub struct CPU {
    pc: u16,
    sp: u16,
    i: u16,
    stack: Vec<u16>,
    registers: [u8; 16]
}

impl CPU {
    pub fn new(program_start: u16) -> Self {
        Self {
            pc: program_start,
            sp: 0,
            i: 0,
            stack: vec![0; 16],
            registers: [0; 16]
        }
    }

    pub fn cycle(&mut self, mmu: &mut MMU) {
        let opcode = mmu.read_short(self.pc as usize).expect("Could not read opcode from memory");
        self.pc += 2;

        let instruction = (opcode & 0xF000) >> 12;
        let v_x = (opcode & 0xF00) >> 8;
        let v_y = (opcode & 0x00F0) >> 4;
        let nnn = opcode & 0x0FFF;
        let nn = (opcode & 0x00FF) as u8;
        let n = (opcode & 0x000F) as u8;

        match instruction {
            0x0 => {
                self.handle_ins_0(opcode, nn);
            }
            0x1 => {
                self.handle_ins_1(nnn);
            }
            0x6 => {
                self.handle_ins_6(v_x, nn);
            }
            0x7 => {
                self.handle_ins_7(v_x, nn);
            }
            0xA => {
                self.handle_ins_a(nnn);
            }
            0xD => {
                self.handle_ins_d(n, v_x, v_y, mmu);
            }
            _ => {
                panic!("Unhandled instruction type: {:X}", instruction);
            }
        }
    }

    fn handle_ins_0(&mut self, opcode: u16, nn: u8) {
        match nn {
            0xE0 => {
                // Clear the framebuffer
            }
            0xEE => {
                self.pc = self.stack[self.sp as usize];
                self.sp -= 1;
            }
            _ => { panic!("Unexpected opcode: {:X}", opcode) }
        }
    }

    fn handle_ins_1(&mut self, nnn: u16) {
        self.pc = nnn;
    }

    fn handle_ins_6(&mut self, v_x: u16, nn: u8) {
        self.registers[v_x as usize] = nn;
    }

    fn handle_ins_7(&mut self, v_x: u16, nn: u8) {
        self.registers[v_x as usize] += nn;
    }

    fn handle_ins_a(&mut self, nnn: u16) {
        self.i = nnn;
    }

    fn handle_ins_d(&mut self, n: u8, v_x: u16, v_y: u16, mmu: &mut MMU) {
        self.registers[0xF] = 0;
        for row in 0..n {
            let y = (self.registers[v_y as usize] + row) % 32;
            for col in 0..8 {
                let x = (self.registers[v_x as usize] + col) % 64;
                let mem_value = mmu.read((self.i + row as u16) as usize);
                let mem_value = match mem_value {
                    Ok(value) => value,
                    Err(..) => return
                };

                let color = mem_value >> (7 - col) & 1;
                let current = mmu.fb_get(x as usize, y as usize);
                self.registers[0xF] |= color & current;
                mmu.fb_set(x as usize, y as usize, current ^ color);
            }
        }
    }
}