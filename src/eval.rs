use crate::prelude::*;
use std::collections::HashMap;
use crate::func::Transient::*;
use crate::func::Func;


#[derive(Debug)]
pub enum Command<'a> {
    Define(&'a str, Vec<&'a str>, NodeRc),
    Expr(NodeRc),
    Eval(NodeRc),
    Bench(NodeRc)
}

pub struct EvalContext {
    builder: Builder,
    defines: HashMap<String, f64>
}

impl EvalContext {
    pub fn new() -> EvalContext {
        EvalContext {
            builder: Builder::new(),
            defines: HashMap::new()
        }
    }
    pub fn eval(&self, node: &Node) -> Result<f64, Error> {
        match *node {
            Node::Poly(ref p) => {
                let mut sum = 0.0;
                for (base, r) in p.factors() {
                    let mut prod = r.as_f64();
                    for &(ref f, ref n) in base.iter() {
                        prod *= self.eval(f)?.powf(n.as_f64());
                    }
                    sum += prod;
                }
                Ok(sum)
            }
            Node::Apply(ref f, ref g) => {
                let x = self.eval(g)?;
                match **f {
                    Node::Op(Func::Transient(f)) => Ok(match f {    
                        Sin => x.sin(),
                        Cos => x.cos(),
                        Log => x.ln(),
                        Exp => x.exp(),
                        _   => todo!("missing function")
                    }),
                    _ => todo!("apply non transients")
                }
            },
            Node::Op(Func::Definition(_, ref node)) => self.eval(node),
            Node::Op(ref op) => {
                dbg!(op);
                todo!("?")
            }
            Node::Var(ref s) => self.defines.get(s).cloned().ok_or(Error::Undefined(s.clone())),
            _ => unimplemented!()
        }
    }
    
    pub fn set(&mut self, var: &str, val: f64) {
        self.defines.insert(var.into(), val);
    }

    pub fn get(&self, var: &str) -> Option<f64> {
        self.defines.get(var).cloned()
    }

    #[cfg(all(target_feature = "avx", feature="jit"))]
    fn bench(&self, expr: NodeRc) -> Result<String, Error> {
        use std::time::Instant;
        use std::simd::f32x8;
        use crate::rt::simd_jit::compile;
        
        let code = compile(&[expr], &["x"])?;
        let data_in = vec![f32x8::splat(self.get("x").unwrap_or(0.1) as f32); code.num_inputs];
        let mut data_out = vec![f32x8::splat(0.0); code.num_outputs];

        let n = 1024*1024;
        let t0 = Instant::now();
        code.bench(&data_in, &mut data_out, n/8);
        let dt = t0.elapsed();

        Ok(format!("{} values/s", n as f64 / duration_as_seconds(dt)))
    }
    
    pub fn run(&mut self, input: &str) -> Result<Option<String>, Error> {
        use crate::lang::CommandParser;
        
        let cmd = match CommandParser::new().parse(&self.builder, input) {
            Ok(r) => r?,
            Err(e) => return Err(Error::parse_error(e, input))
        };

        #[allow(unreachable_patterns)]
        Ok(match cmd {
            Command::Define(f, args, expr) => {
                self.builder.define(f, &args, expr);
                None
            },
            Command::Expr(e) => Some(e.to_string()),
            Command::Eval(e) => Some(self.eval(&e)?.to_string()),
            #[cfg(all(target_feature = "avx", feature="jit"))]
            Command::Bench(e) => Some(self.bench(e)?),
            _ => None
        })
    }
}
