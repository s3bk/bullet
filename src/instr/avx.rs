use std::fmt;
use instr::{Compiler, Vm};
use node::NodeRc;
use simd::x86::avx::f32x8;
use quote::{Tokens, Ident};
use memmap::{Mmap, Protection};

impl fmt::Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ymm{}", self.0)
    }
}
#[derive(Copy, Clone, PartialEq, Debug)]
struct Reg(u8);
enum Source {
    Reg(Reg),
    Const(u16)
}
enum Instr {
    Add(Reg, Reg, Source),
    Mul(Reg, Reg, Source)
}
struct AvxAsm {
    instr: Vec<Instr>,
    registers: [usize; 16],
    used: u8,
    inputs: Vec<(Ident, Reg)>,
    consts: Vec<f32>
}
impl AvxAsm {
    fn new() -> AvxAsm {
        AvxAsm {
            instr: vec![],
            used: 0,
            inputs: vec![],
            consts: vec![],
            registers: [0; 16]
        }
    }
    fn alloc_uses(&mut self, uses: usize) -> Reg {
        let (r_num, r_uses) = self.registers.iter_mut()
            .enumerate()
            .filter(|&(_, &mut c)| c == 0)
            .next().expect("out of registers");
        *r_uses = uses;
        Reg(r_num as u8)
    }
    fn alloc(&mut self) -> Reg {
        self.alloc_uses(1)
    }
    fn drop(&mut self, r: Reg) {
        self.registers[r.0 as usize] -= 1;
    }
    
    fn fold(&mut self, parts: Vec<Source>, f: &Fn(Reg, Reg, Source) -> Instr) -> Source {
        // get a non-const source
        let (skip, mut r_last) = parts.iter().enumerate().filter_map(|(i, p)| {
            match *p {
                Source::Reg(r) => Some((i, r)),
                _ => None
            }
        }).next().expect("all consts");
                
        for (_, part) in parts.into_iter().enumerate().filter(|&(i, _)| i != skip) {
            self.drop(r_last);
            match part {
                Source::Reg(r) => self.drop(r),
                _ => {}
            }

            let r_acc = self.alloc();
            self.instr.push(f(r_acc, r_last, part));

            
            r_last = r_acc;
        }

        Source::Reg(r_last)
    }

}
impl Vm for AvxAsm {
    type Var = Source;
    type Storage = Reg;
    
    fn make_const(&mut self, c: f64) -> Self::Var {
        let c = c as f32;
        match self.consts.iter().position(|&d| c == d) {
            Some(idx) => Source::Const(idx as u16),
            None => {
                let idx = self.consts.len();
                self.consts.push(c);
                Source::Const(idx as u16)
            }
        }
    }
    fn make_source(&mut self, name: &str) -> Self::Var {
        let r = self.alloc();
        self.inputs.push((name.into(), r));
        Source::Reg(r)
    }
    
    fn make_sum(&mut self, parts: Vec<Self::Var>) -> Self::Var {
        self.fold(parts, &|a, b, c| Instr::Add(a, b, c))
    }
    fn make_product(&mut self, parts: Vec<Self::Var>) -> Self::Var {
        self.fold(parts, &|a, b, c| Instr::Mul(a, b, c))
    }
    fn store(&mut self, var: &mut Self::Var, uses: usize) -> Self::Storage {
        match *var {
            Source::Reg(r) => {
                self.registers[r.0 as usize] += uses;
                r
            },
            Source::Const(_) => panic!("can't store a const")
        }
    }
    fn load(&mut self, storage: &Self::Storage) -> Self::Var {
        Source::Reg(*storage)
    }
}

pub fn asm(node: NodeRc) -> Tokens { 
    let mut asm = AvxAsm::new();
    let r_out = match Compiler::run(&mut asm, &node) {
        Source::Reg(r) => r,
        Source::Const(_) => panic!("returned a const")
    };
    let mut lines = String::new();
    for instr in asm.instr {
        use std::fmt::Write;
        match instr {
            Instr::Add(r0, r1, Source::Reg(r2))    => writeln!(lines, "\tvaddps {}, {}, {}", r0, r1, r2),
            Instr::Add(r0, r1, Source::Const(idx)) => writeln!(lines, "\tvaddps {}, {}, [rdi+32*{}]", r0, r1, idx),
            Instr::Mul(r0, r1, Source::Reg(r2))    => writeln!(lines, "\tvmulps {}, {}, {}", r0, r1, r2),
            Instr::Mul(r0, r1, Source::Const(idx)) => writeln!(lines, "\tvmulps {}, {}, [rdi+32*{}]", r0, r1, idx),
        }.unwrap();
    }

    let s_out = format!("={{{}}}", r_out);
    let s_in = asm.inputs.iter().map(|&(ref var, reg)| {
        let s = format!("{{{}}}", reg);
        quote! { #s (#var) }
    });
    let s_consts = asm.consts.iter().map(|c| quote! { #c });
    let s_clobber = (0 .. asm.used).map(|r| format!("{{{}}}", r));
    let num_consts = asm.consts.len();
    let out = quote! { unsafe {
            let out: f32x8;
            static CONSTANTS: [f32x8; #num_consts] = [ #( f32x8::splat(#s_consts) ),* ];
            asm!{ #lines : #s_out(out) : "{rdi}"(CONSTANTS.as_ptr()), #( #s_in ),* : : "intel" : #( #s_clobber ),* }
            out
        }
    };
    {
        use std::fs::File;
        use std::io::Write;
        writeln!(File::create("/tmp/out").unwrap(), "{}", out).unwrap();
    }
    out
}

pub struct Code {
    consts: Vec<f32x8>,
    mmap: Mmap
}
impl Code {
    pub fn call1(&self, v0: f32x8) -> f32x8 {
        unsafe {
            let r;
            asm!{
                "call rax"
                    : "={ymm0}"(r)
                    : "{ymm0}"(v0), "{rdi}"(self.consts.as_ptr()), "{rax}"(self.mmap.ptr())
                    :
                    : "intel"
                    : "{ymm0}", "{ymm1}", "{ymm2}", "{ymm3}", "{ymm4}", "{ymm5}", "{ymm6}", "{ymm7}"
            };
            r  
        }
    }
}

pub fn jit(node: NodeRc) -> Code {
    use instr::x86_64::{Writer, Mode, op};

    let mut asm = AvxAsm::new();
    let r_out = match Compiler::run(&mut asm, &node) {
        Source::Reg(r) => r,
        Source::Const(_) => panic!("returned a const")
    };
    assert_eq!(r_out, Reg(0));


    let mut writer = Writer::new();
    for instr in asm.instr {
        match instr {
            Instr::Add(r0, r1, Source::Reg(r2)) => writer.vex(op::ADD, r0.0, r1.0, r2.0, Mode::Direct),
            Instr::Add(r0, r1, Source::Const(idx)) => writer.vex(op::ADD, r0.0, r1.0, 7, Mode::Memory(idx as i32 * 32)),
            Instr::Mul(r0, r1, Source::Reg(r2)) => writer.vex(op::MUL, r0.0, r1.0, r2.0, Mode::Direct),
            Instr::Mul(r0, r1, Source::Const(idx)) => writer.vex(op::MUL, r0.0, r1.0, 7, Mode::Memory(idx as i32 * 32)),
        }
    }

    let code = writer.finish();
    let mut anon_mmap = Mmap::anonymous(4096, Protection::ReadWrite).unwrap();
    unsafe {
        anon_mmap.as_mut_slice()[.. code.len()].copy_from_slice(&code);
    }
    anon_mmap.set_protection(Protection::ReadExecute).unwrap();

    Code {
        mmap: anon_mmap,
        consts: asm.consts.iter().map(|&c| f32x8::splat(c)).collect()
    }
}

    
