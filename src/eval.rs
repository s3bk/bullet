use node::Node;
use std::collections::HashMap;
use std::fmt::{self, Display};
use std::error::Error;

pub struct EvalContext {
    defines: HashMap<String, f64>
}

#[derive(Debug)]
pub enum EvalError {
    UndefinedVar(String)
}
impl Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EvalError::UndefinedVar(ref var) => write!(f, "variable '{}' is undefined", var)
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
            Node::Int(i) => Ok(i as f64),
            Node::Sum(ref parts) => self.fold(0.0, |a, b| a + b, parts),
            Node::Prod(ref parts) => self.fold(1.0, |a, b| a * b, parts),
            Node::Pow(box (ref f, ref g)) => Ok(self.eval(f)?.powf(self.eval(g)?)),
            Node::Func(f, box ref g) => Ok(f.eval_f64(self.eval(g)?)),
            Node::Var(ref s) => self.defines.get(s).cloned().ok_or(EvalError::UndefinedVar(s.clone()))
        }
    }
    
    fn fold<F>(&self, mut x: f64, step: F, parts: &[Node]) -> Result<f64, EvalError>
        where F: Fn(f64, f64) -> f64
    {
        for n in parts {
            x = step(x, self.eval(n)?);
        }
        Ok(x)
    }

    pub fn set(&mut self, var: &str, val: f64) {
        self.defines.insert(var.into(), val);
    }
}
