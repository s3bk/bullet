use std::fmt;
use compiler::Compiler;
use vm::{Vm, Round};
use node::NodeRc;
use quote::{Tokens, Ident};
use tuple::{TupleElements, Map};

mod jit;

#[allow(dead_code)]
mod x86_64;

pub use self::jit::avx_jit;

impl fmt::Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ymm{}", self.0)
    }
}
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Reg(u8);
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Source {
    Reg(Reg),
    Const(i32),
    Input(i32)
}
impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Source::Reg(r) => r.fmt(f),
            Source::Const(idx) => write!(f, "[rdi+{}*32]", idx),
            Source::Input(idx) => write!(f, "[rdx+{}*32]", idx)
        }
    }
}

#[derive(Debug)]
enum Instr {
    Add(Reg, Reg, Source),
    Sub(Reg, Reg, Source),
    Mul(Reg, Reg, Source),
    Round(Reg, Source, Round),
    Load(Reg, Source),
}
struct AvxAsm {
    instr: Vec<Instr>,
    registers: [usize; 16],
    used: u8,
    inputs: Vec<Ident>,
    consts: Vec<f32>
}
impl AvxAsm {
    fn new() -> AvxAsm {
        AvxAsm {
            instr: vec![],
            used: 0,
            inputs: vec![],
            consts: vec![],
            registers: [0; 16],
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
    fn drop_s(&mut self, s: Source) {
        match s {
            Source::Reg(r) => self.drop(r),
            _ => {}
        }
    }
    fn push(&mut self, i: Instr) {
        self.instr.push(i);
    }
    fn fold(&mut self, mut parts: Vec<Source>, f: &Fn(Reg, Reg, Source) -> Instr) -> Source {
        // get a non-const source
        let (skip, mut r_last) = parts.iter().enumerate().filter_map(|(i, p)| {
            match *p {
                Source::Reg(r) => Some((i, r)),
                _ => None
            }
        }).next().unwrap_or_else(|| {
            // dang it! we need to load one
            let r = self.alloc();
            self.push(Instr::Load(r, parts.pop().unwrap()));
            (parts.len(), r)
        });
                
        for (_, part) in parts.into_iter().enumerate().filter(|&(i, _)| i != skip) {
            self.drop(r_last);
            self.drop_s(part);
            let r_acc = self.alloc();
            self.push(f(r_acc, r_last, part));

            
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
            Some(idx) => Source::Const(idx as i32),
            None => {
                let idx = self.consts.len();
                self.consts.push(c);
                Source::Const(idx as i32)
            }
        }
    }
    fn make_source(&mut self, name: &str) -> Self::Var {
        let idx = self.inputs.len();
        self.inputs.push(name.into());
        Source::Input(idx as i32)
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
            s => {
                let r = self.alloc();
                self.push(Instr::Load(r, s));
                r
            }
        }
    }
    fn load(&mut self, storage: &Self::Storage) -> Self::Var {
        Source::Reg(*storage)
    }
    fn round(&mut self, x: Self::Var, mode: Round) -> Self::Var {
        self.drop_s(x);
        let y = self.alloc();
        self.push(Instr::Round(y, x, mode));
        Source::Reg(y)
    }
}

pub fn avx_asm<'a, N, V>(nodes: N, vars: V) -> Tokens
    where N: TupleElements<Element=&'a NodeRc> + Map<Source>,
          V: TupleElements<Element=&'a str>
{ 
    let mut asm = AvxAsm::new();
    let out = Compiler::compile(&mut asm, nodes, vars);

    let mut lines = String::new();
    for instr in asm.instr {
        use std::fmt::Write;
        match instr {
            Instr::Add(r0, r1, s)            => writeln!(lines, "\tvaddps {}, {}, {}", r0, r1, s),
            Instr::Sub(r0, r1, s)            => writeln!(lines, "\tvsubps {}, {}, {}", r0, r1, s),
            Instr::Mul(r0, r1, s)            => writeln!(lines, "\tvmulps {}, {}, {}", r0, r1, s),
            Instr::Round(r0, s, Round::Up)   => writeln!(lines, "\tvroundps {}, {}, 0x0A", r0, s),
            Instr::Round(r0, s, Round::Down) => writeln!(lines, "\tvroundps {}, {}, 0x09", r0, s),
            Instr::Load(r0, s)               => writeln!(lines, "\tvmovdqa {}, {}", r0, s),
        }.unwrap();
    }
    let num_inputs = asm.inputs.len();
    let num_consts = asm.consts.len();
    let s_inputs = asm.inputs;
    let mut def_out = vec![]; // defines
    let mut reg_out = vec![]; // registers
    let mut vars = vec![]; // the names
    for (i, src) in out.into_elements().enumerate() {
        let v: Ident = format!("out_{}", i).into();
        def_out.push(match src {
            Source::Reg(r) => {
                let reg = format!("={{{}}}", r);
                reg_out.push(quote!{ #reg(#v) });
                quote!{ let #v: f32x8; }
            },
            Source::Input(idx) => {
                let n = idx as usize / 32;
                quote!{ let #v: f32x8 = inputs[#n]; }
            },
            Source::Const(idx) => {
                let n = idx as usize / 32;
                quote!{ let #v: f32x8 = CONSTANTS[#n]; }
            }
        });
        vars.push(v);
    }
    let s_consts = asm.consts.iter().map(|c| quote! { #c });
    let s_clobber = (0 .. asm.used).map(|r| format!("{{{}}}", r));

    let out = quote! { unsafe {
        let inputs: &[f32x8; #num_inputs] = &[ #( #s_inputs ),* ];
        static CONSTANTS: [f32x8; #num_consts] = [ #( f32x8::splat(#s_consts) ),* ];
        #( #def_out )*
        asm!{ #lines : #reg_out(out) : "{rdi}"(CONSTANTS.as_ptr()), "{rdx}"(inputs.as_ptr()) : : "intel" : #( #s_clobber ),* }
        ( #( #vars ),* )
    } };
    {
        use std::fs::File;
        use std::io::Write;
        writeln!(File::create("/tmp/out.asm").unwrap(), "{}", out).unwrap();
    }
    out
}
