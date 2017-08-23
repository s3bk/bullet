use node::Node;
use std::collections::HashMap;
use std::fmt::{self, Display};
use std::error::Error;
use cast::Cast;

pub struct EvalContext {
    defines: HashMap<String, f64>
}

#[derive(Debug)]
pub enum EvalError {
    UndefinedVar(String),
    Overflow
}
impl Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EvalError::UndefinedVar(ref var) => write!(f, "variable '{}' is undefined", var),
            EvalError::Overflow => write!(f, "cannot compute!")
        }
    }
}
impl Error for EvalError {
    fn description(&self) -> &str { "error evaluating expression" }
}

impl EvalContext {
    pub fn new() -> EvalContext {
        EvalContext {
            defines: HashMap::new()
        }
    }
    pub fn eval(&self, node: &Node) -> Result<f64, EvalError> {
        match *node {
            Node::Poly(ref p) => {
                let mut prod = 1.0;
                for (base, r) in p.factors() {
                    for &(ref f, n) in base.iter() {
                        prod *= self.eval(f)?.powi(n.cast().ok_or(EvalError::Overflow)?);
                    }
                }
                Ok(prod)
            }
            Node::Func(f, ref g) => Ok(f.eval_f64(self.eval(g)?)),
            Node::Var(ref s) => self.defines.get(s).cloned().ok_or(EvalError::UndefinedVar(s.clone()))
        }
    }
    
    pub fn set(&mut self, var: &str, val: f64) {
        self.defines.insert(var.into(), val);
    }
}
