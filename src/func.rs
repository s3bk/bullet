use std::fmt;
use prelude::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum Transient {
    Sin,
    Cos,
    Log,
    Exp
}
use self::Transient::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum Func {
    Transient(Transient),
    Diff(String),
    Definition(Vec<String>, NodeRc)
}

impl From<Transient> for Func {
    fn from(t: Transient) -> Func {
        Func::Transient(t)
    }
}

impl fmt::Display for Func {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Func::*;
        match *self {
            Transient(t) => {
                let name = match t {
                    Sin => "sin",
                    Cos => "cos",
                    Log => "log",
                    Exp => "exp"
                };
                f.write_str(name)
            },
            Diff(ref var) => write!(f, "d/d{}", var),
            Definition(ref args, ref expr) => match args.len() {
                0 => expr.fmt(f),
                1 => write!(f, "{} => {}", args[0], expr),
                _ => write!(f, "({}) => {}", args.iter().format(", "), expr)
            }
        }
    }
}
