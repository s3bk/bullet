use itertools::Itertools;
use std::fmt::{self, Write};
use func::Func;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Node {
    Sum(Vec<Node>),
    Prod(Vec<Node>),
    Pow(Box<(Node, Node)>),
    Int(i64),
    Func(Func, Box<Node>),
    Var(String)
}
impl Node {
    pub fn simplify(self) -> Node {
        use simplify::simplify;
        simplify(self)
    }
}
        
impl fmt::Display for Node {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Node::Int(i) => write!(w, "{}", i),
            Node::Sum(ref parts) => write!(w, "({})", parts.iter().join(" + ")),
            Node::Prod(ref parts) => write!(w, "({})", parts.iter().join(" · ")),
            Node::Pow(box (ref f, ref g)) => write!(w, "{}^{}", f, g),
            Node::Func(f, box ref g) => write!(w, "{}({})", f, g),
            Node::Var(ref s) => write!(w, "{}", s)
        }
    }
}
