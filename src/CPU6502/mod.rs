mod opcodes;
mod test;

use opcodes::OpCode;
use std::collections::HashMap;

use bitflags::bitflags;
bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Flags: u8 {
        const C = 1 << 0; //Carry
        const Z = 1 << 1; //Zero
        const I = 1 << 2; //Interrupt disable
        const D = 1 << 3; //Decimal mode flag
        const B = 1 << 4; //Break command
        const V = 1 << 5; //Overflow flag
        const N = 1 << 6; //Negative flag
    }
}

pub enum Instruction {
    LDA, LDX, LDY,
    STA, STX, STY,
    TAX, TAY, TXA, TYA, TSX, TXS,
    PHA, PHP, PLA, PLP,
    AND, EOR, ORA, BIT,
    ADC, SBC, CMP, CPX, CPY,
    INC, INX, INY, DEC, DEX, DEY,
    ASL, LSR, ROL, ROR,
    JMP, JSR, RTS,
    BCC, BCS, BEQ, BMI, BNE, BPL, BVC, BVS,
    CLC, CLD, CLI, CLV, SEC, SED, SEI,
    BRK, NOP, RTI,
}

#[derive(Clone, PartialEq, Eq)]
pub enum AddressingMode {
    Implicit,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndexedIndirect,
    IndirectIndexed,
}

pub struct cpu {
    pub memory: [u8; 0xFFFF + 1],
    pub opcodes_map: HashMap<u8, OpCode>,
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub accumulator: i8,
    pub register_x: i8,
    pub register_y: i8,
    pub status: Flags,
}


impl cpu {
    pub fn new() -> Self {
        let opcodes: Vec<OpCode> = vec![
            OpCode::new(0xA9, Instruction::LDA, AddressingMode::Immediate      , 2, 2),
            OpCode::new(0xA5, Instruction::LDA, AddressingMode::ZeroPage       , 2, 3),
            OpCode::new(0xB5, Instruction::LDA, AddressingMode::ZeroPageX      , 2, 4),
            OpCode::new(0xAD, Instruction::LDA, AddressingMode::Absolute       , 3, 4),
            OpCode::new(0xBD, Instruction::LDA, AddressingMode::AbsoluteX      , 3, 4),// or 5
            OpCode::new(0xB9, Instruction::LDA, AddressingMode::AbsoluteY      , 3, 4),// or 5
            OpCode::new(0xA1, Instruction::LDA, AddressingMode::IndexedIndirect, 2, 6),
            OpCode::new(0xB1, Instruction::LDA, AddressingMode::IndirectIndexed, 2, 5),// or 6
            
            OpCode::new(0xA2, Instruction::LDX, AddressingMode::Immediate      , 2, 2),
            OpCode::new(0xA6, Instruction::LDX, AddressingMode::ZeroPage       , 2, 3),
            OpCode::new(0xB6, Instruction::LDX, AddressingMode::ZeroPageY      , 2, 4),
            OpCode::new(0xAE, Instruction::LDX, AddressingMode::Absolute       , 3, 4),
            OpCode::new(0xBE, Instruction::LDX, AddressingMode::AbsoluteY      , 3, 4),// or 5
            
            OpCode::new(0xA0, Instruction::LDY, AddressingMode::Immediate      , 2, 2),
            OpCode::new(0xA4, Instruction::LDY, AddressingMode::ZeroPage       , 2, 3),
            OpCode::new(0xB4, Instruction::LDY, AddressingMode::ZeroPageX      , 2, 4),
            OpCode::new(0xAC, Instruction::LDY, AddressingMode::Absolute       , 3, 4),
            OpCode::new(0xBC, Instruction::LDY, AddressingMode::AbsoluteX      , 3, 4),// or 5
            
            OpCode::new(0x85, Instruction::STA, AddressingMode::ZeroPage       , 2, 3),
            OpCode::new(0x95, Instruction::STA, AddressingMode::ZeroPageX      , 2, 4),
            OpCode::new(0x8D, Instruction::STA, AddressingMode::Absolute       , 3, 4),
            OpCode::new(0x9D, Instruction::STA, AddressingMode::AbsoluteX      , 3, 5),
            OpCode::new(0x99, Instruction::STA, AddressingMode::AbsoluteY      , 3, 5),
            OpCode::new(0x81, Instruction::STA, AddressingMode::IndexedIndirect, 2, 6),
            OpCode::new(0x91, Instruction::STA, AddressingMode::IndirectIndexed, 2, 6),
            
            OpCode::new(0x86, Instruction::STX, AddressingMode::ZeroPage       , 2, 3),
            OpCode::new(0x96, Instruction::STX, AddressingMode::ZeroPageY      , 2, 4),
            OpCode::new(0x8E, Instruction::STX, AddressingMode::Absolute       , 3, 4),
            OpCode::new(0x84, Instruction::STY, AddressingMode::ZeroPage       , 2, 3),
            OpCode::new(0x94, Instruction::STY, AddressingMode::ZeroPageX      , 2, 4),
            OpCode::new(0x8C, Instruction::STY, AddressingMode::Absolute       , 3, 4),
            
            OpCode::new(0xAA, Instruction::TAX, AddressingMode::Implicit       , 1, 2),
            OpCode::new(0xA8, Instruction::TAY, AddressingMode::Implicit       , 1, 2),
            OpCode::new(0x8A, Instruction::TXA, AddressingMode::Implicit       , 1, 2),
            OpCode::new(0x98, Instruction::TYA, AddressingMode::Implicit       , 1, 2),

            OpCode::new(0xBA, Instruction::TSX, AddressingMode::Implicit       , 1, 2),
            OpCode::new(0x9A, Instruction::TXS, AddressingMode::Implicit       , 1, 2),
            OpCode::new(0x48, Instruction::PHA, AddressingMode::Implicit       , 1, 3),
            OpCode::new(0x08, Instruction::PHP, AddressingMode::Implicit       , 1, 3),
            OpCode::new(0x68, Instruction::PLA, AddressingMode::Implicit       , 1, 4),
            OpCode::new(0x28, Instruction::PLP, AddressingMode::Implicit       , 1, 4),

            OpCode::new(0x29, Instruction::AND, AddressingMode::Immediate      , 2, 2),
            OpCode::new(0x25, Instruction::AND, AddressingMode::ZeroPage       , 2, 3),
            OpCode::new(0x35, Instruction::AND, AddressingMode::ZeroPageX      , 2, 4),
            OpCode::new(0x2D, Instruction::AND, AddressingMode::Absolute       , 3, 4),
            OpCode::new(0x3D, Instruction::AND, AddressingMode::AbsoluteX      , 3, 4), // or 5
            OpCode::new(0x39, Instruction::AND, AddressingMode::AbsoluteY      , 3, 4), // or 5
            OpCode::new(0x21, Instruction::AND, AddressingMode::IndexedIndirect, 2, 6),
            OpCode::new(0x31, Instruction::AND, AddressingMode::IndirectIndexed, 2, 5), // or 6

            OpCode::new(0x49, Instruction::EOR, AddressingMode::Immediate      , 2, 2),
            OpCode::new(0x45, Instruction::EOR, AddressingMode::ZeroPage       , 2, 3),
            OpCode::new(0x55, Instruction::EOR, AddressingMode::ZeroPageX      , 2, 4),
            OpCode::new(0x4D, Instruction::EOR, AddressingMode::Absolute       , 3, 4),
            OpCode::new(0x5D, Instruction::EOR, AddressingMode::AbsoluteX      , 3, 4), // or 5
            OpCode::new(0x59, Instruction::EOR, AddressingMode::AbsoluteY      , 3, 4), // or 5
            OpCode::new(0x41, Instruction::EOR, AddressingMode::IndexedIndirect, 2, 6),
            OpCode::new(0x51, Instruction::EOR, AddressingMode::IndirectIndexed, 2, 5), // or 6

            OpCode::new(0x09, Instruction::ORA, AddressingMode::Immediate      , 2, 2),
            OpCode::new(0x05, Instruction::ORA, AddressingMode::ZeroPage       , 2, 3),
            OpCode::new(0x15, Instruction::ORA, AddressingMode::ZeroPageX      , 2, 4),
            OpCode::new(0x0D, Instruction::ORA, AddressingMode::Absolute       , 3, 4),
            OpCode::new(0x1D, Instruction::ORA, AddressingMode::AbsoluteX      , 3, 4), // or 5
            OpCode::new(0x19, Instruction::ORA, AddressingMode::AbsoluteY      , 3, 4), // or 5
            OpCode::new(0x01, Instruction::ORA, AddressingMode::IndexedIndirect, 2, 6),
            OpCode::new(0x11, Instruction::ORA, AddressingMode::IndirectIndexed, 2, 5), // or 6
        
            OpCode::new(0x24, Instruction::BIT, AddressingMode::ZeroPage       , 2, 3),
            OpCode::new(0x2C, Instruction::BIT, AddressingMode::Absolute       , 3, 4),
            
            OpCode::new(0x69, Instruction::ADC, AddressingMode::Immediate      , 2, 2),
            OpCode::new(0x65, Instruction::ADC, AddressingMode::ZeroPage       , 2, 3),
            OpCode::new(0x75, Instruction::ADC, AddressingMode::ZeroPageX      , 2, 4),
            OpCode::new(0x6D, Instruction::ADC, AddressingMode::Absolute       , 3, 4),
            OpCode::new(0x7D, Instruction::ADC, AddressingMode::AbsoluteX      , 3, 4), // or 5
            OpCode::new(0x79, Instruction::ADC, AddressingMode::AbsoluteY      , 3, 4), // or 5
            OpCode::new(0x61, Instruction::ADC, AddressingMode::IndexedIndirect, 2, 6),
            OpCode::new(0x71, Instruction::ADC, AddressingMode::IndirectIndexed, 2, 5), // or 6
            
            OpCode::new(0xE9, Instruction::SBC, AddressingMode::Immediate      , 2, 2),
            OpCode::new(0xE5, Instruction::SBC, AddressingMode::ZeroPage       , 2, 3),
            OpCode::new(0xF5, Instruction::SBC, AddressingMode::ZeroPageX      , 2, 4),
            OpCode::new(0xED, Instruction::SBC, AddressingMode::Absolute       , 3, 4),
            OpCode::new(0xFD, Instruction::SBC, AddressingMode::AbsoluteX      , 3, 4), // or 5
            OpCode::new(0xF9, Instruction::SBC, AddressingMode::AbsoluteY      , 3, 4), // or 5
            OpCode::new(0xE1, Instruction::SBC, AddressingMode::IndexedIndirect, 2, 6),
            OpCode::new(0xF1, Instruction::SBC, AddressingMode::IndirectIndexed, 2, 5), // or 6
            
            OpCode::new(0xC9, Instruction::CMP, AddressingMode::Immediate      , 2, 2),
            OpCode::new(0xC5, Instruction::CMP, AddressingMode::ZeroPage       , 2, 3),
            OpCode::new(0xD5, Instruction::CMP, AddressingMode::ZeroPageX      , 2, 4),
            OpCode::new(0xCD, Instruction::CMP, AddressingMode::Absolute       , 3, 4),
            OpCode::new(0xDD, Instruction::CMP, AddressingMode::AbsoluteX      , 3, 4), // or 5
            OpCode::new(0xD9, Instruction::CMP, AddressingMode::AbsoluteY      , 3, 4), // or 5
            OpCode::new(0xC1, Instruction::CMP, AddressingMode::IndexedIndirect, 2, 6),
            OpCode::new(0xD1, Instruction::CMP, AddressingMode::IndirectIndexed, 2, 5), // or 6
            
            OpCode::new(0xE0, Instruction::CPX, AddressingMode::Immediate      , 2, 2),
            OpCode::new(0xE4, Instruction::CPX, AddressingMode::ZeroPage       , 2, 3),
            OpCode::new(0xEC, Instruction::CPX, AddressingMode::Absolute       , 3, 4),
    
            OpCode::new(0xC0, Instruction::CPY, AddressingMode::Immediate      , 2, 2),
            OpCode::new(0xC4, Instruction::CPY, AddressingMode::ZeroPage       , 2, 3),
            OpCode::new(0xCC, Instruction::CPY, AddressingMode::Absolute       , 3, 4),
            
            OpCode::new(0xE6, Instruction::INC, AddressingMode::ZeroPage       , 2, 5),
            OpCode::new(0xF6, Instruction::INC, AddressingMode::ZeroPageX      , 2, 6),
            OpCode::new(0xEE, Instruction::INC, AddressingMode::Absolute       , 3, 6),
            OpCode::new(0xFE, Instruction::INC, AddressingMode::AbsoluteX      , 3, 7),
            
            OpCode::new(0xE8, Instruction::INX, AddressingMode::Implicit       , 1, 2),

            OpCode::new(0xC8, Instruction::INY, AddressingMode::Implicit       , 1, 2),

            OpCode::new(0xC6, Instruction::DEC, AddressingMode::ZeroPage       , 2, 5),
            OpCode::new(0xD6, Instruction::DEC, AddressingMode::ZeroPageX      , 2, 6),
            OpCode::new(0xCE, Instruction::DEC, AddressingMode::Absolute       , 3, 6),
            OpCode::new(0xDE, Instruction::DEC, AddressingMode::AbsoluteX      , 3, 7),
            
            OpCode::new(0xCA, Instruction::DEX, AddressingMode::Implicit       , 1, 2),

            OpCode::new(0x88, Instruction::DEY, AddressingMode::Implicit       , 1, 2),
            
            OpCode::new(0x0A, Instruction::ASL, AddressingMode::Accumulator    , 1, 2),
            OpCode::new(0x06, Instruction::ASL, AddressingMode::ZeroPage       , 2, 5),
            OpCode::new(0x16, Instruction::ASL, AddressingMode::ZeroPageX      , 2, 6),
            OpCode::new(0x0E, Instruction::ASL, AddressingMode::Absolute       , 3, 6),
            OpCode::new(0x1E, Instruction::ASL, AddressingMode::AbsoluteX      , 3, 7),
            
            OpCode::new(0x4A, Instruction::LSR, AddressingMode::Accumulator    , 1, 2),
            OpCode::new(0x46, Instruction::LSR, AddressingMode::ZeroPage       , 2, 5),
            OpCode::new(0x56, Instruction::LSR, AddressingMode::ZeroPageX      , 2, 6),
            OpCode::new(0x4E, Instruction::LSR, AddressingMode::Absolute       , 3, 6),
            OpCode::new(0x5E, Instruction::LSR, AddressingMode::AbsoluteX      , 3, 7),
            
            OpCode::new(0x2A, Instruction::ROL, AddressingMode::Accumulator    , 1, 2),
            OpCode::new(0x26, Instruction::ROL, AddressingMode::ZeroPage       , 2, 5),
            OpCode::new(0x36, Instruction::ROL, AddressingMode::ZeroPageX      , 2, 6),
            OpCode::new(0x2E, Instruction::ROL, AddressingMode::Absolute       , 3, 6),
            OpCode::new(0x3E, Instruction::ROL, AddressingMode::AbsoluteX      , 3, 7),
            
            OpCode::new(0x6A, Instruction::ROR, AddressingMode::Accumulator    , 1, 2),
            OpCode::new(0x66, Instruction::ROR, AddressingMode::ZeroPage       , 2, 5),
            OpCode::new(0x76, Instruction::ROR, AddressingMode::ZeroPageX      , 2, 6),
            OpCode::new(0x6E, Instruction::ROR, AddressingMode::Absolute       , 3, 6),
            OpCode::new(0x7E, Instruction::ROR, AddressingMode::AbsoluteX      , 3, 7),
            
            OpCode::new(0x4C, Instruction::JMP, AddressingMode::Absolute       , 3, 3),
            OpCode::new(0x6C, Instruction::JMP, AddressingMode::Indirect       , 3, 5),
            
            OpCode::new(0x20, Instruction::JSR, AddressingMode::Absolute       , 3, 6),
            
            OpCode::new(0x60, Instruction::RTS, AddressingMode::Implicit       , 1, 6),
            
            OpCode::new(0x90, Instruction::BCC, AddressingMode::Relative       , 2, 2), // or 3 or 4
            OpCode::new(0xB0, Instruction::BCS, AddressingMode::Relative       , 2, 2), // or 3 or 4
            OpCode::new(0xF0, Instruction::BEQ, AddressingMode::Relative       , 2, 2), // or 3 or 4
            OpCode::new(0x30, Instruction::BMI, AddressingMode::Relative       , 2, 2), // or 3 or 4
            OpCode::new(0xD0, Instruction::BNE, AddressingMode::Relative       , 2, 2), // or 3 or 4
            OpCode::new(0x10, Instruction::BPL, AddressingMode::Relative       , 2, 2), // or 3 or 4
            OpCode::new(0x50, Instruction::BVC, AddressingMode::Relative       , 2, 2), // or 3 or 4
            OpCode::new(0x70, Instruction::BVS, AddressingMode::Relative       , 2, 2), // or 3 or 4

            OpCode::new(0x18, Instruction::CLC, AddressingMode::Implicit       , 1, 2),
            OpCode::new(0xD8, Instruction::CLD, AddressingMode::Implicit       , 1, 2),
            OpCode::new(0x58, Instruction::CLI, AddressingMode::Implicit       , 1, 2),
            OpCode::new(0xB8, Instruction::CLV, AddressingMode::Implicit       , 1, 2),
            OpCode::new(0x38, Instruction::SEC, AddressingMode::Implicit       , 1, 2),
            OpCode::new(0xF8, Instruction::SED, AddressingMode::Implicit       , 1, 2),
            OpCode::new(0x78, Instruction::SEI, AddressingMode::Implicit       , 1, 2),
            
            OpCode::new(0x00, Instruction::BRK, AddressingMode::Implicit       , 1, 7),
            OpCode::new(0xEA, Instruction::NOP, AddressingMode::Implicit       , 1, 2),
            OpCode::new(0x40, Instruction::RTI, AddressingMode::Implicit       , 1, 6),
        ];
        let mut opcodes_map: HashMap<u8, OpCode> = HashMap::new();
        for opcode in opcodes {
            opcodes_map.insert(opcode.code, opcode);
        }
            
        let device = cpu {
            memory: [0 as u8; 0xFFFF + 1],
            opcodes_map: opcodes_map,
            program_counter: 0x8000,
            stack_pointer: 0,
            accumulator: 0,
            register_x: 0,
            register_y: 0,
            status: Flags::empty(),
        };
        device
    }

    pub fn get_operand_address(&mut self, mode: AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => {
                self.program_counter - 1
            }
            AddressingMode::ZeroPage => {
                self.memory[(self.program_counter - 1) as usize] as u16
            }
            AddressingMode::ZeroPageX => {
                (( self.memory[(self.program_counter - 1) as usize] as u16 + (self.register_x as u8) as u16) & 0xFF) as u16
            }
            AddressingMode::ZeroPageY => {
                (( self.memory[(self.program_counter - 1) as usize] as u16 + (self.register_y as u8) as u16) & 0xFF) as u16
            }
            AddressingMode::Absolute => {
                let h = self.memory[(self.program_counter - 1) as usize] as u16;
                let l = self.memory[(self.program_counter - 2) as usize] as u16;
                (h << 8) | l
            }
            AddressingMode::AbsoluteX => {
                self.get_operand_address(AddressingMode::Absolute) + (self.register_x as u8) as u16
            }
            AddressingMode::AbsoluteY => {
                self.get_operand_address(AddressingMode::Absolute) + (self.register_y as u8) as u16
            }
            AddressingMode::IndexedIndirect => {
                let address_tmp: u16 = self.memory[(self.program_counter - 1) as usize] as u16 + ((self.register_x as u8) as u16);
                let l: u16 = self.memory[address_tmp as usize] as u16;
                let h: u16 = self.memory[(address_tmp as u16 + 1) as usize] as u16;
                (h << 8) | l
            }
            AddressingMode::IndirectIndexed => {
                let address_tmp: u16 = self.memory[(self.program_counter - 1) as usize] as u16;
                let l: u16 = self.memory[address_tmp as usize] as u16;
                let h: u16 = self.memory[(address_tmp + 1) as usize] as u16;
                ((h << 8) | l) + (self.register_y as u8) as u16
            }
            AddressingMode::Relative => {
                self.memory[(self.program_counter - 1) as usize] as u16
            }
            _ => { 0 }
        }
    }


    pub fn load_rom(&mut self, memory: Vec<u8>) {
        self.memory[0x8000 .. (0x8000 + memory.len()) ].copy_from_slice(&memory);
    }

    pub fn load_abs(&mut self, abs: u16, bytes: Vec<u8>) {
        self.memory[abs as usize .. (abs as usize + bytes.len())].copy_from_slice(&bytes);
    }

    pub fn print_abs(&mut self, abs: u16, l: u16) {
        let mut iter = self.memory[abs as usize .. (abs + l) as usize].iter();

        if let Some(item) = iter.next() {
            print!("{}", item);

            for item in iter {
                print!(", {}", item);
            }
        }
        println!("");
    }
    
    pub fn interpret_single(&mut self) {
        let opcode_t = &self.memory[self.program_counter as usize];

        if let Some(opcode) = self.opcodes_map.get(opcode_t) {
            self.program_counter += opcode.nbyte as u16;

            match &opcode.instruction {
                Instruction::LDA => self.LDA(opcode.addressing_mode.clone()),
                Instruction::LDX => self.LDX(opcode.addressing_mode.clone()),
                Instruction::LDY => self.LDY(opcode.addressing_mode.clone()),
                Instruction::STA => self.STA(opcode.addressing_mode.clone()),
                Instruction::STX => self.STX(opcode.addressing_mode.clone()),
                Instruction::STY => self.STY(opcode.addressing_mode.clone()),
                Instruction::TAX => self.TAX(),
                Instruction::TAY => self.TAY(),
                Instruction::TXA => self.TXA(),
                Instruction::TYA => self.TYA(),
                Instruction::TSX => self.TSX(),
                Instruction::TXS => self.TXS(),
                Instruction::PHA => self.PHA(),
                Instruction::PHP => self.PHP(),
                Instruction::PLA => self.PLA(),
                Instruction::PLP => self.PLP(),
                Instruction::AND => self.AND(opcode.addressing_mode.clone()),
                Instruction::EOR => self.EOR(opcode.addressing_mode.clone()),
                Instruction::ORA => self.ORA(opcode.addressing_mode.clone()),
                Instruction::BIT => self.BIT(opcode.addressing_mode.clone()),
                Instruction::ADC => self.ADC(opcode.addressing_mode.clone()),
                Instruction::SBC => self.SBC(opcode.addressing_mode.clone()),
                Instruction::CMP => self.CMP(opcode.addressing_mode.clone()),
                Instruction::CPX => self.CPX(opcode.addressing_mode.clone()),
                Instruction::CPY => self.CPY(opcode.addressing_mode.clone()),
                Instruction::INC => self.INC(opcode.addressing_mode.clone()),
                Instruction::INX => self.INX(),
                Instruction::INY => self.INY(),
                Instruction::DEC => self.DEC(opcode.addressing_mode.clone()),
                Instruction::DEX => self.DEX(),
                Instruction::DEY => self.DEY(),
                Instruction::ASL => self.ASL(opcode.addressing_mode.clone()),
                Instruction::LSR => self.LSR(opcode.addressing_mode.clone()),
                Instruction::ROL => self.ROL(opcode.addressing_mode.clone()),
                Instruction::ROR => self.ROR(opcode.addressing_mode.clone()),
                Instruction::JMP => self.JMP(opcode.addressing_mode.clone()),
                Instruction::JSR => self.JSR(opcode.addressing_mode.clone()),
                Instruction::RTS => self.RTS(),
                Instruction::BCC => self.BCC(opcode.addressing_mode.clone()),
                Instruction::BCS => self.BCS(opcode.addressing_mode.clone()),
                Instruction::BEQ => self.BEQ(opcode.addressing_mode.clone()),
                Instruction::BMI => self.BMI(opcode.addressing_mode.clone()),
                Instruction::BNE => self.BNE(opcode.addressing_mode.clone()),
                Instruction::BPL => self.BPL(opcode.addressing_mode.clone()),
                Instruction::BVC => self.BVC(opcode.addressing_mode.clone()),
                Instruction::BVS => self.BVS(opcode.addressing_mode.clone()),
                Instruction::CLC => self.CLC(),
                Instruction::CLD => self.CLD(),
                Instruction::CLI => self.CLI(),
                Instruction::CLV => self.CLV(),
                Instruction::SEC => self.SEC(),
                Instruction::SED => self.SED(),
                Instruction::SEI => self.SEI(),
                Instruction::BRK => self.BRK(),
                Instruction::NOP => (),
                Instruction::RTI => self.RTI(),
            }
        }
    }
}