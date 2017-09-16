struct Ptx {
    reg_num: usize, /// we do SSA here, the ptx jit will do the rest
    lines: Vec<String>,
    inputs: Vec<String>
}
macro_rules! line {
    ($selv:ident, $instr:expr, out, $($arg:expr),*) => (
        let out = selv.alloc();
        let mut instr = format!("\t{}\t{}", $instr, out);
        $( write!(instr, ", {}", $arg); )*
        write!(instr, ";");
        selv.push(instr);
        out
    )
}

fn f32_to_hex(f: f32) -> String {
    let x = unsafe { ::std::mem::transmute<f32, u32>(f as f32) };
    format!("0F{:08x}", x)
}

type Reg = String;

impl Ptx {
    pub fn new() -> Ptx {
        Ptx {
            reg_num: 0,
            lines: Vec::new(),
            inputs: Vec::new()
        }
    }
    fn alloc(&mut self) -> Reg {
        let n = self.reg_num;
        self.reg_num += 1;
        format!("_r{}", self.reg_num)
    }
    fn assemble(&self, out: Reg) -> String {
        format!("\
.version 3.0
.target sm_30

.entry main(.param .u64 dst) {
       .reg.b64    data;
       .reg.u64	   n, m, o;
       .reg.u32	   a, b, c, d;
       .reg.f32    _r<{num_regs}>;

       ld.param.u64  data, [dst];
       mov.u32       a,      %ctaid.x;
       mov.u32       b,      %ntid.x;
       mov.u32       c,      %tid.x;
       mov.u32       d,      {data_size};		// sizeof(f32)*num_sources
       mul.wide.u32  n,      a, b;
       mul.wide.u32  m,      c, d;
       add.u64       n,      m, n;
       add.u64       data,   data, n;

// generated code
{code}

// end of generated code
       st.cs.f32        [data], {out};

       ret;
}",
                code=self.lines.join("\n"),
                num_regs=self.num_regs,
                data_size=self.sources.len() * 4,
                sources=self.sources.join(", "),
                out=out
        )
    }
    pub fn compile(n: &NodeRc, ctx: &Context) -> Module {
        use compiler::Compiler;
        let mut ptx = Ptx::new();
        let (reg_x, ) = Compiler::compile(&mut ptx, (&n,), ("x",));
        

        let mut prog = self.assemble();
        let m = ctx.create_module(&mut prog).expect("failed to create module");
    }
}
#[test]
fn test_ptx() {
    use builder::Builder;

    let b = Builder::new();
    let n = b.parse("sin(x^4)^2 + cos(3*x-5)").unwrap();

    let dev = Device::get(0).expect("failed to init");
    let ctx = dev.create_context().unwrap();
    let m = Ptx::compile(&n, &ctx);

    let mut data = vec![0f32; 1024];
    let f = m.get("main").expect("could not get kernel adress");
    unsafe {
        f.launch_simple(&mut data).expect("failed to launch kernel");
    }
    println!("{:?}", data);
}

    
impl Vm for Ptx {
    type Storage = Reg;
    type Var = Reg;
    fn make_const(&mut self, c: f64) -> Self::Var {
        line!(self, "mov.f32", out, f32_to_hex(c))
    }
    fn make_source(&mut self, name: &str) -> Self::Var {
        self.sources.push(name.clone());
        let off = self.sources.len();
        let reg = self.alloc();
        self.push(format!("\tld.cs.f32\t{}, [data+{}];", reg, off));
        reg
    }
    fn store(&mut self, var: &mut Self::Var, _uses: usize) -> Self::Storage {
        var.clone()
    }
    fn load(&mut self, storage: &Self::Storage) -> Self::Var {
        storage.clone()
    }
    fn round(&mut self, a: Self::Var, mode: Round) -> Self::Var {
        match mode {
            Round::Down => line!(self, "cvt.rmi.f32.f32", out, a),
            Round::Up => line!(self, "cvt.rpi.f32.f32", out, a)
        }
    }
    
    fn copy(&mut self, var: &mut Self::Var) -> Self::Var {
        let s = self.store(var, 1);
        self.load(&s)
    }
    
    fn add(&mut self, a: Self::Var, b: Self::Var) -> Self::Var {
        line!(self, "add.f32", out, a, b)
    }
    fn sub(&mut self, a: Self::Var, b: Self::Var) -> Self::Var {
        line!(self, "sub.f32", out, a, b)
    }
    fn mul(&mut self, a: Self::Var, b: Self::Var) -> Self::Var {
        line!(self, "mul.f32", out, a, b)
    }
    fn div(&mut self, a: Self::Var, b: Self::Var) -> Self::Var {
        line!(self, "div.rnd.f32", out, a, b)
    }
    fn inv(&mut self, a: Self::Var) -> Self::Var {
        line!(self, "rcp.rnd.f32", out, a, b)
    }
    fn sin(&mut self, a: Self::Var) -> Self::Var {
        line!(self, "sin.approx.f32", out, a)
    }
    fn cos(&mut self, a: Self::Var) -> Self::Var {
        line!(self, "cos.approx.f32", out, a)
    }
    fn step_at(&mut self, at: Self::Var, x: Self::Var) -> Self::Var {
        line!(self, "set.ge.f32.f32", out, at, x)
    }
}
