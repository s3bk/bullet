use std::cell::RefCell;
use node::*;
use func::Func;
use rational::Rational;
use poly::{Poly, PolyError};
use lang::parse_Expr;
use lalrpop_util;
use cast::Cast;
use std::fmt;
use diff::diff;

#[derive(Copy, Clone, Debug)]
pub enum Op<'a> {
    Diff(&'a str)
}

#[derive(Debug)]
pub enum Error<'a> {
    MissingFunction(&'a str),
    ParseError(lalrpop_util::ParseError<usize, (usize, &'a str), ()>),
    IntegerError,
    Poly(PolyError),
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
            ShapeMismatch(a, b) => writeln!(f, "shapes do not match ({} vs. {})", a, b)
        }       
    }
}
impl<'a> From<PolyError> for Error<'a> {
    fn from(e: PolyError) -> Error<'a> { Error::Poly(e) }
}
pub type NodeResult<'a> = Result<NodeRc, Error<'a>>;

pub struct Builder {
    cache: RefCell<Cache>,
}

fn poly(node: NodeRc) -> Poly {
    if let Node::Poly(ref p) = *node {
        return p.clone();
    }
    Poly::from_node(node)
}

impl Builder {
    pub fn new() -> Builder {
        Builder { cache: RefCell::new(Cache::new()) }
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
                let mut v = Vec::with_capacity(ta.len());
                for (a, b) in ta.iter().zip(tb.iter()) {
                    v.push(f(a.clone(), b.clone())?);
                }
                Ok(self.tuple(v))
            },
            (&Node::Tuple(ref ta), _) => {
                let mut v = Vec::with_capacity(ta.len());
                for a in ta {
                    v.push(f(a.clone(), b.clone())?);
                }
                Ok(self.tuple(v))
            },
            (_, &Node::Tuple(ref tb)) => {
                let mut v = Vec::with_capacity(tb.len());
                for b in tb {
                    v.push(f(a.clone(), b.clone())?);
                }
                Ok(self.tuple(v))
            },
            (_, _) => f(a.clone(), b.clone())
        }
    }
    fn uniform_one<F, T>(&self, a: NodeRc, t: T, f: F) -> NodeResult<'static>
        where F: Fn(NodeRc, T) -> NodeResult<'static>, T: Clone
    {
        match *a {
            Node::Tuple(ref ta) => {
                let mut v = Vec::with_capacity(ta.len());
                for a in ta {
                    v.push(f(a.clone(), t.clone())?);
                }
                Ok(self.tuple(v))
            },
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

    pub fn op<'a>(&self, o: Op<'a>, f: NodeRc) -> NodeResult<'a> {
        self.uniform_one(f, o, |f, o| match o {
            Op::Diff(v) => Ok(diff(self, &f, v)?)
        })
    }
    pub fn op_n<'a>(&self, o: Op<'a>, n: u64, mut f: NodeRc) -> NodeResult<'a> {
        for _ in 0 .. n {
            f = self.op(o, f)?;
        }
        Ok(f)
    }

    /// f(g) (by name)
    pub fn function<'a>(&self, name: &'a str, arg: NodeRc) -> Result<NodeRc, Error<'a>> {
        let f = Func::from_name(name).ok_or(Error::MissingFunction(name))?;
        self.func(f, arg)
    }

    /// make a name variable
    pub fn var(&self, name: &str) -> NodeRc {
        self.intern(Node::Var(name.into()))
    }

    /// f_0 · f_1 · f_2 · … · f_n
    pub fn product<I>(&self, mut factors: I) -> NodeResult<'static>
        where I: IntoIterator<Item=NodeRc>
    {
        try_fold(factors, self.int(1), |a, b| self.mul(a, b))
    }

    /// f_0 + f_1 + f_2 + … + f_n
    pub fn sum<I>(&self, summands: I) -> NodeResult<'static>
        where I: IntoIterator<Item=NodeRc>
    {
        try_fold(summands, self.int(0), |a, b| self.add(a, b))
    }

    pub fn rational(&self, r: Rational) -> NodeRc {
        self.poly(Poly::rational(r))
    }

    pub fn tuple(&self, parts: Vec<NodeRc>) -> NodeRc {
        self.intern(Node::Tuple(parts))
    }
    
    pub fn intern(&self, node: Node) -> NodeRc {
        self.cache.borrow_mut().intern(node).clone()
    }
}

fn try_fold<T, I, F, E>(iter: I, mut init: T, func: F) -> Result<T, E>
    where I: IntoIterator<Item=T>, F: Fn(T, T) -> Result<T, E>
{
    for i in iter.into_iter() {
        init = func(init, i)?;
    }
    Ok(init)
}
