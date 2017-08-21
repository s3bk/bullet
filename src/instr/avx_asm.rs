use std::fmt;
use std::cmp::PartialEq;
use lang::parse_Expr;
use instr::{Assembler, Vm};
use proc_macro::TokenStream;
use quote::Ident;

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ymm{}", u8::from(*self))
    }
}
#[derive(Copy, Clone)]
enum Value {
    Owned(u8),
    Shared(u8)
}
impl Value {
    fn is_owned(&self) -> bool {
        match *self {
            Value::Owned(_) => true,
            Value::Shared(_) => false
        }
    }
}
impl PartialEq for Value {
    fn eq(&self, rhs: &Value) -> bool {
        u8::from(*self) == u8::from(*rhs)
    }
}
impl From<Value> for u8 {
    fn from(v: Value) -> u8 {
        match v {
            Value::Owned(r) | Value::Shared(r) => r
        }
    }
}
struct AvxAsm {
    lines: Vec<String>,
    free: Vec<u8>,
    used: u8,
    inputs: Vec<(Ident, u8)>,
    consts: Vec<f32>
}
impl AvxAsm {
    fn new() -> AvxAsm {
        AvxAsm {
            free: (0..16).rev().collect(),
            lines: vec![],
            used: 0,
            inputs: vec![],
            consts: vec![]
        }
    }
    fn alloc(&mut self) -> Value {
        let reg = self.free.pop().expect("out of registers");
        if reg > self.used {
            self.used = reg;
        }
        Value::Owned(reg)
    }
    fn drop(&mut self, v: Value) {
        match v {
            Value::Owned(reg) => {
                assert_eq!(self.free.contains(&reg), false, "already free");
                self.free.push(reg);
                self.free.sort_by(|a, b| b.cmp(a));
            },
            Value::Shared(_) => {}
        }
    }
    
    fn get_owned(&mut self, parts: &[Value]) -> Value {
        // well will free all registers in this process
        // allocate the lowest one for the return value
        parts.iter()
            .filter(|v| v.is_owned()).min_by_key(|&&v| u8::from(v)).cloned()
            .unwrap_or_else(|| {
                let first = parts.get(0).expect("empty sum").clone();
                let reg = self.alloc();
                self.lines.push(format!("vmovdqa {}, {};\n", reg, first));
                reg
            })
    }

}
impl Vm for AvxAsm {
    type Var = Value;
    type Storage = u8;
    
    fn make_const(&mut self, c: f32) -> Self::Var {
        let idx = self.consts.len();
        self.consts.push(c);
        let reg = self.alloc();
        self.lines.push(format!("vbroadcastss {}, [rdi+{}];\n", reg, 4*idx));
        reg
    }
    fn make_source(&mut self, name: &str) -> Self::Var {
        let r = self.alloc();
        self.inputs.push((name.into(), r.into()));
        r
    }

    fn make_sum(&mut self, parts: Vec<Self::Var>) -> Self::Var {
        let r_sum = self.get_owned(&parts);
        for part in parts {
            if part != r_sum { // we already started with r_sum
                self.drop(part);
                self.lines.push(format!("vaddps {}, {}, {};\n", r_sum, r_sum, part));
            }
        }

        r_sum
    }
    fn make_product(&mut self, parts: Vec<Self::Var>) -> Self::Var {
        let r_prod = self.get_owned(&parts);

        for part in parts {
            if part != r_prod {
                self.drop(part);
                self.lines.push(format!("vmulps {}, {}, {};\n", r_prod, r_prod, part));
            }
        }

        r_prod
    }
    fn store(&mut self, var: Self::Var) -> Self::Storage {
        match var {
            Value::Owned(reg) => reg,
            Value::Shared(reg) => reg
        }
    }
    fn load(&mut self, storage: &Self::Storage) -> Self::Var {
        Value::Shared(*storage)
    }
    fn forget(&mut self, storage: Self::Storage) {
        self.drop(Value::Owned(storage));
    }
}

pub fn math_avx(input: TokenStream) -> TokenStream {
    let node = parse_Expr(&input.to_string()).expect("failed to parse")
        .to_node().expect("can't convert to node");

    let mut asm = AvxAsm::new();
    let r_out = Assembler::run(&mut asm, &node);
    let content: String = asm.lines.into_iter().collect();
    let s_out = format!("={{{}}}", r_out);
    let s_in = asm.inputs.iter().map(|&(ref var, reg)| {
        let s = format!("{{ymm{}}}", reg);
        quote! { #s (#var) }
    });
    let s_consts = asm.consts.iter().map(|c| quote! { #c });
    let s_clobber = (0 .. asm.used).map(|r| format!("{{ymm{}}}", r));
    let num_consts = asm.consts.len();
    let out = quote! { unsafe {
            let out: f32x8;
            static CONSTANTS: [f32; #num_consts] = [ #( #s_consts ),* ];
            asm!{ #content : #s_out(out) : "{rdi}"(CONSTANTS.as_ptr()), #( #s_in ),* : : "intel" : #( #s_clobber ),* }
            out
        }
    };
    use std::fs::File;
    use std::io::Write;
    writeln!(File::create("/tmp/out").unwrap(), "{}", out).unwrap();
    
    out.parse().expect("failed to parse output")
}
