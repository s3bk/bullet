use std::fmt;
use crate::func::Func;
use std::ops::Deref;
use std::collections::HashSet;
use std::rc::{Rc};
use crate::poly::Poly;

pub struct Cache {
    items: HashSet<Rc<Node>>
}
impl Cache {
    pub fn new() -> Cache {
        Cache { items: HashSet::new() }
    }
    pub fn intern(&mut self, node: Node) -> NodeRc {
        let inner = if let Some(rc) = self.items.get(&node) {
            rc.clone()
        } else {
            let rc = Rc::new(node);
            self.items.insert(rc.clone());
            rc
        };
        NodeRc { inner }
    }
}
#[derive(Clone, Debug, Ord, PartialOrd, PartialEq, Eq, Hash)]
pub struct NodeRc {
    inner: Rc<Node>,
}
impl Deref for NodeRc {
    type Target = Node;
    fn deref(&self) -> &Node { &self.inner }
}

impl fmt::Display for NodeRc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.fmt(f)
    }
}


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Sign {
    Negative,
    Positive
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum Op {
    Diff(String),
}
impl fmt::Display for Op {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Op::Diff(ref v) => write!(w, "d/d{}", v)
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Node {
    Var(String),
    Op(Func),
    Apply(NodeRc, NodeRc),
    Poly(Poly),
    Tuple(Vec<NodeRc>)
}

impl fmt::Display for Node {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        use crate::display::*;
        Tokens::node(self, &Mode::Text).fmt(w)
    }
}
