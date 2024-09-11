#![allow(non_snake_case, non_camel_case_types)]

use std::mem::transmute;

#[derive(Copy, Clone)]
pub enum Reg {
    RAX,
    RCX,
    RDX,
    RBX,
    RSP,
    RBP,
    RSI,
    RDI
}

#[derive(Copy, Clone)]
pub enum Mode {
    Direct(u8), // reg3 operates normal
    Memory(Reg, i32) // reg3 spcifies memory base. index, offset
}
#[derive(PartialEq)]
pub enum Prefix {
    P_0F,
    P_0F_38,
    P_0F_3A
}
pub enum SimdPrefix {
    None,
    S_66,
    S_F3,
    S_F2
}
pub type Opcode = (SimdPrefix, Prefix, u8);

pub mod op {
    use super::Opcode;
    use super::Prefix::*;
    use super::SimdPrefix::*;
    
    pub const MOV: Opcode = (None, P_0F, 0x28);
    pub const ADD: Opcode = (None, P_0F, 0x58);
    pub const SUB: Opcode = (S_66, P_0F, 0x5C);
    pub const MUL: Opcode = (None, P_0F, 0x59);
    pub const DIV: Opcode = (None, P_0F, 0x5E);
    pub const RECIP: Opcode = (None, P_0F, 0x53);
    pub const ROUND: Opcode = (S_66, P_0F_3A, 0x08);
    pub const READ: Opcode = (S_66, P_0F, 0x6F);
    pub const WRITE: Opcode = (S_66, P_0F, 0x7F);
    pub const CMP: Opcode = (S_66, P_0F, 0xC2);
    pub const MASKREAD: Opcode = (S_66, P_0F_38, 0x2C);
}
pub struct Writer {
    buf: Vec<u8>
}
impl Writer {
    pub fn new() -> Writer {
        Writer { buf: Vec::with_capacity(256) }
    }
    pub fn finish(mut self) -> Vec<u8> {
        self.push(0xc3); // ret
        self.buf
    }
    fn push(&mut self, b: u8) {
        self.buf.push(b);
    }
    fn pushq(&mut self, b: u32) {
        unsafe { self.buf.extend_from_slice(&transmute::<u32, [u8; 4]>(b)) } // this is fine!
    }

    pub fn vex(&mut self, (simd, prefix, op): Opcode, reg1: u8, reg2: u8, mode: Mode, imm8: Option<u8>) {
        let reg3 = match mode {
            Mode::Direct(r) => r,
            Mode::Memory(r, _) => r as u8
        };
        let R = reg1 & 8 != 0;
        let B = reg3 & 8 != 0;
        let X = false;
        let L = true; // 256bit mode
        let W = false;
        let pp = match simd {
            SimdPrefix::None => 0b00,
            SimdPrefix::S_66 => 0b01,
            SimdPrefix::S_F3 => 0b10,
            SimdPrefix::S_F2 => 0b11
        };
        let m = match prefix {
            Prefix::P_0F    => 0b00001,
            Prefix::P_0F_38 => 0b00010,
            Prefix::P_0F_3A => 0b00011
        };
        
        if X | B || prefix != Prefix::P_0F {
            self.push(0xc4);
            self.push(((!R as u8) << 7) | ((!X as u8) << 6) | (!(B as u8) << 5) | m);
            self.push((W as u8) << 7 | (0xf ^ reg2) << 3 | (L as u8) << 2 | pp);
        } else {
            self.push(0xc5);
            self.push((!R as u8) << 7 | (0xf ^ reg2) << 3 | (L as u8) << 2 | pp);
        }
        self.push(op);
        
        let sip = ((reg1 & 7) << 3) | (reg3 & 7);
        match mode {
            Mode::Direct(_) => self.push(0b11 << 6 | sip),
            Mode::Memory(_, 0) => self.push(0b00 << 6 | sip),
            Mode::Memory(_, off) if off >= -128 && off < 128 => {
                self.push(0b01 << 6 | sip);
                self.push(off as u8);
            },
            Mode::Memory(_, off) => {
                self.push(0b10 << 6 | sip);
                self.pushq(off as u32);
            }
        }
        if let Some(imm) = imm8 {
            self.push(imm);
        }
    }
}
        
#[test]
fn test_opcodes() {
    use self::Reg::*;
    
    let mut w = Writer::new();
    w.vex(op::ADD, 0,  0, Mode::Direct(0), None);
    w.vex(op::ADD, 1,  0, Mode::Direct(0), None);
    w.vex(op::MUL, 8,  0, Mode::Direct(0), None);
    w.vex(op::MUL, 0, 15, Mode::Direct(0), None);
    w.vex(op::MUL, 0,  0, Mode::Direct(1), None);
    w.vex(op::MUL, 0,  0, Mode::Direct(8), None);
    w.vex(op::MUL, 0,  0, Mode::Direct(15), None);
    w.vex(op::ADD, 0,  0, Mode::Memory(RDI, 0), None);
    w.vex(op::ADD, 0,  0, Mode::Memory(RDI, 4), None);
    w.vex(op::ADD, 0,  0, Mode::Memory(RBP, 128), None);
    w.vex(op::ROUND, 0, 0, Mode::Direct(0), Some(9));
    
    let a = w.finish();
    let b = vec![
        0xc5, 0xfc, 0x58, 0xc0, // vaddps ymm0,ymm0,ymm0
        0xc5, 0xfc, 0x58, 0xc8, // vaddps ymm1,ymm0,ymm0
        0xc5, 0x7c, 0x59, 0xc0, // vmulps ymm8,ymm0,ymm0,
        0xc5, 0x84, 0x59, 0xc0, // vmulps ymm0,ymm15,ymm0
        0xc5, 0xfc, 0x59, 0xc1, // vmulps ymm0,ymm0,ymm1
        0xc4, 0xc1, 0x7c, 0x59, 0xc0, // vmulps ymm0,ymm0,ymm8
        0xc4, 0xc1, 0x7c, 0x59, 0xc7, // vmulps ymm0,ymm0,ymm15
        0xc5, 0xfc, 0x58, 0x07, // vaddps ymm0,ymm0,YMMWORD PTR [rdi]
        0xc5, 0xfc, 0x58, 0x47, 0x04, // vaddps ymm0,ymm0,YMMWORD PTR [rdi+0x4]
        0xc5, 0xfc, 0x58, 0x85, 0x80, 0x00, 0x00, 0x00, // vaddps ymm0,ymm0,YMMWORD PTR [rbp+0x80]
        0xc4, 0xe3, 0x7d, 0x08, 0xc0, 0x09, // vroundps ymm0,ymm0,0x9
        0xc3                    // ret
    ];

    println!("");
    for (i, (&a, &b)) in a.iter().zip(b.iter()).enumerate() {
        println!("{:4x}  {:02x} ({:08b})  {:02x} ({:08b})  {}", i, a, a, b, b, if a == b {"✓"} else {""});
    }
    assert_eq!(a, b);
}
