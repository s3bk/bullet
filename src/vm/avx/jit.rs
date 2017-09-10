use simd::x86::avx::f32x8;
use tuple::{Map, TupleElements};
use super::{AvxAsm, Source, Instr};
use compiler::Compiler;
use vm::{Round, Cmp};
use std::marker::PhantomData;
use node::NodeRc;
use super::x86_64::{Writer, op, Mode, Reg};
use memmap::{Mmap, Protection};
use vm::avx;


pub struct Code<V> {
    consts: Vec<f32x8>,
    code: Mmap,
    num_vars: usize,
    _m: PhantomData<V>
}
macro_rules! A { ($A:ty, $B:tt) => ($A) }
macro_rules! impl_call {
    ($($name:ident : $reg:tt),*) => (
        impl Code<($(A!(f32x8, $name),)*)> {
            #[inline(always)]
            pub fn call(&self, v: &[f32x8]) -> ($(A!(f32x8, $name),)*) {
                assert_eq!(v.len(), self.num_vars);
                $( let $name; )*
                unsafe { asm!{
                    "call rax"
                  : $( $reg ($name) ),*
                  : "{rdi}"(self.consts.as_ptr()),
                    "{rdx}"(v.as_ptr()),
                    "{rax}"(self.code.ptr())
                  :
                  : "intel"
                  : "{ymm0}", "{ymm1}", "{ymm2}", "{ymm3}", "{ymm4}", "{ymm5}", "{ymm6}", "{ymm7}",
                    "{ymm8}", "{ymm9}", "{ymm10}", "{ymm11}", "{ymm12}", "{ymm13}", "{ymm14}", "{ymm15}"
                } }
                ( $($name,)* )
            }
            #[inline(always)]
            pub fn bench(&self, v: &[f32x8], n: usize) -> ($(A!(f32x8, $name),)*) {
                assert_eq!(v.len(), self.num_vars);
                $( let $name; )*
                unsafe { asm!{ "
1:  call rax
    loop 1b
"
                  : $( $reg ($name) ),*
                  : "{rdi}"(self.consts.as_ptr()),
                    "{rdx}"(v.as_ptr()),
                    "{rax}"(self.code.ptr()),
                    "{rcx}"(n)
                  :
                  : "intel"
                  : "{ymm0}", "{ymm1}", "{ymm2}", "{ymm3}", "{ymm4}", "{ymm5}", "{ymm6}", "{ymm7}",
                    "{ymm8}", "{ymm9}", "{ymm10}", "{ymm11}", "{ymm12}", "{ymm13}", "{ymm14}", "{ymm15}"
                } }
                ( $($name,)* )
            }
        }
    )
}
impl_call!(r0: "={ymm0}");
impl_call!(r0: "={ymm0}", r1: "={ymm1}");
impl_call!(r0: "={ymm0}", r1: "={ymm1}", r2: "={ymm2}");
impl_call!(r0: "={ymm0}", r1: "={ymm1}", r2: "={ymm2}", r3: "={ymm3}");


pub fn avx_jit<'a, F, V, R>(nodes: F, vars: V) -> Code<<R as Map<f32x8>>::Output>
    where V: TupleElements<Element=&'a str>,
          R: TupleElements<Element=Source> + Map<f32x8> + ::std::fmt::Debug,
          F: TupleElements<Element=&'a NodeRc> + Map<Source, Output=R>
{
    let mut asm = AvxAsm::new();
    let mut num_results = 0;
    let mut renames = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]; // virtual reg -> ymm reg
    Compiler::compile(&mut asm, nodes, vars, |asm, r| {
        // this runs for every result
        let r = match r {
            Source::Reg(r) => r,
            s => {
                let r = asm.alloc();
                asm.push(Instr::Load(r, s)); // write the source to the output register
                r
            }
        };
        //        n r         pos n, r
        // 0 1 2  0:2  0//2  [0][2]
        // 2 1 0  1:0  2//1  [0][1]
        // 1 2 0  2:1        [1][1]
        
        // 2 0 1  2 -> 0
        // 0 2 1  2 -> 1
        // 0 1 2

        // 0 2 1  0 -> 0
        // 0 2 1  2 -> 1
        // 0 1 2
        if renames[r.0 as usize] != num_results as u8 {
            let pos_n = renames.iter().position(|&n| n == num_results as u8).unwrap();
            renames.swap(pos_n, r.0 as usize);
        }
        
        num_results += 1;
    });
    assert_eq!(num_results, F::N);

    // constant expressions will not allocate registers
    for i in 0 .. 16 {
        println!("{:2} -> {:2}", i, renames[i]);
    }
    let reg = |r: avx::Reg| renames[r.0 as usize];
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
    println!("{:?}", asm.registers);
    let code = writer.finish();
    
    {
        use std::fs::File;
        use std::io::Write;
        File::create("/tmp/out").unwrap().write_all(&code).unwrap();
    }

    let mut anon_mmap = Mmap::anonymous(4096, Protection::ReadWrite).unwrap();
    unsafe {
        anon_mmap.as_mut_slice()[.. code.len()].copy_from_slice(&code);
    }
    anon_mmap.set_protection(Protection::ReadExecute).unwrap();
    
    Code {
        code: anon_mmap,
        consts: asm.consts.iter().map(|&c| f32x8::splat(c)).collect(),
        num_vars: V::N,
        _m: PhantomData
    }
}
