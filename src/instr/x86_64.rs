#![allow(non_snake_case, non_camel_case_types)]

use std::mem::transmute;

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

pub enum Mode {
    Direct, // reg3 operates normal
    Memory(i32) // reg3 spcifies memory base. index, offset
}
pub enum Prefix {
    P_0F
}
pub type Opcode = (Prefix, u8);

pub mod op {
    use super::Opcode;
    use super::Prefix::*;
    pub const ADD: Opcode = (P_0F, 0x58);
    pub const MUL: Opcode = (P_0F, 0x59);
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

    pub fn vex(&mut self, (prefix, op): Opcode, reg1: u8, reg2: u8, reg3: u8, mode: Mode) {
        let R = reg1 & 8 != 0;
        let B = reg3 & 8 != 0;
        let X = false;
        let L = true; // 256bit mode
        let W = false;
        let pp = 0;
        let m = match prefix {
            Prefix::P_0F => 0b00001
        };
        
        if X | B {
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
            Mode::Direct => self.push(0b11 << 6 | sip),
            Mode::Memory(0) => self.push(0b00 << 6 | sip),
            Mode::Memory(off) if off >= -128 && off < 128 => {
                self.push(0b01 << 6 | sip);
                self.push(off as u8);
            },
            Mode::Memory(off) => {
                self.push(0b10 << 6 | sip);
                self.pushq(off as u32);
            }
        }
    }
}
        
#[test]
fn test_opcodes() {
    use instr::x86_64::Reg::*;
    
    let mut w = Writer::new();
    w.vex(op::ADD, 0,  0, 0, Mode::Direct);
    w.vex(op::ADD, 1,  0, 0, Mode::Direct);
    w.vex(op::MUL, 8,  0, 0, Mode::Direct);
    w.vex(op::MUL, 0, 15, 0, Mode::Direct);
    w.vex(op::MUL, 0,  0, 1, Mode::Direct);
    w.vex(op::MUL, 0,  0, 8, Mode::Direct);
    w.vex(op::MUL, 0,  0, 15, Mode::Direct);
    w.vex(op::ADD, 0,  0, RDI as u8, Mode::Memory(0));
    w.vex(op::ADD, 0,  0, RDI as u8, Mode::Memory(4));
    w.vex(op::ADD, 0,  0, RBP as u8, Mode::Memory(128));
    
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
        0xc3                    // ret
    ];

    println!("");
    for (i, (&a, &b)) in a.iter().zip(b.iter()).enumerate() {
        println!("{:4x}  {:02x} ({:08b})  {:02x} ({:08b})  {}", i, a, a, b, b, if a == b {"✓"} else {""});
    }
    assert_eq!(a, b);
}
