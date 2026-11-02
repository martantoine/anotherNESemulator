use super::{Instruction, AddressingMode, Flags};

pub struct OpCode {
    pub code: u8,
    pub instruction: Instruction,
    pub addressing_mode: AddressingMode,
    pub nbyte: u8,
    pub ncycle: u8,
}

impl OpCode {
    pub fn new(code: u8, instruction: Instruction, addressing_mode: AddressingMode, nbyte: u8, ncycle: u8) -> Self {
        OpCode {
            code: code,
            instruction: instruction,
            addressing_mode: addressing_mode,
            nbyte: nbyte,
            ncycle: ncycle,
        }
    }
}

impl super::cpu {
    pub fn update_ZN(&mut self, value: i8) {
        self.status.set(Flags::Z, value == 0);
        self.status.set(Flags::N, value < 0);
    }
    pub fn LDA(&mut self, mode: AddressingMode) {
        let tmp: i8 = self.memory[self.get_operand_address(mode) as usize] as i8;
        self.accumulator = tmp.clone();
        self.update_ZN(tmp);
    }

    pub fn LDX(&mut self, mode: AddressingMode) {
        let tmp: i8 = self.memory[self.get_operand_address(mode) as usize] as i8;
        self.register_x = tmp.clone();
        self.update_ZN(tmp);
    }

    pub fn LDY(&mut self, mode: AddressingMode) {
        let tmp: i8 = self.memory[self.get_operand_address(mode) as usize] as i8;
        self.register_y = tmp.clone();
        self.update_ZN(tmp);
    }

    pub fn STA(&mut self, mode: AddressingMode) {
        self.memory[self.get_operand_address(mode) as usize] = self.accumulator as u8;
    }

    pub fn STX(&mut self, mode: AddressingMode) {
        self.memory[self.get_operand_address(mode) as usize] = self.register_x as u8;
    }

    pub fn STY(&mut self, mode: AddressingMode) {
        self.memory[self.get_operand_address(mode) as usize] = self.register_y as u8;
    }

    pub fn TAX(&mut self) {
        self.register_x = self.accumulator;
        self.update_ZN(self.register_x);
    }

    pub fn TAY(&mut self) {
        self.register_y = self.accumulator;
        self.update_ZN(self.register_y);
    }

    pub fn TXA(&mut self) {
        self.accumulator = self.register_x;
        self.update_ZN(self.accumulator);
    }
    
    pub fn TYA(&mut self) {
        self.accumulator = self.register_y;
        self.update_ZN(self.accumulator);
    }
    
    pub fn TSX(&mut self) {
        self.register_x = self.stack_pointer as i8;
        self.update_ZN(self.register_x);
    }
    
    pub fn TXS(&mut self) {
        self.stack_pointer = self.register_x as u8;
    }
    
    pub fn PHA(&mut self) {
        self.memory[(self.stack_pointer as u16 + 0x0100) as usize] = self.accumulator as u8;
        self.stack_pointer -= if self.stack_pointer > 0 { 1 } else { 0 };
    }
    
    pub fn PHP(&mut self) {
        self.memory[(self.stack_pointer as u16 + 0x0100) as usize] = self.status.bits();
        self.stack_pointer -= if self.stack_pointer > 0 { 1 } else { 0 };
    }
    
    pub fn PLA(&mut self) {
        self.stack_pointer += if self.stack_pointer < 0xFF { 1 } else { 0 };
        self.accumulator = self.memory[(self.stack_pointer as u16 + 0x0100) as usize] as i8;
        self.update_ZN(self.accumulator);
    }

    pub fn PLP(&mut self) {
        self.stack_pointer += if self.stack_pointer < 0xFF { 1 } else { 0 };
        self.status = Flags::from_bits(self.memory[(self.stack_pointer as u16 + 0x0100) as usize]).unwrap();
    }

    pub fn AND(&mut self, mode: AddressingMode) {
        self.accumulator = (self.accumulator as u8 & self.memory[self.get_operand_address(mode) as usize]) as i8;
        self.update_ZN(self.accumulator);
    }

    pub fn EOR(&mut self, mode: AddressingMode) {
        self.accumulator = (self.accumulator as u8 ^ self.memory[self.get_operand_address(mode) as usize]) as i8;
        self.update_ZN(self.accumulator);
    }

    pub fn ORA(&mut self, mode: AddressingMode) {
        self.accumulator = (self.accumulator as u8 | self.memory[self.get_operand_address(mode) as usize]) as i8;
        self.update_ZN(self.accumulator);
    }

    pub fn BIT(&mut self, mode: AddressingMode) {
        let tmp = self.accumulator as u8 & self.memory[self.get_operand_address(mode) as usize];
        self.status.set(Flags::Z, tmp == 0);
        self.status.set(Flags::V, (1 << 6) == (tmp & (1 << 6)));
        self.status.set(Flags::N, (1 << 7) == (tmp & (1 << 7)));
    }

    pub fn ADC(&mut self, mode: AddressingMode) {
        let C: u16 = if self.status.contains(Flags::C) { 1 } else { 0 };
        let M: u16 = self.memory[self.get_operand_address(mode) as usize] as u16;
        let N: u16 = (self.accumulator as u8) as u16;
        let S: u16 = M + N + C;
        
        self.status.set(Flags::C, S > 0xFF);
        self.status.set(Flags::V, ((M^S)&(N^S)&0x80) != 0);
        self.accumulator = ((S & 0xFF) as u8) as i8;
        self.update_ZN(self.accumulator);
    }
    
    pub fn SBC(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode.clone()) as usize;
        self.memory[addr] = !self.memory[addr];
        self.ADC(mode);
    }

    pub fn compare(&mut self, mode: AddressingMode, target: i8) {
        let M = self.memory[self.get_operand_address(mode) as usize] as u8;
        self.status.set(Flags::C, (target as u8) >= M); 
        self.status.set(Flags::Z, (target as u8) == M); 
        self.status.set(Flags::N, ((target as u8).wrapping_add(!M).wrapping_add(1)) & (1 << 7) != 0);
    }

    pub fn CMP(&mut self, mode: AddressingMode) {
        self.compare(mode, self.accumulator);
    }

    pub fn CPX(&mut self, mode: AddressingMode) {
        self.compare(mode, self.register_x);
    }

    pub fn CPY(&mut self, mode: AddressingMode) {
        self.compare(mode, self.register_y);
    }

    pub fn INC(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode) as usize;
        self.memory[addr] = self.memory[addr].wrapping_add(1);
        self.update_ZN(self.memory[addr] as i8);
    }

    pub fn INX(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_ZN(self.register_x);
    }

    pub fn INY(&mut self) {
        self.register_y = self.register_y.wrapping_add(1);
        self.update_ZN(self.register_y);
    }

    pub fn DEC(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode) as usize;
        self.memory[addr] = self.memory[addr].wrapping_sub(1);
        self.update_ZN(self.memory[addr] as i8);
    }

    pub fn DEX(&mut self) {
        self.register_x = self.register_x.wrapping_sub(1);
        self.update_ZN(self.register_x);
    }

    pub fn DEY(&mut self) {
        self.register_y = self.register_y.wrapping_sub(1);
        self.update_ZN(self.register_y);
    }
    
    pub fn ASL(&mut self, mode: AddressingMode) {
        if mode != AddressingMode::Accumulator {
            let addr = self.get_operand_address(mode) as usize;
            self.status.set(Flags::C, self.memory[addr] & (1 << 7) != 0);
            self.memory[addr] = self.memory[addr].wrapping_shl(1);
            self.update_ZN(self.memory[addr] as i8);
        } else  {
            self.status.set(Flags::C, self.accumulator & (1 << 7) != 0);
            self.accumulator = self.accumulator.wrapping_shl(1);
            self.update_ZN(self.accumulator as i8);
        }
    }
    
    pub fn LSR(&mut self, mode: AddressingMode) {
        if mode != AddressingMode::Accumulator {
            let addr = self.get_operand_address(mode) as usize;
            self.status.set(Flags::C, self.memory[addr] & 1 != 0);
            self.memory[addr] = self.memory[addr].wrapping_shr(1);
            self.update_ZN(self.memory[addr] as i8);
        } else  {
            self.status.set(Flags::C, self.accumulator & 1 != 0);
            self.accumulator = self.accumulator.wrapping_shr(1);
            self.update_ZN(self.accumulator as i8);
        }
    }
    
    pub fn ROL(&mut self, mode: AddressingMode) {
        if mode != AddressingMode::Accumulator {
            let addr = self.get_operand_address(mode) as usize;
            self.status.set(Flags::C, self.memory[addr] & (1 << 7) != 0);
            self.memory[addr] = self.memory[addr].rotate_left(1);
            self.update_ZN(self.memory[addr] as i8);
        } else  {
            self.status.set(Flags::C, self.accumulator & (1 << 7) != 0);
            self.accumulator = self.accumulator.rotate_left(1);
            self.update_ZN(self.accumulator as i8);
        }
    }
    
    pub fn ROR(&mut self, mode: AddressingMode) {
        if mode != AddressingMode::Accumulator {
            let addr = self.get_operand_address(mode) as usize;
            self.status.set(Flags::C, self.memory[addr] & 1 != 0);
            self.memory[addr] = self.memory[addr].rotate_right(1);
            self.update_ZN(self.memory[addr] as i8);
        } else  {
            self.status.set(Flags::C, self.accumulator & 1 != 0);
            self.accumulator = self.accumulator.rotate_right(1);
            self.update_ZN(self.accumulator as i8);            
        }
    }

    pub fn JMP(&mut self, mode: AddressingMode) {
        self.program_counter = self.get_operand_address(mode);
    }

    pub fn JSR(&mut self, mode: AddressingMode) {
        let return_address = self.program_counter;//.wrapping_sub(1);
        let l = (return_address & 0xFF) as u8;
        let h = (return_address.wrapping_shr(8) & 0xFF) as u8;
        self.memory[(self.stack_pointer as u16 + 0x0100) as usize] = h;
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        self.memory[(self.stack_pointer as u16 + 0x0100) as usize] = l;
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        
        self.program_counter = self.get_operand_address(mode);
    }
    
    pub fn RTS(&mut self) {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        let l = self.memory[(self.stack_pointer as u16 + 0x0100) as usize];
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        let h = self.memory[(self.stack_pointer as u16 + 0x0100) as usize];
        
        self.program_counter = (h as u16).wrapping_shl(8) | (l as u16);
    }

    pub fn jmp_relative(&mut self, mode: AddressingMode) {
        let r = ((self.get_operand_address(mode) & 0xFF) as i8) as u16;
        self.program_counter = self.program_counter.wrapping_add(r);
    }
    
    pub fn BCC(&mut self, mode: AddressingMode) {
        if !self.status.contains(Flags::C) {
            self.jmp_relative(mode);
        }
    }

    pub fn BCS(&mut self, mode: AddressingMode) {
        if self.status.contains(Flags::C) {
            self.jmp_relative(mode);
        }
    }

    pub fn BEQ(&mut self, mode: AddressingMode) {
        if self.status.contains(Flags::Z) {
            self.jmp_relative(mode);
        }
    }

    pub fn BMI(&mut self, mode: AddressingMode) {
        if self.status.contains(Flags::N) {
            self.jmp_relative(mode);
        }
    }

    pub fn BNE(&mut self, mode: AddressingMode) {
        if !self.status.contains(Flags::Z) {
            self.jmp_relative(mode);
        }
    }

    pub fn BPL(&mut self, mode: AddressingMode) {
        if !self.status.contains(Flags::N) {
            self.jmp_relative(mode);
        }
    }
    
    pub fn BVC(&mut self, mode: AddressingMode) {
        if !self.status.contains(Flags::V) {
            self.jmp_relative(mode);
        }
    }
    
    pub fn BVS(&mut self, mode: AddressingMode) {
        if self.status.contains(Flags::V) {
            self.jmp_relative(mode);
        }
    }
    
    pub fn CLC(&mut self) {
        self.status.remove(Flags::C);
    }

    pub fn CLD(&mut self) {
        self.status.remove(Flags::D);
    }
    
    pub fn CLI(&mut self) {
        self.status.remove(Flags::I);
    }
    
    pub fn CLV(&mut self) {
        self.status.remove(Flags::V);
    }
    
    pub fn SEC(&mut self) {
        self.status.insert(Flags::C);
    }
    
    pub fn SED(&mut self) {
        self.status.insert(Flags::D);
    }
    
    pub fn SEI(&mut self) {
        self.status.insert(Flags::I);
    }
    
    pub fn BRK(&mut self) {
        let return_address = self.program_counter;//.wrapping_sub(1);
        let l = (return_address & 0xFF) as u8;
        let h = (return_address.wrapping_shr(8) & 0xFF) as u8;
        self.memory[(self.stack_pointer as u16 + 0x0100) as usize] = h;
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        self.memory[(self.stack_pointer as u16 + 0x0100) as usize] = l;
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        self.memory[(self.stack_pointer as u16 + 0x0100) as usize] = self.status.bits();
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        
        self.status.insert(Flags::B);
    }
    
    pub fn RTI(&mut self) {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        self.status = Flags::from_bits(self.memory[(self.stack_pointer as u16 + 0x0100) as usize]).unwrap();

        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        let l = self.memory[(self.stack_pointer as u16 + 0x0100) as usize];
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        let h = self.memory[(self.stack_pointer as u16 + 0x0100) as usize];
        self.program_counter = (h as u16).wrapping_shl(8) | (l as u16);
    }
}