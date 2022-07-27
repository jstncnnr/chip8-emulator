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
                self.handle_ins_0(opcode, nn, mmu);
            }
            0x1 => {
                self.handle_ins_1(nnn);
            }
            0x2 => {
                self.handle_ins_2(nnn);
            }
            0x3 => {
                self.handle_ins_3(v_x, nn);
            }
            0x4 => {
                self.handle_ins_4(v_x, nn);
            }
            0x5 => {
                self.handle_ins_5(v_x, v_y);
            }
            0x6 => {
                self.handle_ins_6(v_x, nn);
            }
            0x7 => {
                self.handle_ins_7(v_x, nn);
            }
            0x8 => {
                self.handle_ins_8(v_x, v_y, n);
            }
            0x9 => {
                self.handle_ins_9(v_x, v_y);
            }
            0xA => {
                self.handle_ins_a(nnn);
            }
            0xD => {
                self.handle_ins_d(n, v_x, v_y, mmu);
            }
            0xF => {
                self.handle_ins_f(v_x, nn, mmu);
            }
            _ => {
                panic!("Unhandled instruction type: {:X}", instruction);
            }
        }
    }

    fn handle_ins_0(&mut self, opcode: u16, nn: u8, mmu: &mut MMU) {
        match nn {
            0xE0 => {
                mmu.fb_clear();
            }
            0xEE => {
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
            }
            _ => { panic!("Unexpected opcode: {:X}", opcode) }
        }
    }

    fn handle_ins_1(&mut self, nnn: u16) {
        self.pc = nnn;
    }

    fn handle_ins_2(&mut self, nnn: u16) {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = nnn;
    }

    fn handle_ins_3(&mut self, v_x: u16, nn: u8) {
        if self.registers[v_x as usize] == nn {
            self.pc += 2;
        }
    }

    fn handle_ins_4(&mut self, v_x: u16, nn: u8) {
        if self.registers[v_x as usize] != nn {
            self.pc += 2;
        }
    }

    fn handle_ins_5(&mut self, v_x: u16, v_y: u16) {
        if self.registers[v_x as usize] == self.registers[v_y as usize] {
            self.pc += 2;
        }
    }

    fn handle_ins_6(&mut self, v_x: u16, nn: u8) {
        self.registers[v_x as usize] = nn;
    }

    fn handle_ins_7(&mut self, v_x: u16, nn: u8) {
        self.registers[v_x as usize] = self.registers[v_x as usize].wrapping_add(nn);
    }

    fn handle_ins_8(&mut self, v_x: u16, v_y: u16, n: u8) {
        let v_x = v_x as usize;
        let v_y = v_y as usize;

        match n {
            0x0 => {
                self.registers[v_x] = self.registers[v_y];
            }
            0x1 => {
                self.registers[v_x] |= self.registers[v_y];
            }
            0x2 => {
                self.registers[v_x] &= self.registers[v_y];
            }
            0x3 => {
                self.registers[v_x] ^= self.registers[v_y];
            }
            0x4 => {
                let (result, overflow) = self.registers[v_x].overflowing_add(self.registers[v_y]);
                self.registers[v_x] = result;
                self.registers[0xF] = overflow as u8;
            }
            0x6 => {
                self.registers[0xF] = self.registers[v_x] & 1;
                self.registers[v_x] >>= 1;
            }
            0x5 => {
                self.registers[0xF] = if self.registers[v_x] > self.registers[v_y] { 1 } else { 0 };
                self.registers[v_x] = self.registers[v_x].wrapping_sub(self.registers[v_y]);
            }
            0x7 => {
                self.registers[0xF] = if self.registers[v_y] > self.registers[v_x] { 1 } else { 0 };
                self.registers[v_x] = self.registers[v_y].wrapping_sub(self.registers[v_x]);
            }
            0xE => {
                self.registers[0xF] = (self.registers[v_x] & 0xFF) >> 7;
                self.registers[v_x] <<= 1;
            }
            _ => { panic!("Unexpected operation 8XY{:X}", n) }
        }
    }

    fn handle_ins_9(&mut self, v_x: u16, v_y: u16) {
        if self.registers[v_x as usize] != self.registers[v_y as usize] {
            self.pc += 2;
        }
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

    fn handle_ins_f(&mut self, v_x: u16, nn: u8, mmu: &mut MMU) {
        let v_x = v_x as usize;
        match nn {
            0x33 => {
                mmu.write(self.i as usize, self.registers[v_x] / 100).expect("Unable to write to memory");
                mmu.write((self.i + 1) as usize, (self.registers[v_x] % 100) / 10).expect("Unable to write to memory");
                mmu.write((self.i + 2) as usize, self.registers[v_x] % 10).expect("Unable to write to memory");
            }
            0x55 => {
                mmu.write_to_mem(self.registers[0..(v_x + 1)].to_vec(), self.i as usize)
                    .expect("Unable to write to memory");
            }
            0x65 => {
                for i in 0..v_x + 1 {
                    self.registers[i] = mmu.read((self.i as usize) + i).expect("Unable to read memory");
                }
            }
            _ => { panic!("Unexpected opcode FX{:X}", nn) }
        }
    }
}