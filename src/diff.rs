use itertools::Itertools;
use std::fmt::{self, Write};
use std::iter::once;
use std::ops::{MulAssign, AddAssign, DivAssign};
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Func {
    Sin,
    Cos,
    Log,
    Exp
}
impl fmt::Display for Func {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Func::Sin => "sin",
            Func::Cos => "cos",
            Func::Log => "log",
            Func::Exp => "exp"
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Node {
    Sum(Vec<Node>),
    Prod(Vec<Node>),
    Pow(Box<(Node, Node)>),
    Int(i64),
    Func(Func, Box<Node>),
    Var(String)
}

impl fmt::Display for Node {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Node::Int(i) => write!(w, "{}", i),
            Node::Sum(ref parts) => write!(w, "({})", parts.iter().join(" + ")),
            Node::Prod(ref parts) => write!(w, "({})", parts.iter().join(" · ")),
            Node::Pow(box (ref f, ref g)) => write!(w, "{}^{}", f, g),
            Node::Func(f, box ref g) => write!(w, "{} {}", f, g),
            Node::Var(ref s) => write!(w, "{}", s)
        }
    }
}
pub fn diff(node: &Node, var: &str) -> Node {
    let out = match *node {
        Node::Int(_) => Node::Int(0),
        Node::Sum(ref parts) => Node::Sum(parts.iter().map(|n| diff(n, var)).collect()),
        Node::Prod(ref parts) => {
            Node::Sum(
                (0 .. parts.len()).map(|i| {
                    Node::Prod(
                        parts.iter().enumerate().map(|(j, f)| {
                            if i == j {
                                diff(f, var)
                            } else {
                                f.clone()
                            }
                        }).collect()
                    )
                }).collect()
            )
        },
        Node::Pow(box (ref f, ref g)) => {
            // f(x)^g(x) ( log f(x) · g'(x) + g(x) f'(x) f(x)^-1 )
            Node::Prod(vec![
                Node::Pow(box( // f(x)^g(x)
                    f.clone(),
                    g.clone()
        	)),
                Node::Sum(vec![ // log f(x) · g'(x) + g(x) f'(x) f(x)^-1
                    Node::Prod(vec![ // log f(x) · g'(x)
                        Node::Func(Func::Log, box f.clone()), // log f(x)
                        diff(g, var)
                    ]),
                    Node::Prod(vec![ // g(x) f'(x) f(x)^-1
                        g.clone(),
                        diff(f, var),
                        Node::Pow(box(f.clone(), Node::Int(-1))) // f(x)^-1
                    ])
                ])
            ])
        },
        Node::Func(f, box ref g) => {
            match f {
                Func::Sin => Node::Prod(vec![
                    Node::Func(Func::Cos, box g.clone()),
                    diff(g, var)
                ]),
                Func::Cos => Node::Prod(vec![
                    Node::Int(-1),
                    Node::Func(Func::Sin, box g.clone()),
                    diff(g, var)
                ]),
                Func::Log => Node::Prod(vec![
                    Node::Pow(box(g.clone(), Node::Int(-1))),
                    diff(g, var)
                ]),
                Func::Exp => Node::Prod(vec![
                    Node::Func(Func::Exp, box g.clone()),
                    diff(g, var)
                ])
            }
        },
        Node::Var(ref s) => {
            if s == var {
                Node::Int(1)
            } else {
                Node::Int(0)
            }
        },
    };
    println!("d/d{} {} = {}", var, node, out);
    out
}

fn gcd(mut ab: (i64, i64)) -> i64 {
    loop {
        match ab {
            (a, 0) => break a,
            (a, b) => ab = (b, a % b)
        }
    }
}
#[derive(Copy, Clone)]
struct Rational {
    num: i64,
    denom: i64
}
impl MulAssign<i64> for Rational {
    fn mul_assign(&mut self, n: i64) {
        self.num *= n;
    }
}
impl DivAssign<i64> for Rational {
    fn div_assign(&mut self, n: i64) {
        self.denom *= n;
    }
}
impl Rational {
    fn frac(&self) -> Result<(i64, i64), &'static str> {
        match (self.num, self.denom) {
            (0, 0) => Err("undefined"),
            (_, 0) => Err("infinite"),
            (0, _) => Ok((0, 1)),
            (n, 1) => Ok((n, 1)),
            (1, -1) => Ok((1, 1)),
            (mut n, mut d) => {
                if d < 0 {
                    n = -n;
                    d = -d;
                }
                let c = gcd((n, d));
                Ok((n/c, d/c))
            }
        }
    }
    
    pub fn to_node(&self) -> Result<Node, &'static str> {
        self.frac().map(|f| match f {
            (n, 1) => Node::Int(n),
            (n, d) => Node::Prod(vec![Node::Int(n), Node::Pow(box (Node::Int(d), Node::Int(-1)))])
        })
    }

    pub fn mul(&self, node: Node) -> Result<Node, &'static str> {
        self.frac().map(|f| match f {
            (1, 1) => node,
            (0, _) => Node::Int(0),
            (n, 1) => Node::Prod(vec![Node::Int(n),                                               node]),
            (1, d) => Node::Prod(vec![              Node::Pow(box (Node::Int(d), Node::Int(-1))), node]),
            (n, d) => Node::Prod(vec![Node::Int(n), Node::Pow(box (Node::Int(d), Node::Int(-1))), node])
        })
    }
    pub fn is_zero(&self) -> bool {
        self.num == 0
    }
}
impl From<i64> for Rational {
    fn from(i: i64) -> Rational {
        Rational {
            num: i,
            denom: 1
        }
    }
}
impl AddAssign<i64> for Rational {
    fn add_assign(&mut self, i: i64) {
        self.num += i * self.denom;
    }
}


struct SumFactors {
    factors: HashMap<Node, Rational>,
    base: Rational,
}
impl SumFactors {
    fn new() -> SumFactors {
        SumFactors {
            factors: HashMap::new(),
            base: Rational::from(0)
        }
    }
    fn from_sum<I>(parts: I) -> SumFactors where I: Iterator<Item=Node> {
        let mut sum = SumFactors::new();
        for n in parts {
            sum.add(n);
        }
        sum
    }
    
    fn add(&mut self, node: Node) {
        match simplify(node) {
            Node::Int(i) => self.base += i,
            Node::Sum(parts) => {
                for n in parts {
                    self.add(simplify(n));
                }
            },
            n => {
                *self.factors.entry(simplify(n)).or_insert(Rational::from(0)) += 1;
            }
        }
    }

    fn to_node(mut self) -> Node {
        let mut parts = Vec::new();
        if !self.base.is_zero() {
            parts.push(self.base.to_node().unwrap());
        }
        for (n, r) in self.factors {
            parts.push(r.mul(n).unwrap());
        }
        match parts.len() {
            0 => Node::Int(0),
            1 => parts.pop().unwrap(),
            _ => Node::Sum(parts)
        }
    }
}

fn simplify_sum<I>(parts: I) -> Node where I: Iterator<Item=Node> {
    SumFactors::from_sum(parts).to_node()
}

struct ProductPowers {
    // x^{y_0, y_1 ... y_n}
    powers: HashMap<Node, Vec<Node>>,
    mul: Rational,
}
impl ProductPowers {
    fn new() -> ProductPowers {
        ProductPowers {
            powers: HashMap::new(),
            mul: Rational::from(1)
        }
    }
    fn from_product<I>(parts: I) -> ProductPowers where I: Iterator<Item=Node> {
        let mut p = ProductPowers::new();
        for n in parts {
            p.mul(n);
        }
        p
    }
    fn mul_power(&mut self, n: Node, pow: Node) {
        self.powers.entry(n).or_insert(Vec::new()).push(pow);
    }
    fn mul(&mut self, n: Node) {
        match simplify(n) {
            Node::Int(1) => {},
            Node::Int(i) => self.mul *= i,
            Node::Prod(parts) => {
                for n in parts {
                    self.mul(n);
                }
            },
            Node::Pow(box (n, pow)) => self.mul_power(n, pow),
            n => self.mul_power(n, Node::Int(1))
        }
    }
    fn to_node(self) -> Node {
        let mut parts = vec![];
        let (num, denom) = self.mul.frac().unwrap();
        if num != 1 {
            parts.push(Node::Int(num));
        }
        if denom != 1 {
            parts.push(Node::Pow(box (Node::Int(denom), Node::Int(-1))));
        }
        for (base, exp) in self.powers {
            match simplify_sum(exp.into_iter()) {
                Node::Int(0) => {},
                Node::Int(1) => parts.push(base),
                exp => parts.push(Node::Pow(box (base, exp)))
            }
        }
        
        if parts.contains(&Node::Int(0)) {
            Node::Int(0)
        } else {
            match parts.len() {
                0 => Node::Int(1),
                1 => parts.pop().unwrap(),
                _ => Node::Prod(parts)
            }
        }
    }
}

pub fn simplify(n: Node) -> Node {
    match n {
        Node::Prod(parts) => ProductPowers::from_product(parts.into_iter()).to_node(),
        Node::Sum(parts) => simplify_sum(parts.into_iter()),
        Node::Pow(box fg) => match fg {
            (Node::Int(1), _) => Node::Int(1),
            (f, Node::Int(1)) => simplify(f),
            (f, Node::Int(0)) => Node::Int(1),
            (f, g) => Node::Pow(box (f, g))
        },  
        n => n
    }
}
