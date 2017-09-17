use std::cell::RefCell;
use node::*;
use func::Func;
use rational::Rational;
use poly::{Poly, PolyError};
use lang::parse_Expr;
use lalrpop_util;
use cast::Cast;
use std::fmt;
use std::collections::HashMap;
use std::iter::once;

#[derive(Debug)]
pub enum Error<'a> {
    MissingFunction(&'a str),
    ParseError(lalrpop_util::ParseError<usize, (usize, &'a str), ()>),
    IntegerError,
    Poly(PolyError),
    Undefined(&'a str),
    ShapeMismatch(usize, usize),
}
impl<'a> fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;
        use lalrpop_util::ParseError::UnrecognizedToken;
        match *self {
            MissingFunction(s) => write!(f, "the function '{}' is not implemented", s),
            ParseError(UnrecognizedToken { token: Some((_start, (_, t), _end)), ref expected }) => 
                write!(f, "the character '{}' was not one of the expected {}", t, expected.join(", ")),
            ParseError(ref e) => write!(f, "{:?}", e),
            IntegerError => write!(f, "not an integer"),
            Poly(PolyError::DivZero) => write!(f, "division by zero"),
            Undefined(name) => write!(f, "'{}' is not defined", name),
            ShapeMismatch(a, b) => writeln!(f, "shapes do not match ({} vs. {})", a, b)
        }       
    }
}
impl<'a> From<PolyError> for Error<'a> {
    fn from(e: PolyError) -> Error<'a> { Error::Poly(e) }
}
pub type NodeResult<'a> = Result<NodeRc, Error<'a>>;

struct Definition {
    args: Vec<String>,
    expr: NodeRc
}

pub struct Builder {
    cache: RefCell<Cache>,
    defs: HashMap<String, Definition>
}

fn poly(node: NodeRc) -> Poly {
    if let Node::Poly(ref p) = *node {
        return p.clone();
    }
    Poly::from_node(node)
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            cache: RefCell::new(Cache::new()),
            defs:  HashMap::new()
        }
    }
    pub fn parse<'a>(&self, expr: &'a str) -> NodeResult<'a> {
        match parse_Expr(self, expr) {
            Ok(r) => r,
            Err(e) => Err(Error::ParseError(e))
        }
    }
    pub fn int(&self, i: i64) -> NodeRc {
        self.intern(Node::Poly(Poly::int(i)))
    }
    
    /// decimal number
    pub fn decimal<'a>(&self, n: &'a str) -> NodeResult<'a> {
        let i: i64 = n.parse().map_err(|_| Error::IntegerError)?;
        Ok(self.int(i))
    }

    pub fn poly(&self, p: Poly) -> NodeRc {
        self.intern(Node::Poly(p))
    }

    fn uniform<F>(&self, a: NodeRc, b: NodeRc, f: F) -> NodeResult<'static>
        where F: Fn(NodeRc, NodeRc) -> NodeResult<'static>
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
            (_, &Node::Tuple(ref tb)) => self.tuple(tb.iter().map(|a| f(a.clone(), b.clone()))),
            (_, _) => f(a.clone(), b.clone())
        }
    }
    fn uniform_one<F, T>(&self, a: NodeRc, t: T, f: F) -> NodeResult<'static>
        where F: Fn(NodeRc, T) -> NodeResult<'static>, T: Clone
    {
        match *a {
            Node::Tuple(ref ta) => self.tuple(ta.iter().map(|a| f(a.clone(), t.clone()))),
            _ => f(a.clone(), t)
        }
    }
    /// a + b
    pub fn add(&self, a: NodeRc, b: NodeRc) -> NodeResult<'static> {
        self.uniform(a, b, |a, b| Ok(self.poly(poly(a) + poly(b))))
    }

    /// a - b
    pub fn sub(&self, a: NodeRc, b: NodeRc) -> NodeResult<'static> {
        self.uniform(a, b, |a, b| Ok(self.poly(poly(a) + poly(b) * (-1))))
    }

    /// a * b
    pub fn mul(&self, a: NodeRc, b: NodeRc) -> NodeResult<'static> {
        self.uniform(a, b, |a, b| Ok(self.poly(poly(a) * poly(b))))
    }

    /// a / b
    pub fn div(&self, a: NodeRc, b: NodeRc) -> NodeResult<'static> {
        self.uniform(a, b, |a, b| Ok(self.poly(poly(a) * (poly(b).pow_i(self, -1)?))))
    }

    /// - a
    pub fn neg(&self, a: NodeRc) -> NodeResult<'static> {
        self.mul(self.int(-1), a)
    }
    
    /// a ^ b
    pub fn pow(&self, a: NodeRc, b: NodeRc) -> NodeResult<'static> {
        self.uniform(a, b, |a, b| {
            if let Node::Poly(ref p) = *b {
                if let Some(i) = p.as_int().and_then(|i| i.cast()) {          
                    return Ok(self.pow_i(a, i)?);
                }
            }
            
            let g = self.func(Func::Log, a)?;
            self.func(Func::Exp, self.mul(g, b)?)
        })
    }
    /// a ^ i
    pub fn pow_i(&self, a: NodeRc, i: i32) -> NodeResult<'static> {
        self.uniform_one(a, i, |a, i| Ok(self.poly(poly(a).pow_i(self, i)?)))
    }

    /// f(g)
    pub fn func(&self, f: Func, g: NodeRc) -> NodeResult<'static> {
        self.uniform_one(g, f, |g, f| Ok(self.intern(Node::Func(f, g))))
    }

    pub fn op(&self, o: Op) -> NodeRc {
        self.intern(Node::Op(o))
    }

    /// f(g) (by name)
    pub fn function<'a>(&self, name: &'a str, arg: NodeRc) -> NodeResult<'a> {
        let f = Func::from_name(name).ok_or(Error::MissingFunction(name))?;
        self.func(f, arg)
    }

    /// make a name variable
    pub fn var(&self, name: &str) -> NodeRc {
        self.intern(Node::Var(name.into()))
    }

    /// magic 'apply' function
    pub fn apply_named<'a>(&self, left: &'a str, right: NodeRc) -> NodeResult<'a> {
        let def = self.defs.get(left).ok_or(Error::Undefined(left))?;

        let subst = match *right {
            Node::Tuple(ref parts) => parts.clone(),
            _ => vec![right.clone()]
        };
        let map: HashMap<_, _> = def.args.iter()
            .zip(subst.into_iter())
            .map(|(var, subst)| (&**var, subst))
            .collect();

        self.substitute(&def.expr, &map)
    }

    fn substitute(&self, node: &NodeRc, map: &HashMap<&str, NodeRc>) -> NodeResult<'static> {
        match **node {
            Node::Var(ref name) => match map.get(&**name) {
                Some(node) => Ok(node.clone()),
                None => Ok(node.clone())
            },
            Node::Tuple(ref parts) => self.tuple(parts.iter().map(|n| self.substitute(n, map))),
            Node::Poly(ref p) => self.sum(
                p.factors().map(|(base, &fac)| {
                    self.product(
                        once(Ok(self.rational(fac)))
                            .chain(
                                base.iter().map(|&(ref v, p)| self.pow_i(
                                    self.substitute(v, map)?,
                                    p.cast().expect("too high")
                                ))
                            )
                    )
                })
            ),
            Node::Func(f, ref n) => self.func(f, self.substitute(n, map)?),
            _ => unimplemented!()
        }
    }

    /// f_0 · f_1 · f_2 · … · f_n
    pub fn product<'a, I>(&self, mut factors: I) -> NodeResult<'a>
        where I: IntoIterator<Item=NodeResult<'a>>
    {
        try_fold(factors, self.int(1), |a, b| self.mul(a, b))
    }

    /// f_0 + f_1 + f_2 + … + f_n
    pub fn sum<'a, I>(&self, summands: I) -> NodeResult<'a>
        where I: IntoIterator<Item=NodeResult<'a>>
    {
        try_fold(summands, self.int(0), |a, b| self.add(a, b))
    }

    pub fn rational(&self, r: Rational) -> NodeRc {
        self.poly(Poly::rational(r))
    }

    pub fn tuple<'a, I>(&self, parts: I) -> NodeResult<'a>
        where I: IntoIterator<Item=NodeResult<'a>>
    {
        let v: Result<Vec<_>, _> = parts.into_iter().collect();
        Ok(self.intern(Node::Tuple(v?)))
    }
    
    pub fn intern(&self, node: Node) -> NodeRc {
        self.cache.borrow_mut().intern(node).clone()
    }
}

fn try_fold<T, I, F, E>(iter: I, mut init: T, func: F) -> Result<T, E>
    where I: IntoIterator<Item=Result<T, E>>, F: Fn(T, T) -> Result<T, E>
{
    for i in iter.into_iter() {
        init = func(init, i?)?;
    }
    Ok(init)
}
