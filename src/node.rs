use std::fmt;
use func::Func;
use std::ops::Deref;
use std::collections::hash_map::{HashMap, DefaultHasher, Entry};
use std::rc::Rc;
use std::cmp::Ordering;
use poly::Poly;
use std::hash::{Hash, Hasher};

pub struct Cache {
    items: HashMap<u64, NodeRc>
}
impl Cache {
    pub fn new() -> Cache {
        Cache { items: HashMap::new() }
    }
    pub fn intern(&mut self, node: Node) -> NodeRc {
        let mut h = DefaultHasher::new();
        node.hash(&mut h);
        let hash = h.finish();
        match self.items.entry(hash) {
            Entry::Vacant(v) => v.insert(NodeRc {
                inner: Rc::new((hash, node))
            }).clone(),
            Entry::Occupied(o) => {
                assert_eq!(o.get().inner.1, node);
                o.get().clone()
            }
        }
    }
}
#[derive(Clone, Debug)]
pub struct NodeRc {
    inner: Rc<(u64, Node)>,
}
impl Deref for NodeRc {
    type Target = Node;
    fn deref(&self) -> &Node { &self.inner.1 }
}
impl PartialEq for NodeRc {
    fn eq(&self, rhs: &NodeRc) -> bool {
        self.inner.0 == rhs.inner.0
    }
}
impl Eq for NodeRc {}
impl Hash for NodeRc {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.inner.0);
    }
}
impl PartialOrd for NodeRc {
    fn partial_cmp(&self, rhs: &NodeRc) -> Option<Ordering> {
        self.inner.0.partial_cmp(&rhs.inner.0)
    }
}
impl Ord for NodeRc {
    fn cmp(&self, rhs: &NodeRc) -> Ordering {
        self.inner.0.cmp(&rhs.inner.0)
    }
}
impl fmt::Display for NodeRc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.1.fmt(f)
    }
}


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Sign {
    Negative,
    Positive
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Node {
    Var(String),
    Func(Func, NodeRc),
    Poly(Poly)
}

impl fmt::Display for Node {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Node::Func(f, ref g) => write!(w, "{}({})", f, g),
            Node::Var(ref s) => s.fmt(w),
            Node::Poly(ref p) => p.fmt(w)
        }
    }
}
