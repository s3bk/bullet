use std::fmt::{self, Display, Debug};

#[derive(Debug)]
pub enum Expr {
    Add(Node, Node),
    Sub(Node, Node),
    Mul(Node, Node),
    Div(Node, Node),
    Int(i32)
}
pub type Node = Box<Expr>;

impl Display for Expr {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        use self::Expr::*;
        match *self {
            Add(ref l, ref r) => write!(w, "({} + {})", l, r),
            Sub(ref l, ref r) => write!(w, "({} - {})", l, r),
            Mul(ref l, ref r) => write!(w, "({} * {})", l, r),
            Div(ref l, ref r) => write!(w, "({} / {})", l, r),
            Int(n) => write!(w, "{}", n),
        }
    }
}
