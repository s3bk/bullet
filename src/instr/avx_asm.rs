use std::fmt;
use instr::{Assembler, Vm};
use builder::Builder;
use quote::{Tokens, Ident};

impl fmt::Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ymm{}", self.0)
    }
}
#[derive(Copy, Clone, PartialEq)]
struct Reg(u8);
enum Source {
    Reg(Reg),
    Const(u16)
}

struct AvxAsm {
    lines: Vec<String>,
    registers: [usize; 16],
    used: u8,
    inputs: Vec<(Ident, Reg)>,
    consts: Vec<f32>
}
impl AvxAsm {
    fn new() -> AvxAsm {
        AvxAsm {
            lines: vec![],
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
    
    fn fold(&mut self, parts: Vec<Source>, instr: &str) -> Source {
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
            let arg2 = match part {
                Source::Const(idx) => format!("[rdi+32*{}]", idx),
                Source::Reg(r) => format!("{}", r)
            };
            self.lines.push(format!("{} {}, {}, {};\n", instr, r_acc, r_last, arg2));
            
            r_last = r_acc;
        }

        Source::Reg(r_last)
    }

}
impl Vm for AvxAsm {
    type Var = Source;
    type Storage = Reg;
    
    fn make_const(&mut self, c: f32) -> Self::Var {
        let idx = self.consts.len();
        self.consts.push(c);
        Source::Const(idx as u16)
    }
    fn make_source(&mut self, name: &str) -> Self::Var {
        let r = self.alloc();
        self.inputs.push((name.into(), r));
        Source::Reg(r)
    }
    
    fn make_sum(&mut self, parts: Vec<Self::Var>) -> Self::Var {
        self.fold(parts, "vaddps")
    }
    fn make_product(&mut self, parts: Vec<Self::Var>) -> Self::Var {
        self.fold(parts, "vmulps")
    }
    fn store(&mut self, var: Self::Var, uses: usize) -> (Self::Storage, Self::Var) {
        match var {
            Source::Reg(r) => {
                self.registers[r.0 as usize] += uses;
                (r, var)
            },
            Source::Const(_) => panic!("can't store a const")
        }
    }
    fn load(&mut self, storage: &Self::Storage) -> Self::Var {
        Source::Reg(*storage)
    }
}

pub fn math_avx(input: String) -> Tokens {
    let builder = Builder::new();
    let node = builder.parse(&input).expect("failed to parse");

    let mut asm = AvxAsm::new();
    let r_out = match Assembler::run(&mut asm, &node) {
        Source::Reg(r) => r,
        Source::Const(_) => panic!("returned a const")
    };
    let content: String = asm.lines.into_iter().collect();
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
            asm!{ #content : #s_out(out) : "{rdi}"(CONSTANTS.as_ptr()), #( #s_in ),* : : "intel" : #( #s_clobber ),* }
            out
        }
    };
    use std::fs::File;
    use std::io::Write;
    writeln!(File::create("/tmp/out").unwrap(), "{}", out).unwrap();
    
    out
}
