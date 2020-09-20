use prelude::*;
use packed_simd::f32x8;
use vm::simd::{SimdAsm, Source, Instr};
use compiler::Compiler;
use vm::{Round, Cmp};
use rt::x86_64::{Writer, op, Mode, Reg};
use memmap::{Mmap, MmapOptions};
use vm::simd::Reg as SimdReg;


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
        
        unsafe { llvm_asm!{
            "call rax"
          : // no outputs
          : "{rdi}"(self.consts.as_ptr()),
            "{rdx}"(inputs.as_ptr()),
            "{rbx}"(outputs.as_mut_ptr()),
            "{rax}"(self.code.as_ptr())
          :
          : "intel"
          : "{ymm0}", "{ymm1}", "{ymm2}", "{ymm3}", "{ymm4}", "{ymm5}", "{ymm6}", "{ymm7}",
            "{ymm8}", "{ymm9}", "{ymm10}", "{ymm11}", "{ymm12}", "{ymm13}", "{ymm14}", "{ymm15}"
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

        writer.vex(op::WRITE, reg(r), 0, Mode::Memory(Reg::RBX, i as i32 * 32), None);
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
