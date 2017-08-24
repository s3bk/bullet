use node::{Node, NodeRc};
use rational::Rational;
use std::iter::once;
use std::collections::hash_map::{HashMap, Entry, Iter};
use std::ops::{Add, Mul, MulAssign};
use std::fmt::{self, Write};
use std::cmp::{max, PartialEq, Eq};
use std::hash::{Hash, Hasher};
use itertools::Itertools;

pub type Base = Vec<(NodeRc, i64)>;
#[derive(Debug, Clone)]
pub struct Poly {
    // never contains a zero
    elements: HashMap<Base, Rational>,
}

fn add_to<'a>(e: Entry<'a, Base, Rational>, r: Rational) {
    match e {
        Entry::Vacant(v) => {
            v.insert(r);
        },
        Entry::Occupied(mut o) => {
            *o.get_mut() += r;
            if (*o.get()).is_zero() {
                o.remove();
            }
        }
    }
}

impl Poly {
    pub fn rational(r: Rational) -> Poly {
        Poly {
            elements: if r.is_zero() {
                HashMap::new()
            } else {
                once((vec![], r)).collect()
            }
        }
    }
    pub fn int(i: i64) -> Poly {
        Poly::rational(i.into())
    }
    pub fn from_node(node: NodeRc) -> Poly {
        if let Node::Poly(ref p) = *node {
            return p.clone();
        }
                
        Poly {
            elements: once((vec![(node, 1)], 1.into())).collect()
        }
    }
    pub fn pow_i(self, i: i32) -> Poly {
        if i == 0 {
            return Poly::int(1);
        }
        let elements = self.elements.into_iter()
            .map(|(base, fac)| (
                base.into_iter().map(|(v, n)| (v, n * i as i64)).collect(),
                fac.pow(i)
            ))
            .collect();
        Poly { elements }
    }
    pub fn is_zero(&self) -> bool {
        self.elements.len() == 0
    }
    pub fn factors(&self) -> Iter<Base, Rational> {
        self.elements.iter()
    }
    pub fn as_rational(&self) -> Option<Rational> {
        match self.elements.len() {
            0 => Some(0.into()),
            1 => self.elements.get(&vec![]).map(|&r| Some(r)).unwrap_or(None),
            _ => None
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        self.as_rational().and_then(|r| r.as_int())
    }
    // split into vec of polynoms, each containing one summand
    pub fn split(&self) -> Vec<Poly> {
        let mut out = Vec::with_capacity(self.elements.len());
        for (base, &fac) in self.elements.iter() {
            out.push(Poly { elements: once((base.clone(), fac)).collect() });
        }
        out
    }
}
        
impl Add for Poly {
    type Output = Poly;
    fn add(mut self, rhs: Poly) -> Poly {
        for (base, fac) in rhs.elements.into_iter() {
            add_to(self.elements.entry(base), fac);
        }
        self
    }
}
impl Mul for Poly {
    type Output = Poly;
    fn mul(self, rhs: Poly) -> Poly {
        let mut elements = HashMap::with_capacity(max(self.elements.len(), rhs.elements.len()));
        for ((a_base, &a_fac), (b_base, &b_fac)) in self.elements.iter().cartesian_product(rhs.elements.iter()) {
            // multiply base vector by adding powers
            let mut base = a_base.clone();
            for &(ref v, n) in b_base.iter() {
                match base.iter().position(|b| *v == b.0) {
                    Some(i) => {
                        base[i].1 += n;
                        if base[i].1 == 0 {
                            base.swap_remove(i);
                        }
                    }
                    None => base.push((v.clone(), n))
                }
            }
            base.sort();
            add_to(elements.entry(base), a_fac * b_fac);
        }
        Poly { elements }
    }
}

impl Mul<i64> for Poly {
    type Output = Poly;
    fn mul(mut self, rhs: i64) -> Poly {
        self *= rhs;
        self
    }
}
impl MulAssign<i64> for Poly {
    fn mul_assign(&mut self, rhs: i64) {
        *self *= Rational::from(rhs);
    }
}
impl MulAssign<Rational> for Poly {
    fn mul_assign(&mut self, rhs: Rational) {
        for fac in self.elements.values_mut() {
            *fac *= rhs;
        }
    }
}

impl PartialEq for Poly {
    fn eq(&self, rhs: &Poly) -> bool {
        (self.elements.len() == rhs.elements.len()) && self.elements.iter().all(|(k, v)| rhs.elements.get(k) == Some(v))
    }
}
impl Eq for Poly {}

struct Tokens {
    content: String
}
impl Tokens {
    fn new() -> Tokens {
        Tokens { content: String::new() }
    }
    fn push<T: fmt::Display>(&mut self, t: T) -> fmt::Result {
        if self.content.len() > 0 {
            write!(self.content, " ")?;
        }
        write!(self.content, "{}", t)
    }
}

impl fmt::Display for Poly {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut tokens = Tokens::new();
    
        for (n, (base, fac)) in self.elements.iter().enumerate() {
            let (nom, denom) = fac.frac();

            if nom < 0 {
                tokens.push("-")?;
            } else if n != 0 {
                tokens.push("+")?;
            }
            if nom.abs() != 1 {
                tokens.push(nom.abs())?;
            }

            for &(ref v, n) in base.iter() {
                if n == 1 {
                    tokens.push(v)?;
                } else {
                    tokens.push(format!("{}{}", v, int_super(n)))?;
                }
            }

            match denom {
                1 => {},
                d => {
                    tokens.push("/")?;
                    tokens.push(d)?;
                }
            }
        }
        if self.elements.len() > 1 {
            write!(f, "({})", tokens.content)
        } else {
            write!(f, "{}", tokens.content)
        }
    }
}
impl Hash for Poly {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for f in self.factors() {
            f.hash(state);
        }
    }
}

fn int_super(i: i64) -> String {
    i.to_string().chars().map(|c| {
        match c {
            '-' => '⁻',
            '0' => '⁰',
            '1' => '¹',
            '2' => '²',
            '3' => '³',
            '4' => '⁴',
            '5' => '⁵',
            '6' => '⁶',
            '7' => '⁷',
            '8' => '⁸',
            '9' => '⁹',
            _ => unreachable!()
        }
    }).collect()
}
