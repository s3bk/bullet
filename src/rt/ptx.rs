use prelude::*;
use vm::*;
use std::time::Instant;
use rt::cuda::{Buffer, Device, Context, Module, CudaError};
use std::fmt::Write;
use compiler::Compiler;

struct Ptx {
    num_regs: usize, /// we do SSA here, the ptx jit will do the rest
    lines: Vec<String>,
    inputs: Vec<String>
}
macro_rules! line {
    ($selv:ident, $instr:expr, out, $($arg:expr),*) => (
        {
            let out = $selv.alloc();
            let mut instr = format!("    {:19} {}", $instr, out);
            $( write!(instr, ", {}", $arg).unwrap(); )*
            write!(instr, ";").unwrap();
            $selv.push(instr);
            out
        }
    )
}

fn f32_to_hex(f: f32) -> String {
    let x = unsafe { ::std::mem::transmute::<f32, u32>(f) };
    format!("0F{:08x}", x)
}

#[derive(Debug)]
pub enum PtxError {
    Core(Error),
    Cuda(CudaError)
}

type Reg = String;

impl Ptx {
    pub fn new() -> Ptx {
        Ptx {
            num_regs: 0,
            lines: Vec::new(),
            inputs: Vec::new()
        }
    }
    fn push(&mut self, line: String) {
        self.lines.push(line);
    }
    fn alloc(&mut self) -> Reg {
        let n = self.num_regs;
        self.num_regs += 1;
        format!("_r{}", n)
    }
    fn assemble(&self, out: Vec<Reg>) -> String {
        format!("\
.version 3.0
.target sm_30

.entry main(.param.b64 src, .param.u64 dst) {{
    .reg.u64            data_in, data_out;
    .reg.u64            n, m, o;
    .reg.u32            a, b, c, d;
    .reg.f32            _r<{num_regs}>;

    ld.param.u64        data_in,    [src];
    ld.param.u64        data_out,   [dst];
    mov.u32             a,          %ctaid.x;
    mov.u32             b,          %ntid.x;
    mov.u32             c,          %tid.x;
    mov.u32             d,          {data_size};		// sizeof(f32)*num_sources
    mul.wide.u32        n,          a, b;
    mul.wide.u32        m,          c, d;
    add.u64             n,          m, n;
    add.u64             data_in,    data_in, n;
    add.u64             data_out,   data_out, n;

// generated code
{code}

// end of generated code
    st.cs.f32           [data_out], {out};

    ret;
}}",
                code=self.lines.join("\n"),
                num_regs=self.num_regs,
                data_size=self.inputs.len() * 4,
                out=out[0]
        )
    }
    pub fn compile<'a>(n: NodeRc, ctx: &'a Context) -> Result<Module<'a>, PtxError> {
        let mut ptx = Ptx::new();
        let out = Compiler::compile(&mut ptx, &[n], &["x"]).map_err(|e| PtxError::Core(e))?;

        let mut prog = ptx.assemble(out);
        println!("{}", prog);
        ctx.create_module(&mut prog).map_err(|e| PtxError::Cuda(e))
    }
}

pub fn bench_ptx(n: NodeRc, count: usize) -> f64 {
    let dev = Device::get(0).expect("failed to init");
    let ctx = dev.create_context().unwrap();
    let m = Ptx::compile(n, &ctx).unwrap();

    let mut data_in = Buffer::with_capacity(count).unwrap();
    let mut data_out = Buffer::with_capacity(count).unwrap();
    for _ in 0 .. count {
        data_in.push(0f32);
    }
    data_out.push(-1.0);
    let f = m.get("main").expect("could not get kernel adress");
    let t0 = Instant::now();
    unsafe {
        f.launch_simple(&data_in, &mut data_out).expect("failed to launch kernel");
    }
    let dt = t0.elapsed();

    println!("{} ... {}", data_out[0], data_out[count-1]);
    dt.as_secs() as f64 + dt.subsec_nanos() as f64 * 1e-9
}

#[test]
fn test_ptx() {
    use builder::Builder;
    let b = Builder::new();
    let n = b.parse("sin(x^4)^2 + cos(3*x-5)").unwrap();
    println!("{}ms", 1000. * bench_ptx(n, 1024*1024));
}
    
impl Vm for Ptx {
    type Storage = Reg;
    type Var = Reg;
    fn make_const(&mut self, c: f64) -> Self::Var {
        line!(self, "mov.f32", out, f32_to_hex(c as f32))
    }
    fn make_source(&mut self, name: &str) -> Self::Var {
        let off = self.inputs.len() * 4;
        self.inputs.push(name.to_owned());
        
        let reg = self.alloc();
        self.push(format!("    ld.cs.f32           {}, [data_in+{}];", reg, off));
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
        line!(self, "rcp.rnd.f32", out, a)
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
