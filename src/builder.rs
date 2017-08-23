use std::cell::RefCell;
use node::*;
use func::Func;
use rational::Rational;
use poly::Poly;
use lang::parse_Expr;

pub struct Builder {
    cache: RefCell<Cache>,
}

fn poly(node: NodeRc) -> Poly {
    match *node {
        Node::Poly(p) => p,
        _ => Poly::from_node(node)
    }
}
    
impl Builder {
    pub fn new() -> Builder {
        Builder { cache: Cache::new() }
    }
    pub fn parse(&self, expr: &str) -> Result<NodeRc, ()> {
        parse_Expr(self, expr)
    }
    pub fn int(&self, i: i64) -> NodeRc {
        self.intern(Node::Poly(Poly::int(i)))
    }
    
    /// decimal number
    pub fn decimal(&self, n: &str) -> NodeRc {
        let i: i64 = n.parse(n).expect("failed to parse decimal");
        self.int(i)
    }

    fn poly(&self, p: Poly) -> NodeRc {
        self.intern(Node::Poly(p))
    }
    
    /// a + b
    pub fn add(&self, a: NodeRc, b: NodeRc) -> NodeRc {
        self.poly(poly(a) + poly(b))
    }

    /// a - b
    pub fn sub(&self, a: NodeRc, b: NodeRc) -> NodeRc {
        self.poly(poly(a) + poly(b) * (-1))
    }

    /// a * b
    pub fn mul(&self, a: NodeRc, b: NodeRc) -> NodeRc {
        self.poly(poly(a) * poly(b))
    }

    /// a / b
    pub fn div(&self, a: NodeRc, b: NodeRc) -> NodeRc {
        self.poly(poly(a) * poly(b).pow_i(-1))
    }

    /// a ^ b
    pub fn pow(&self, a: NodeRc, b: NodeRc) -> NodeRc {
        if let Node::Poly(p) = b {
            if let Some(i) = p.as_rational().as_int() {          
                return self.pow_i(a, i);
            }
        }

        let g = self.func(Func::Log, b);
        self.func(Func::Exp, g)
    }
    /// a ^ i
    pub fn pow_i(&self, a: NodeRc, i: i64) -> NodeRc {
        self.poly(poly(a).pow_i(i))
    }

    /// f(g)
    pub fn func(&self, f: Func, g: NodeRc) -> NodeRc {
        self.intern(Node::Func(f, g))
    }

    /// f(g) (by name)
    pub fn function(&self, name: &str, arg: NodeRc) -> NodeRc {
        self.func(Func::from_name(name), arg)
    }

    /// make a name variable
    pub fn var(&self, name: &str) -> NodeRc {
        self.intern(Node::Var(name.into()))
    }

    /// f_0 · f_1 · f_2 · … · f_n
    pub fn product<I>(&self, factors: I) -> NodeRc where I: IntoIterator<Item=NodeRc> {
        let mut p = Poly::int(1);
        for f in factors.into_iter() {
            p *= poly(f);
        }
        self.poly(p)
    }

    /// f_0 + f_1 + f_2 + … + f_n
    pub fn sum<I>(&self, summands: I) -> NodeRc where I: IntoIterator<Item=NodeRc> {
        let mut p = Poly::int(0);
        for n in summands.into_iter() {
            p += poly(n);
        }
        self.poly(p)
    }

    pub fn rational(&self, r: Rational) -> NodeRc {
        self.poly(Poly::rational(r))
    }
    
    pub fn intern(&self, node: Node) -> NodeRc {
        self.cache.intern(node)
    }
}
