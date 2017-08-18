use std::fmt::{self, Display, Debug};
use diff::{Node, Func};

#[derive(Debug)]
pub enum Expr {
    Add(Box<(Expr, Expr)>),
    Sub(Box<(Expr, Expr)>),
    Mul(Box<(Expr, Expr)>),
    Div(Box<(Expr, Expr)>),
    Pow(Box<(Expr, Expr)>),
    Func(String, Box<Expr>),
    Int(i32),
    Var(String)
}

impl Display for Expr {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        use self::Expr::*;
        match *self {
            Add(box (ref l, ref r)) => write!(w, "({} + {})", l, r),
            Sub(box (ref l, ref r)) => write!(w, "({} - {})", l, r),
            Mul(box (ref l, ref r)) => write!(w, "({} * {})", l, r),
            Div(box (ref l, ref r)) => write!(w, "({} / {})", l, r),
            Int(n) => write!(w, "{}", n),
            Pow(box (ref b, ref e)) => write!(w, "{}^{}", b, e),
            Func(ref f, box ref g) => write!(w, "{} {}", f, g),
            Var(ref s) => write!(w, "{}", s)
        }
    }
}

impl Expr {    
    pub fn to_node(&self) -> Result<Node, String> {
        Ok(match *self {
            Expr::Add(box (ref f, ref g)) => Node::Sum(vec![f.to_node()?, g.to_node()?]),
            Expr::Sub(box (ref f, ref g)) => Node::Sum(vec![f.to_node()?, Node::Prod(vec![Node::Int(-1), g.to_node()?])]),
            Expr::Mul(box (ref f, ref g)) => Node::Prod(vec![f.to_node()?, g.to_node()?]),
            Expr::Div(box (ref f, ref g)) => Node::Prod(vec![f.to_node()?, Node::Pow(box (g.to_node()?, Node::Int(-1)))]),
            Expr::Pow(box (ref f, ref g)) => Node::Pow(box(f.to_node()?, g.to_node()?)),
            Expr::Func(ref name, box ref g) => {
                let f = match name.as_str() {
                    "sin" => Func::Sin,
                    "cos" => Func::Cos,
                    "log" => Func::Log,
                    "exp" => Func::Exp,
                    s => return Err(format!("'{}' is not implemented yet", s))
                };
                Node::Func(f, box g.to_node()?)
            },
            Expr::Int(i) => Node::Int(i as i64),
            Expr::Var(ref name) => Node::Var(name.clone())
        })
    }
}
