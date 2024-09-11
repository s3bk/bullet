use crate::prelude::*;
use crate::vm::simd::{SimdAsm, Source, Instr};
use crate::compiler::Compiler;
use crate::vm::{Round, Cmp, simd::Reg as SimdReg};
use crate::rt::x86_64::{Writer, op, Mode, Reg};
use memmap::{Mmap, MmapOptions};
use std::simd::f32x8;
use std::arch::asm;

pub struct Code {
    consts: Vec<f32x8>,
    code: Mmap,
    pub num_inputs: usize,
    pub num_outputs: usize,
}
impl Code {
    #[inline(always)]
    pub fn call(&self, inputs: &[f32x8], outputs: &mut [f32x8]) {
        assert_eq!(self.num_inputs, inputs.len());
        assert_eq!(self.num_outputs, outputs.len());
        
        unsafe { asm!{
            "call rax",
            in("rdi") self.consts.as_ptr(),
            in("rdx") inputs.as_ptr(),
            in("rcx") outputs.as_mut_ptr(),
            in("rax") self.code.as_ptr(),
            out("ymm0") _,
            out("ymm1") _,
            out("ymm2") _,
            out("ymm3") _,
            out("ymm4") _,
            out("ymm5") _,
            out("ymm6") _,
            out("ymm7") _,
            out("ymm8") _,
            out("ymm9") _,
            out("ymm10") _,
            out("ymm11") _,
            out("ymm12") _,
            out("ymm13") _,
            out("ymm14") _,
            out("ymm15") _
        } }
    }
    
    pub fn bench(&self, inputs: &[f32x8], outputs: &mut [f32x8], n: usize) {
        assert_eq!(self.num_inputs, inputs.len());
        assert_eq!(self.num_outputs, outputs.len());

        for _ in 0 .. n {
            self.call(inputs, outputs);
        }
    }
}

pub fn compile(nodes: &[NodeRc], vars: &[&str]) -> Result<Code, Error>
{
    let mut asm = SimdAsm::new();
    let outputs = Compiler::compile(&mut asm, nodes, vars)?;

    let reg = |r: SimdReg| r.0;
    let mode = |s: Source| match s {
        Source::Reg(r) => Mode::Direct(reg(r)),
        Source::Const(idx) => Mode::Memory(Reg::RDI, idx as i32 * 32),
        Source::Input(idx) => Mode::Memory(Reg::RDX, idx as i32 * 32),
    };

    let mut writer = Writer::new();
    for instr in asm.instr.iter() {
        match *instr {
            //Instr::Move(r0, s)         => writer.vex(op::MOV,   reg(r0), 0,       mode(s), None),
            Instr::Add(r0, r1, s)      => writer.vex(op::ADD,   reg(r0), reg(r1), mode(s), None),
            Instr::Sub(r0, r1, s)      => writer.vex(op::SUB,   reg(r0), reg(r1), mode(s), None),
            Instr::Mul(r0, r1, s)      => writer.vex(op::MUL,   reg(r0), reg(r1), mode(s), None),
            Instr::Div(r0, r1, s)      => writer.vex(op::DIV,   reg(r0), reg(r1), mode(s), None),
            Instr::Inv(r0, s)          => writer.vex(op::RECIP, reg(r0), 0,       mode(s), None),
            Instr::Round(r0, s, dir)   => writer.vex(op::ROUND, reg(r0), 0,       mode(s), Some(match dir {
                Round::Down => 0x9,
                Round::Up => 0xA
            })),
            Instr::Load(r0, s)         => writer.vex(op::READ,  reg(r0), 0,       mode(s), None),
            Instr::Cmp(r0, r1, s, ord) => writer.vex(op::CMP,   reg(r0), reg(r1),   mode(s), Some(match ord {
                Cmp::EQ => 0x0,
                Cmp::NE => 0xC,
                Cmp::LT => 0x11,
                Cmp::LE => 0x12,
                Cmp::GT => 0x1E,
                Cmp::GE => 0x1D
            })),
            Instr::MaskMove(r0, r1, s) => writer.vex(op::MASKREAD, reg(r0), reg(r1), mode(s), None)
        }
    }
    for (i, &r) in outputs.iter().enumerate() {
        let r = match r {
            Source::Reg(r) => r,
            s => {
                let r = asm.alloc();
                asm.push(Instr::Load(r, s)); // write the source to the output register
                r
            }
        };

        writer.vex(op::WRITE, reg(r), 0, Mode::Memory(Reg::RCX, i as i32 * 32), None);
        asm.drop(r);
    }
    
    println!("{:?}", asm.registers);
    let code = writer.finish();
    /*{
        use std::fs::File;
        use std::io::Write;
        File::create("/tmp/out").unwrap().write_all(&code).unwrap();
    }*/

    let mut anon_mmap = MmapOptions::new()
        .len(4096)
        .map_anon().unwrap();
    
    anon_mmap[.. code.len()].copy_from_slice(&code);
    let mmap = anon_mmap.make_exec().unwrap();
    
    Ok(Code {
        code: mmap,
        consts: asm.consts.iter().map(|&c| f32x8::splat(c)).collect(),
        num_inputs: asm.inputs.len(),
        num_outputs: outputs.len()
    })
}
