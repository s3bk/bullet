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
    Poly(PolyError)
}
impl<'a> fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;
        use lalrpop_util::ParseError::UnrecognizedToken;
        match *self {
            MissingFunction(s) => write!(f, "the function '{}' is not implemented", s),
            ParseError(UnrecognizedToken { token: Some((_start, (_, t), _end)), ref expected }) => 
                write!(f, "the character '{}' was not on of the expected ({})", t, expected.join(", ")),
            ParseError(ref e) => write!(f, "{:?}", e),
            IntegerError => write!(f, "not an integer"),
            Poly(PolyError::DivZero) => write!(f, "division by zero")
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
    pub fn div(&self, a: NodeRc, b: NodeRc) -> NodeResult<'static> {
        Ok(self.poly(poly(a) * (poly(b).pow_i(self, -1)?)))
    }

    /// - a
    pub fn neg(&self, a: NodeRc) -> NodeRc {
        self.mul(self.int(-1), a)
    }
    
    /// a ^ b
    pub fn pow(&self, a: NodeRc, b: NodeRc) -> NodeResult<'static> {
        if let Node::Poly(ref p) = *b {
            if let Some(i) = p.as_int().and_then(|i| i.cast()) {          
                return Ok(self.pow_i(a, i)?);
            }
        }

        let g = self.func(Func::Log, a);
        Ok(self.func(Func::Exp, self.mul(g, b)))
    }
    /// a ^ i
    pub fn pow_i(&self, a: NodeRc, i: i32) -> NodeResult<'static> {
        Ok(self.poly(poly(a).pow_i(self, i)?))
    }

    /// f(g)
    pub fn func(&self, f: Func, g: NodeRc) -> NodeRc {
        self.intern(Node::Func(f, g))
    }

    pub fn op<'a>(&self, o: Op<'a>, f: NodeRc) -> NodeResult<'a> {
        match o {
            Op::Diff(v) => Ok(diff(self, &f, v)?)
        }
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
        Ok(self.func(f, arg))
    }

    /// make a name variable
    pub fn var(&self, name: &str) -> NodeRc {
        self.intern(Node::Var(name.into()))
    }

    /// f_0 · f_1 · f_2 · … · f_n
    pub fn product<I>(&self, factors: I) -> NodeRc where I: IntoIterator<Item=NodeRc> {
        let mut p = Poly::int(1);
        for f in factors.into_iter() {
            p = p * poly(f);
        }
        self.poly(p)
    }

    /// f_0 + f_1 + f_2 + … + f_n
    pub fn sum<I>(&self, summands: I) -> NodeRc where I: IntoIterator<Item=NodeRc> {
        let mut p = Poly::int(0);
        for n in summands.into_iter() {
            p = p + poly(n);
        }
        self.poly(p)
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
