use prelude::*;
use std::cell::RefCell;
use func::Func;
use func::Transient::*;
use poly::Poly;
use lang::ExprParser;
use std::collections::HashMap;
use std::iter::once;
use diff::diff;

pub type NodeResult = Result<NodeRc, Error>;

pub struct Builder {
    cache: RefCell<Cache>,
    defs: HashMap<String, NodeRc>
}

fn poly(node: NodeRc) -> Poly {
    if let Node::Poly(ref p) = *node {
        return p.clone();
    }
    Poly::from_node(node)
}

impl Builder {
    pub fn new() -> Builder {
        let mut b = Builder {
            cache: RefCell::new(Cache::new()),
            defs:  HashMap::new()
        };
        b.init();
        b
    }
    fn init(&mut self) {
        let x = self.var("x");
        for &(n, f) in [("sin", Sin), ("cos", Cos), ("exp", Exp), ("log", Log), ("ln", Log)].iter() {
            let f = self.func(Func::Transient(f), x.clone()).unwrap();
            self.define(n, &["x"], f);
        }
    }
    pub fn define(&mut self, name: &str, args: &[&str], node: NodeRc) {
        let def = Node::Op(Func::Definition(
            args.iter().map(|&s| s.into()).collect(),
            node
        ));
        let defn = self.intern(def);
        self.defs.insert(name.to_owned(), defn);
    }
    pub fn parse(&self, expr: &str) -> NodeResult {
        ExprParser::new().parse(self, expr).unwrap_or_else(|e| Err(Error::parse_error(e, expr)))
    }
    pub fn int<T: Into<Int>>(&self, i: T) -> NodeRc {
        self.intern(Node::Poly(Poly::int(i.into())))
    }
    
    /// decimal number
    pub fn decimal(&self, n: &str) -> NodeResult {
        Ok(self.int(Int::parse(n, 10)?))
    }
    pub fn decimal_float(&self, s: &str) -> NodeResult {
        let dp = s.find('.').unwrap();
        let div = Int::from(10).pow((s.len() - dp - 1) as u32);
        let i = Int::parse(&s[..dp], 10)?;
        let j = Int::parse(&s[dp+1..], 10)?;
        self.add(self.int(i), self.div(self.int(j), self.int(div))?)
    }

    pub fn poly(&self, p: Poly) -> NodeRc {
        self.intern(Node::Poly(p))
    }

    fn uniform<F>(&self, a: NodeRc, b: NodeRc, f: F) -> NodeResult
        where F: Fn(NodeRc, NodeRc) -> NodeResult
    {
        match (&*a, &*b) {
            (&Node::Tuple(ref ta), &Node::Tuple(ref tb)) => {
                if ta.len() != tb.len() {
                    return Err(Error::ShapeMismatch(ta.len(), tb.len()));
                }
                self.tuple(
                    ta.iter().zip(tb.iter()).map(|(a, b)| f(a.clone(), b.clone()))
                )
            },
            (&Node::Tuple(ref ta), _) => self.tuple(ta.iter().map(|a| f(a.clone(), b.clone()))),
            (_, &Node::Tuple(ref tb)) => self.tuple(tb.iter().map(|b| f(a.clone(), b.clone()))),
            (_, _) => f(a.clone(), b.clone())
        }
    }
    fn uniform_one<F, T>(&self, a: NodeRc, t: T, f: F) -> NodeResult
        where F: Fn(NodeRc, T) -> NodeResult, T: Clone
    {
        match *a {
            Node::Tuple(ref ta) => self.tuple(ta.iter().map(|a| f(a.clone(), t.clone()))),
            _ => f(a.clone(), t)
        }
    }
    /// a + b
    pub fn add(&self, a: NodeRc, b: NodeRc) -> NodeResult {
        self.uniform(a, b, |a, b| Ok(self.poly(poly(a) + poly(b))))
    }

    /// a - b
    pub fn sub(&self, a: NodeRc, b: NodeRc) -> NodeResult {
        self.uniform(a, b, |a, b| Ok(self.poly(poly(a) + poly(b) * (-1))))
    }

    /// a * b
    pub fn mul(&self, a: NodeRc, b: NodeRc) -> NodeResult {
        self.uniform(a, b, |a, b| Ok(self.poly(poly(a) * poly(b))))
    }

    /// a / b
    pub fn div(&self, a: NodeRc, b: NodeRc) -> NodeResult {
        self.uniform(a, b, |a, b| Ok(self.poly(poly(a) * poly(b).pow_i(self, -1)?)))
    }

    /// - a
    pub fn neg(&self, a: NodeRc) -> NodeResult {
        self.mul(self.int(-1), a)
    }
    
    /// a ^ b
    pub fn pow(&self, a: NodeRc, b: NodeRc) -> NodeResult {
        self.uniform(a, b, |a, b| {
            if let Node::Poly(ref p) = *b {
                if let Some(i) = p.to_int() {          
                    return self.pow_i(a, i);
                }
            }
            
            let g = self.func(Log.into(), a)?;
            self.func(Exp.into(), self.mul(g, b)?)
        })
    }
    /// a ^ i
    pub fn pow_i<I>(&self, a: NodeRc, i: I) -> NodeResult
        where I: Into<Int>
    {
        let i: Int = i.into();
        self.uniform_one(a, i, |a, i| Ok(self.poly(poly(a).pow(self, i)?)))
    }
    
    /// a!
    pub fn factorial(&self, a: NodeRc) -> NodeResult {
        todo!("factorial")
    }
    
    /// f(g)
    pub fn func(&self, f: Func, g: NodeRc) -> NodeResult {
        self.apply(self.op(f)?, g)
    }

    /// f
    pub fn op(&self, f: Func) -> NodeResult {
        Ok(self.intern(Node::Op(f)))
    }

    /// make a name variable
    pub fn var(&self, name: &str) -> NodeRc {
        self.intern(Node::Var(name.into()))
    }
    pub fn named(&self, name: &str) -> NodeRc {
        match self.defs.get(name) {
            Some(n) => n.clone(),
            None => self.var(name)
        }
    }

    /// magic 'apply' function
    pub fn apply(&self, left: NodeRc, right: NodeRc) -> NodeResult {
        match *left {
            Node::Op(ref op) => match *op {
                Func::Diff(ref var) => return self.uniform_one(right, (), |g, ()| diff(self, &g, var)),
                Func::Definition(ref def_args, ref body) => {
                    let map = |args: &[NodeRc]| -> HashMap<&str, NodeRc> {
                        args.iter()
                            .zip(def_args.iter())
                            .map(|(subst, var)| (&**var, subst.clone()))
                            .collect()
                    };

                    return match *right {
                        Node::Tuple(ref parts) => match def_args.len() {
                            1 => {
                                self.tuple(parts.windows(1).map(|p| self.substitute(body, &map(p))))
                            },
                            n if n == parts.len() => self.substitute(&body, &map(parts)),
                            n => Err(Error::ShapeMismatch(n, parts.len()))
                        },
                        _ if def_args.len() == 1 => self.substitute(&body, &map(&[right.clone()])),
                        _ => Err(Error::ShapeMismatch(def_args.len(), 1))
                    };
                },
                _ => return Ok(self.intern(Node::Apply(left.clone(), right)))       
            }
            _ => {}
        }
        self.mul(left, right)
    }

    fn substitute(&self, node: &NodeRc, map: &HashMap<&str, NodeRc>) -> NodeResult {
        match **node {
            Node::Var(ref name) => match map.get(&**name) {
                Some(node) => Ok(node.clone()),
                None => Ok(node.clone())
            },
            Node::Tuple(ref parts) => self.tuple(parts.iter().map(|n| self.substitute(n, map))),
            Node::Poly(ref p) => self.sum(
                p.factors().map(|(base, fac)| {
                    self.product(
                        once(Ok(self.rational(fac.clone())))
                            .chain(
                                base.iter().map(|&(ref v, ref p)| self.pow_i(
                                    self.substitute(v, map)?,
                                    p.clone()
                                ))
                            )
                    )
                })
            ),
            Node::Apply(ref f, ref g) => self.apply(self.substitute(f, map)?, self.substitute(g, map)?),
            Node::Op(_) => Ok(node.clone())
        }
    }

    /// f_0 · f_1 · f_2 · … · f_n
    pub fn product<I>(&self, factors: I) -> NodeResult
        where I: IntoIterator<Item=NodeResult>
    {
        try_fold(factors, self.int(1), |a, b| self.mul(a, b))
    }

    /// f_0 + f_1 + f_2 + … + f_n
    pub fn sum<I>(&self, summands: I) -> NodeResult
        where I: IntoIterator<Item=NodeResult>
    {
        try_fold(summands, self.int(0), |a, b| self.add(a, b))
    }

    pub fn rational(&self, r: Rational) -> NodeRc {
        self.poly(Poly::rational(r))
    }

    pub fn tuple<I>(&self, parts: I) -> NodeResult
        where I: IntoIterator<Item=NodeResult>
    {
        let v: Result<Vec<_>, _> = parts.into_iter().collect();
        Ok(self.intern(Node::Tuple(v?)))
    }

    pub fn array<I>(&self, _parts: I) -> NodeResult
        where I: IntoIterator<Item=NodeResult>
    {
        //let v: Result<Vec<_>> = parts.into_iter().collect();
        todo!("arrays")
    }
    
    pub fn intern(&self, node: Node) -> NodeRc {
        self.cache.borrow_mut().intern(node).clone()
    }
}

