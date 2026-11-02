#[cfg(test)]
use super::{cpu, Flags};

#[test]
fn test_lda_and_memory_addressing_modes() -> () {
    let mut cpu_dut = cpu::new();
    cpu_dut.register_x = 0xFFu8 as i8;
    cpu_dut.register_y = 0xFFu8 as i8;
    cpu_dut.load_abs(0x0000, vec![42]);
    cpu_dut.load_abs(0x00FF, vec![23]);
    cpu_dut.load_abs(0x00FE, vec![50]);
    cpu_dut.load_abs(0x01FE, vec![55]);
    cpu_dut.load_abs(0xFFFF, vec![72]);
    cpu_dut.load_abs(23, vec![100]);
    cpu_dut.load_abs(297, vec![101]);
    cpu_dut.load_abs(0x101, vec![0xFF, 0xFF]);
    let memory: Vec<u8> = vec![0xA9, 0x00, 0xA9, 0x7F, 0xA9, 0xFF,                   //Immediate addressing
                                0xA5, 0x00, 0xA5, 0xFF,                               //ZeroPage
                                0xB5, 0x00, 0xB5, 0xFF,                               //ZeroPageX @ (0x00 + 0xFF = 0xFF, 0xFF + 0xFF = 0x1FE & 0xFF = 0xFE)
                                0xAD, 0x00, 0x00, 0xAD, 0xFF, 0x00, 0xAD, 0xFF, 0xFF, //Absolute (@ 0x0000, 0x00FF, 0xFFFF)
                                0xBD, 0x00, 0x00, 0xBD, 0xFF, 0x00, 0xBD, 0x00, 0xFF, //AbsoluteX (@ 0x0000+0xFF = 0x00FF, 0x00FF + 0xFF = 0x01FE, 0xFF00 + 0xFF = 0xFFFF)
                                0xB9, 0x00, 0x00, 0xB9, 0xFF, 0x00, 0xB9, 0x00, 0xFF, //AbsoluteY (@ 0x0000+0x00FF, 0x00FF + 0x00FF = 0x01FE, 0xFFFF, 0xFF00 + 0x00FF)
                                0xA1, 0x00, 0xA1, 0x02,                                //IndexedIndirect (0x00 + 0xFF = 0x00FF -> 23 -> 100, 0x02 + 0xFF = 0x101 -> 0xFFFF -> 72)
                                0xB1, 0x00,                                            //IndirectIndexed (0x00 -> (0x00 << 8) | 42 + 0xFF = 297 -> 101)
                                ];
    cpu_dut.load_rom(memory);
    
    //Immediate
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.accumulator, 0x00, "wrong accumulator value");
    assert_eq!(cpu_dut.status & Flags::Z, Flags::Z      , "wrong Z flag value");
    assert_eq!(cpu_dut.status & Flags::N, Flags::empty(), "wrong N flag value");
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.accumulator, 0x7F, "wrong accumulator value");
    assert_eq!(cpu_dut.status & Flags::Z, Flags::empty(), "wrong Z flag value");
    assert_eq!(cpu_dut.status & Flags::N, Flags::empty(), "wrong N flag value");
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.accumulator, 0xFFu8 as i8, "wrong accumulator value");
    assert_eq!(cpu_dut.status & Flags::Z, Flags::empty(), "wrong Z flag value");
    assert_eq!(cpu_dut.status & Flags::N, Flags::N      , "wrong N flag value");

    //ZeroPage
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.accumulator, 42 as i8, "wrong accumulator value");
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.accumulator, 23 as i8, "wrong accumulator value");

    //ZeroPageX
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.accumulator, 23 as i8, "wrong accumulator value");
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.accumulator, 50 as i8, "wrong accumulator value");

    //Absolute
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.accumulator, 42 as i8, "wrong accumulator value");
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.accumulator, 23 as i8, "wrong accumulator value");
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.accumulator, 72 as i8, "wrong accumulator value");

    //AbsoluteX
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.accumulator, 23 as i8, "wrong accumulator value");
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.accumulator, 55 as i8, "wrong accumulator value");
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.accumulator, 72 as i8, "wrong accumulator value");

    //AbsoluteY
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.accumulator, 23 as i8, "wrong accumulator value");
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.accumulator, 55 as i8, "wrong accumulator value");
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.accumulator, 72 as i8, "wrong accumulator value");

    //IndexedIndirect
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.accumulator, 100 as i8, "wrong accumulator value");
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.accumulator, 72 as i8, "wrong accumulator value");

    //IndirectIndexed
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.accumulator, 101 as i8, "wrong accumulator value");
}

#[test]
fn test_ldx() -> () {
    let mut cpu_dut = cpu::new();
    let memory: Vec<u8> = vec![0xA2, 0x00, 0xA2, 0x7F, 0xA2, 0xFF]; //Immediate
    cpu_dut.load_rom(memory);
    
    //Immediate
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.register_x, 0x00, "wrong register_x value");
    assert_eq!(cpu_dut.status & Flags::Z, Flags::Z      , "wrong Z flag value");
    assert_eq!(cpu_dut.status & Flags::N, Flags::empty(), "wrong N flag value");
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.register_x, 0x7F, "wrong register_x value");
    assert_eq!(cpu_dut.status & Flags::Z, Flags::empty(), "wrong Z flag value");
    assert_eq!(cpu_dut.status & Flags::N, Flags::empty(), "wrong N flag value");
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.register_x, 0xFFu8 as i8, "wrong register_x value");
    assert_eq!(cpu_dut.status & Flags::Z, Flags::empty(), "wrong Z flag value");
    assert_eq!(cpu_dut.status & Flags::N, Flags::N      , "wrong N flag value");
}


#[test]
fn test_ldy() -> () {
    let mut cpu_dut = cpu::new();
    let memory: Vec<u8> = vec![0xA0, 0x00, 0xA0, 0x7F, 0xA0, 0xFF]; //Immediate
    cpu_dut.load_rom(memory);
    
    //Immediate
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.register_y, 0x00, "wrong register_y value");
    assert_eq!(cpu_dut.status & Flags::Z, Flags::Z      , "wrong Z flag value");
    assert_eq!(cpu_dut.status & Flags::N, Flags::empty(), "wrong N flag value");
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.register_y, 0x7F, "wrong register_y value");
    assert_eq!(cpu_dut.status & Flags::Z, Flags::empty(), "wrong Z flag value");
    assert_eq!(cpu_dut.status & Flags::N, Flags::empty(), "wrong N flag value");
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.register_y, 0xFFu8 as i8, "wrong register_y value");
    assert_eq!(cpu_dut.status & Flags::Z, Flags::empty(), "wrong Z flag value");
    assert_eq!(cpu_dut.status & Flags::N, Flags::N      , "wrong N flag value");
}

#[test]
fn test_sta() -> () {
    let mut cpu_dut = cpu::new();
    let memory: Vec<u8> = vec![0x85, 0x00, 0x85, 0xFF, //ZeroPage
                                0x95, 0xFF, //ZeroPageX
                                0x9D, 0x00, 0xFF, //AbsoluteX
                                0x81, 0x01, //IndexedIndirect (0x01 + 0xFF = 0x100 -> 0x100 is low and 0x101 is high -> 0x10)
                                0x91, 0x01, //IndirectIndexed (0x1 is low and 0x2 is high -> 0x100 + 0xFF = 0x1FF)
                                ];
    cpu_dut.accumulator = 42 as i8;
    cpu_dut.register_x = 0xFFu8 as i8;
    cpu_dut.register_y = 0xFFu8 as i8;
    cpu_dut.load_abs(0x101, vec![1]);
    cpu_dut.load_abs(0x1, vec![0, 1]);
    cpu_dut.load_rom(memory);
    
    //ZeroPage
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.memory[0x00], 42, "wrong memory value");
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.memory[0xFF], 42, "wrong memory value");
    
    //ZeroPageX
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.memory[0xFE], 42, "wrong memory value");
    assert_eq!(cpu_dut.memory[0x1FE], 0, "should'nt have been modified");
    
    //AbsoluteX
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.memory[0xFFFF], 42, "wrong memory value");
    
    //IndexedIndirect
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.memory[0x100], 42, "wrong memory value");

    //IndirectIndexed
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.memory[0x1FF], 42, "wrong memory value");    
}

#[test]
fn test_stx() -> () {
    let mut cpu_dut = cpu::new();
    let memory: Vec<u8> = vec![0x8E, 0x00, 0xFF]; //Absolute
    cpu_dut.register_x = 42;
    cpu_dut.load_rom(memory);
    
    //Absolute
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.memory[0xFF00], 42, "wrong memory value");
}

#[test]
fn test_sty() -> () {
    let mut cpu_dut = cpu::new();
    let memory: Vec<u8> = vec![0x8C, 0xFF, 0xFF]; //Absolute
    cpu_dut.register_y = 42;
    cpu_dut.load_rom(memory);
    
    //Absolute
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.memory[0xFFFF], 42, "wrong memory value");
}

#[test]
fn test_tax_tay_txa_tya() -> () {
    let mut cpu_dut = cpu::new();
    let memory: Vec<u8> = vec![0xAA, 0xA8, 0x8A, 0x98]; //Implicit TAX, TAY, TXA, TYA
    cpu_dut.load_rom(memory);
    
    cpu_dut.accumulator = 42;
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.register_x, 42, "wrong register_x value");
    cpu_dut.accumulator = 43;
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.register_y, 43, "wrong register_y value");
    
    cpu_dut.register_x = 44;
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.accumulator, 44, "wrong accumulator value");
    cpu_dut.register_y = 45;
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.accumulator, 45, "wrong accumulator value");
}

#[test]
fn test_tsx_txs_pha_php_pla_plp() -> () {
    let mut cpu_dut = cpu::new();
    let memory: Vec<u8> = vec![0x9A, 0x48, 0x08, 0xBA, 0x28, 0x68]; //TXS, PHA, PHP, TSX, PLP, PLA
    cpu_dut.load_rom(memory);
    
    assert_eq!(cpu_dut.stack_pointer, 0x00);
    
    cpu_dut.register_x = 0xFFu8 as i8;
    cpu_dut.interpret_single(); // TXS: S = reg X = 0xFF
    assert_eq!(cpu_dut.stack_pointer, 0xFF);
    
    cpu_dut.accumulator = 42;
    cpu_dut.interpret_single(); // PHA: @0x01FF = 42
    assert_eq!(cpu_dut.memory[0x01FF], 42);
    assert_eq!(cpu_dut.stack_pointer, 0xFE);
    
    let tmp_flags = Flags::Z | Flags::C | Flags::V;
    cpu_dut.status = tmp_flags;
    cpu_dut.interpret_single(); //PHP
    assert_eq!(cpu_dut.memory[0x01FE], tmp_flags.bits());
    assert_eq!(cpu_dut.stack_pointer, 0xFD);
    
    cpu_dut.status = Flags::empty();
    cpu_dut.interpret_single(); //TSX
    assert_eq!(cpu_dut.register_x as u8, 0xFDu8);
    assert_eq!(cpu_dut.status, Flags::N);

    cpu_dut.interpret_single(); //PLP
    assert_eq!(cpu_dut.status, tmp_flags);
    assert_eq!(cpu_dut.stack_pointer, 0xFE);

    cpu_dut.interpret_single(); //PLA
    assert_eq!(cpu_dut.accumulator, 42);
    assert_eq!(cpu_dut.stack_pointer, 0xFF);
}

#[test]
fn test_and_eor_ora_bit() -> () {
    let mut cpu_dut = cpu::new();
    let memory: Vec<u8> = vec![0x29, 0xFF,  // AND Immediate
                                0x49, 0xFF,  // EOR Immediate
                                0x09, 0x0F,  // ORA Immediate
                                0x24, 0xFF, 0x24, 0xFF, 0x24, 0xFF]; // BIT ZeroPage
    cpu_dut.load_rom(memory);
    cpu_dut.load_abs(0xFF, vec![0xFF]);
    
    cpu_dut.accumulator = 0b10101111u8 as i8;
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.accumulator, 0b10101111u8 as i8);
    
    cpu_dut.accumulator = 0b10101111u8 as i8;
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.accumulator, 0b01010000u8 as i8);
    
    cpu_dut.accumulator = 0b10100110u8 as i8;
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.accumulator, 0b10101111u8 as i8);

    cpu_dut.accumulator = 0;
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.status, Flags::Z);

    cpu_dut.accumulator = (1 << 7) as i8;
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.status, Flags::N);

    cpu_dut.accumulator = (1 << 6) as i8;
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.status, Flags::V);

    println!("{}", (-17 as i8) as i16);
}

#[test]
fn test_adc() -> () {
    let mut cpu_dut = cpu::new();
    /*  lda #$10
        adc #$50
        clc

        lda #$10
        sec
        adc #$50
        clc

        lda #$50
        adc #$50
        clc

        lda #$90
        adc #$50
        clc

        lda #$D0
        adc #$50
        clc

        lda #$D0
        sec
        adc #$50
        clc

        lda #$10
        adc #$D0
        clc

        lda #$50
        adc #$D0
        clc

        lda #$50
        sec
        adc #$D0
        clc

        lda #$90
        adc #$D0
        clc

        lda #$90
        sec
        adc #$D0
        clc

        lda #$D0
        adc #$D0
        clc

        lda #$D0
        sec
        adc #$D0 */
    let memory: Vec<u8> = vec![0xA9, 0x10, 0x69, 0x50,
                                0xA9, 0x10, 0x69, 0x50, //C
                                0xA9, 0x50, 0x69, 0x50,
                                0xA9, 0x90, 0x69, 0x50,
                                0xA9, 0xD0, 0x69, 0x50,
                                0xA9, 0xD0, 0x69, 0x50, //C
                                0xA9, 0x10, 0x69, 0xD0,
                                0xA9, 0x50, 0x69, 0xD0,
                                0xA9, 0x50, 0x69, 0xD0, //C
                                0xA9, 0x90, 0x69, 0xD0,
                                0xA9, 0x90, 0x69, 0xD0, //C
                                0xA9, 0xD0, 0x69, 0xD0,
                                0xA9, 0xD0, 0x69, 0xD0,]; //C
    cpu_dut.load_rom(memory);
    
    cpu_dut.interpret_single(); //10
    cpu_dut.interpret_single(); //50
    assert_eq!(cpu_dut.accumulator, 0x60 as i8);
    assert_eq!(cpu_dut.status, Flags::empty());
    
    cpu_dut.status.insert(Flags::C);
    cpu_dut.interpret_single(); //10
    cpu_dut.interpret_single(); //50
    assert_eq!(cpu_dut.accumulator, 0x61 as i8);
    assert_eq!(cpu_dut.status, Flags::empty());

    cpu_dut.interpret_single(); //50
    cpu_dut.interpret_single(); //50
    assert_eq!(cpu_dut.accumulator, 0xA0u8 as i8);
    assert_eq!(cpu_dut.status, Flags::V | Flags::N);
    
    cpu_dut.interpret_single(); //90
    cpu_dut.interpret_single(); //50
    assert_eq!(cpu_dut.accumulator, 0xE0u8 as i8);
    assert_eq!(cpu_dut.status, Flags::N);

    cpu_dut.interpret_single(); //D0
    cpu_dut.interpret_single(); //50
    assert_eq!(cpu_dut.accumulator, 0x20 as i8);
    assert_eq!(cpu_dut.status, Flags::C);
    cpu_dut.status.remove(Flags::C);
    
    cpu_dut.status.insert(Flags::C);
    cpu_dut.interpret_single(); //D0
    cpu_dut.interpret_single(); //50
    assert_eq!(cpu_dut.accumulator, 0x21 as i8);
    assert_eq!(cpu_dut.status, Flags::C);
    cpu_dut.status.remove(Flags::C);
    
    
    cpu_dut.interpret_single(); //10
    cpu_dut.interpret_single(); //D0
    assert_eq!(cpu_dut.accumulator, 0xE0u8 as i8);
    assert_eq!(cpu_dut.status, Flags::N);
    
    cpu_dut.interpret_single(); //50
    cpu_dut.interpret_single(); //D0
    assert_eq!(cpu_dut.accumulator, 0x20 as i8);
    assert_eq!(cpu_dut.status, Flags::C);
    cpu_dut.status.remove(Flags::C);
    
    cpu_dut.status.insert(Flags::C);
    cpu_dut.interpret_single(); //50
    cpu_dut.interpret_single(); //D0
    assert_eq!(cpu_dut.accumulator, 0x21 as i8);
    assert_eq!(cpu_dut.status, Flags::C);
    cpu_dut.status.remove(Flags::C);
    
    cpu_dut.interpret_single(); //90
    cpu_dut.interpret_single(); //D0
    assert_eq!(cpu_dut.accumulator, 0x60 as i8);
    assert_eq!(cpu_dut.status, Flags::C | Flags::V);
    cpu_dut.status.remove(Flags::C);

    cpu_dut.status.insert(Flags::C);
    cpu_dut.interpret_single(); //90
    cpu_dut.interpret_single(); //D0
    assert_eq!(cpu_dut.accumulator, 0x61 as i8);
    assert_eq!(cpu_dut.status, Flags::C | Flags::V);
    cpu_dut.status.remove(Flags::C);
    
    cpu_dut.interpret_single(); //D0
    cpu_dut.interpret_single(); //D0
    assert_eq!(cpu_dut.accumulator, 0xA0u8 as i8);
    assert_eq!(cpu_dut.status, Flags::C | Flags::N);
    cpu_dut.status.remove(Flags::C);
    
    cpu_dut.status.insert(Flags::C);
    cpu_dut.interpret_single(); //D0
    cpu_dut.interpret_single(); //D0
    assert_eq!(cpu_dut.accumulator, 0xA1u8 as i8);
    assert_eq!(cpu_dut.status, Flags::C | Flags::N);
}

#[test]
fn test_sbc() -> () {
    let mut cpu_dut = cpu::new();
    let memory: Vec<u8> = vec![0xA9, 0x50, 0xE9, 0xF0,
                                0xA9, 0x50, 0xE9, 0xB0, //C
                                0xA9, 0x50, 0xE9, 0x70,
                                0xA9, 0x50, 0xE9, 0x30,

                                0xA9, 0xD0, 0xE9, 0xF0,
                                0xA9, 0xD0, 0xE9, 0xB0,
                                0xA9, 0xD0, 0xE9, 0x70,
                                0xA9, 0xD0, 0xE9, 0x30,];
    /*lda #$50
        sbc #$F0
        clc

        lda #$50
        sec
        sbc #$B0
        clc

        lda #$50
        sbc #$70
        clc

        lda #$50
        sbc #$30
        clc


        lda #$D0
        sbc #$F0
        clc

        lda #$D0
        sbc #$B0
        clc

        lda #$D0
        sbc #$70
        clc

        lda #$D0
        sbc #$30
        clc
        */
    cpu_dut.load_rom(memory);
    //A-M-(1-C)
    
    cpu_dut.interpret_single(); //50
    cpu_dut.interpret_single(); //F0
    assert_eq!(cpu_dut.accumulator, 0x5F);
    assert_eq!(cpu_dut.status, Flags::empty());
    cpu_dut.status.remove(Flags::C);
    
    cpu_dut.status.insert(Flags::C);
    cpu_dut.interpret_single(); //50
    cpu_dut.interpret_single(); //B0
    assert_eq!(cpu_dut.accumulator, 0xA0u8 as i8);
    assert_eq!(cpu_dut.status, Flags::V | Flags::N);
    cpu_dut.status.remove(Flags::C);

    cpu_dut.interpret_single(); //50
    cpu_dut.interpret_single(); //70
    assert_eq!(cpu_dut.accumulator, 0xDFu8 as i8);
    assert_eq!(cpu_dut.status, Flags::N);
    cpu_dut.status.remove(Flags::C);
    
    cpu_dut.interpret_single(); //50
    cpu_dut.interpret_single(); //30
    assert_eq!(cpu_dut.accumulator, 0x1F as i8);
    assert_eq!(cpu_dut.status, Flags::C);
    cpu_dut.status.remove(Flags::C);

    cpu_dut.interpret_single(); //D0
    cpu_dut.interpret_single(); //F0
    assert_eq!(cpu_dut.accumulator, 0xDFu8 as i8);
    assert_eq!(cpu_dut.status, Flags::N);
    cpu_dut.status.remove(Flags::C);
    
    cpu_dut.interpret_single(); //D0
    cpu_dut.interpret_single(); //B0
    assert_eq!(cpu_dut.accumulator, 0x1F as i8);
    assert_eq!(cpu_dut.status, Flags::C);
    cpu_dut.status.remove(Flags::C);
    
    cpu_dut.interpret_single(); //D0
    cpu_dut.interpret_single(); //70
    assert_eq!(cpu_dut.accumulator, 0x5F as i8);
    assert_eq!(cpu_dut.status, Flags::C | Flags::V);
    cpu_dut.status.remove(Flags::C);
    
    cpu_dut.interpret_single(); //D0
    cpu_dut.interpret_single(); //30
    assert_eq!(cpu_dut.accumulator, 0x9Fu8 as i8);
    assert_eq!(cpu_dut.status, Flags::C | Flags::N);
    cpu_dut.status.remove(Flags::C);
}

#[test]
fn test_cmp() -> () {
    let mut cpu_dut = cpu::new();
    let memory: Vec<u8> = vec![0xA9, 0x00, 0xC9, 0x00, //LDA, CMP
                                0xA9, 0xFF, 0xC9, 0xFF, //LDA, CMP
                                0xA9, 0x1F, 0xC9, 0x1F, //LDA, CMP

                                0xA9, 0x01, 0xC9, 0x00, //LDA, CMP
                                0xA9, 0x20, 0xC9, 0x1F, //LDA, CMP
                                0xA9, 0xFF, 0xC9, 0xFE, //LDA, CMP

                                0xA9, 0x01, 0xC9, 0x02, //LDA, CMP
                                0xA9, 0x1F, 0xC9, 0x20, //LDA, CMP
                                0xA9, 0xFE, 0xC9, 0xFF, //LDA, CMP

                                0xA2, 0xFF, 0xE0, 0xFF, //LDA, CPX
                                0xA2, 0x20, 0xE0, 0x1F, //LDA, CPX
                                0xA2, 0x1F, 0xE0, 0x20, //LDA, CPX

                                0xA0, 0xFF, 0xC0, 0xFF, //LDA, CPY
                                0xA0, 0x20, 0xC0, 0x1F, //LDA, CPY
                                0xA0, 0x1F, 0xC0, 0x20, //LDA, CPY
    ];
    cpu_dut.load_rom(memory);
    
    cpu_dut.interpret_single();
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.status, Flags::Z | Flags::C);
    cpu_dut.interpret_single();
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.status, Flags::Z | Flags::C);
    cpu_dut.interpret_single();
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.status, Flags::Z | Flags::C);
    
    cpu_dut.interpret_single();
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.status, Flags::C);
    cpu_dut.interpret_single();
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.status, Flags::C);
    cpu_dut.interpret_single();
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.status, Flags::C);

    cpu_dut.interpret_single();
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.status, Flags::N);
    cpu_dut.interpret_single();
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.status, Flags::N);
    cpu_dut.interpret_single();
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.status, Flags::N);

    cpu_dut.interpret_single();
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.status, Flags::Z | Flags::C);
    cpu_dut.interpret_single();
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.status, Flags::C);
    cpu_dut.interpret_single();
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.status, Flags::N);

    cpu_dut.interpret_single();
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.status, Flags::Z | Flags::C);
    cpu_dut.interpret_single();
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.status, Flags::C);
    cpu_dut.interpret_single();
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.status, Flags::N);
}       

#[test]
fn test_inc_inx_iny_dec_dex_dey() -> () {
    let mut cpu_dut = cpu::new();
    let memory: Vec<u8> = vec![0xA2, 0x00, 0xE8, //LDX, INX
                                0xCA, 0xCA, //DEX, DEX
                                0xA0, 0xFF, 0x88, //LDY, DEY
                                0xC8, 0xC8, //INY, INY
        ];
    cpu_dut.load_rom(memory);
    
    cpu_dut.interpret_single();
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.register_x, 1);
    assert_eq!(cpu_dut.status, Flags::empty());
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.register_x, 0);
    assert_eq!(cpu_dut.status, Flags::Z);
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.register_x, 0xFFu8 as i8);
    assert_eq!(cpu_dut.status, Flags::N);

    cpu_dut.interpret_single();
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.register_y, 0xFEu8 as i8);
    assert_eq!(cpu_dut.status, Flags::N);
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.register_y, 0xFFu8 as i8);
    assert_eq!(cpu_dut.status, Flags::N);
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.register_y, 0);
    assert_eq!(cpu_dut.status, Flags::Z);
}

#[test]
fn test_asl_lsr_rol_ror() -> () {
    let mut cpu_dut = cpu::new();
    let memory: Vec<u8> = vec![0x0A, 0x0A, //ASL
                                0x4A, 0x4A, //LSR
        ];
    cpu_dut.load_rom(memory);

    cpu_dut.accumulator = 0b11110000u8 as i8;
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.accumulator, 0b11100000u8 as i8);
    assert_eq!(cpu_dut.status, Flags::N | Flags::C);
    cpu_dut.accumulator = 0b01110000;
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.accumulator, 0b11100000u8 as i8);
    assert_eq!(cpu_dut.status, Flags::N);

    cpu_dut.accumulator = 0b1111;
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.accumulator, 0b111);
    assert_eq!(cpu_dut.status, Flags::C);
    cpu_dut.accumulator = 0b1110;
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.accumulator, 0b111);
    assert_eq!(cpu_dut.status, Flags::empty());
}

#[test]
fn test_jmp() -> () {
    let mut cpu_dut = cpu::new();
    let memory: Vec<u8> = vec![0xA5, 0x00, //LDA
                                0x4C, 0x05, 0x06,//JMP
                                0xA5, 0x00, //LDA
                                0x4C, 0x00, 0x06,//JMP
        ];
    cpu_dut.load_abs(0x0600, memory);
    cpu_dut.program_counter = 0x0600;

    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.program_counter, 0x0602);
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.program_counter, 0x0605);
    cpu_dut.interpret_single();
    cpu_dut.interpret_single();
    assert_eq!(cpu_dut.program_counter, 0x0600);
}

#[test]
fn test_jsr_rst() -> () {
    let mut cpu_dut = cpu::new();
    let memory: Vec<u8> = vec![0xA2, 0x00,       //LDX
                                0xA0, 0x00,       //LDY
                                0x20, 0x09, 0x06, //JSR SR1
                                0xA2, 0xAA,       //LDX
                                0xE8,             //INX : SR1/609
                                0x20, 0x0F, 0x06, //JSR SR2
                                0xC8,             //INY
                                0x60,             //RTS
                                0xE8,             //INX : SR2/60F
                                0x60,];           //RTS
    cpu_dut.load_abs(0x0600, memory);
    cpu_dut.program_counter = 0x0600;
    cpu_dut.stack_pointer = 0xFF;

    cpu_dut.interpret_single(); //LDX
    cpu_dut.interpret_single(); //LDY
    cpu_dut.interpret_single(); //JSR SR1
    assert_eq!(cpu_dut.program_counter, 0x0609);
    cpu_dut.interpret_single(); //INX
    assert_eq!(cpu_dut.register_x, 1);
    cpu_dut.interpret_single(); //JSR SR2

    assert_eq!(cpu_dut.program_counter, 0x060F);
    cpu_dut.interpret_single(); //INX
    assert_eq!(cpu_dut.register_x, 2);
    cpu_dut.interpret_single(); //RTS SR2
    cpu_dut.interpret_single(); //INY
    assert_eq!(cpu_dut.register_y, 1);
    cpu_dut.interpret_single(); //RTS SR1
    cpu_dut.interpret_single(); //LDX
    assert_eq!(cpu_dut.register_x, 0xAAu8 as i8);
}

#[test]
fn test_branches() -> () {
    let mut cpu_dut = cpu::new();
    let memory: Vec<u8> = vec![0xA9, 0x10,       //LDA
                                0xF0, 0x06,       //BEQ
                                0xA9, 0x00,       //LDA
                                0xF0, 0xFA,       //BEQ
                                0xA9, 0xF0,       //LDA
                                0xA9, 0x0F,];     //LDA
                                
    cpu_dut.load_abs(0x0600, memory);
    cpu_dut.program_counter = 0x0600;
    cpu_dut.stack_pointer = 0xFF;

    cpu_dut.interpret_single(); //LDA
    cpu_dut.interpret_single(); //BEQ
    assert_eq!(cpu_dut.program_counter, 0x0604);
    cpu_dut.interpret_single(); //LDA
    cpu_dut.interpret_single(); //BEQ
    assert_eq!(cpu_dut.program_counter, 0x0602);
    cpu_dut.interpret_single(); //BEQ
    assert_eq!(cpu_dut.program_counter, 0x060a);
    cpu_dut.interpret_single(); //LDA
    assert_eq!(cpu_dut.accumulator, 0x0F);
}