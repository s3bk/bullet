use simd::x86::avx::f32x8;
use tuple::{Map, TupleElements};
use super::{AvxAsm, Source, Instr};
use compiler::Compiler;
use vm::Round;
use std::marker::PhantomData;
use node::NodeRc;
use avx::x86_64::{Writer, op, Mode, Reg};
use memmap::{Mmap, Protection};
use avx;


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

fn mode(s: Source) -> Mode {
    match s {
        Source::Reg(r) => Mode::Direct(r.0),
        Source::Const(idx) => Mode::Memory(Reg::RDI, idx as i32 * 32),
        Source::Input(idx) => Mode::Memory(Reg::RDX, idx as i32 * 32),
    }
}

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

        if r.0 as usize != num_results {
            renames.swap(r.0 as usize, num_results);
        }

        num_results += 1;
    });
    assert_eq!(num_results, F::N);

    // constant expressions will not allocate registers

    let reg = |r: avx::Reg| renames[r.0 as usize];
    let mut writer = Writer::new();
    for instr in asm.instr.iter() {
        match *instr {
            Instr::Add(r0, r1, s) => writer.vex(op::ADD, reg(r0), reg(r1), mode(s), None),
            Instr::Sub(r0, r1, s) => writer.vex(op::SUB, reg(r0), reg(r1), mode(s), None),
            Instr::Mul(r0, r1, s) => writer.vex(op::MUL, reg(r0), reg(r1), mode(s), None),
            Instr::Round(r0, s, Round::Down) => writer.vex(op::ROUND, reg(r0), 0, mode(s), Some(0x9)),
            Instr::Round(r0, s, Round::Up) => writer.vex(op::ROUND, reg(r0), 0, mode(s), Some(0xA)),
            Instr::Load(r0, s) => writer.vex(op::READ, reg(r0), 0, mode(s), None),
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
