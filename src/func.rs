use std::fmt;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum Func {
    Sin,
    Cos,
    Log,
    Exp
}
use self::Func::*;

impl fmt::Display for Func {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Sin => "sin",
            Cos => "cos",
            Log => "log",
            Exp => "exp"
        };
        f.write_str(s)
    }
}
impl Func {
    pub fn eval_f64(self, x: f64) -> f64 {
        match self {
            Sin => x.sin(),
            Cos => x.cos(),
            Log => x.ln(),
            Exp => x.exp()
        }
    }
}
