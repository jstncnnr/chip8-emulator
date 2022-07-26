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
        let v_x = ((opcode & 0xF00) >> 8);
        let v_y = ((opcode & 0x00F0) >> 4);
        let nnn = opcode & 0x0FFF;
        let nn = (opcode & 0x00FF) as u8;

        match instruction {
            _ => {
                panic!("Unhandled instruction type: {:X}", instruction);
            }
        }
    }
}