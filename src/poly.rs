use prelude::*;
use std::iter::once;
use std::collections::hash_map::{HashMap, Entry, Iter};
use std::ops::{Add, Mul, MulAssign};
use std::cmp::{min, max, PartialEq, Eq, PartialOrd, Ord, Ordering};
use std::hash::{Hash, Hasher};
use std::{fmt};

pub type Base = Vec<(NodeRc, Int)>;
#[derive(Debug, Clone)]
pub struct Poly {
    // never contains a zero
    elements: HashMap<Base, Rational>,
}

#[derive(Debug, Clone)]
pub enum PolyError {
    DivZero
}
fn add_to<'a>(e: Entry<'a, Base, Rational>, r: Rational) {
    match e {
        Entry::Vacant(v) => {
            v.insert(r);
        },
        Entry::Occupied(mut o) => {
            if {
                let q = o.get_mut();
                *q += r;
                q.is_zero()
            } {
                o.remove();
            }
        }
    }
}

fn base<I>(bv: I) -> Base where I: IntoIterator<Item=(NodeRc, Int)>
{
    let mut base: Vec<_> = bv.into_iter()
        .filter(|&(_, ref n)| *n != 0)
        .collect();
    base.sort();
    base
}

impl Poly {
    fn one(bv: Base, fac: Rational) -> Poly {
        Poly { elements: once((base(bv), fac)).collect() }
    }
    pub fn zero() -> Poly {
        Poly { elements: HashMap::new() }
    }
    pub fn rational(r: Rational) -> Poly {
        if r.is_zero() {
            Poly::zero()
        } else {
            Poly::one(vec![], r)
        }
    }
    pub fn int<I: Into<Int>>(i: I) -> Poly {
        let i: Int = i.into();
        Poly::rational(i.into())
    }
    pub fn from_node(node: NodeRc) -> Poly {
        if let Node::Poly(ref p) = *node {
            return p.clone();
        }
                
        Poly::one(vec![(node, 1.into())], 1.into())
    }
    pub fn pow(self, builder: &Builder, i: Int) -> Result<Poly, Error> {
        if let Some(i) = i.as_i32() {
            self.pow_i(builder, i)
        } else {
            Ok(Poly::one(vec![(builder.poly(self), i)], 1.into()))
        }
    }
    pub fn pow_i(self, builder: &Builder, i: i32) -> Result<Poly, Error> {
        if i == 0 {
            return Ok(Poly::int(1));
        }
        if let Some(r) = self.as_rational() {
            // r^i
            if r.is_zero() && i.is_negative() {
                // 1 / (0^-i)
                return Err(PolyError::DivZero.into());
            }
            return Ok(Poly::rational(r.pow(i)));
        }
        
        if self.elements.len() == 1 {
            let (base, fac) = self.elements.into_iter().next().unwrap();
            let base = base.into_iter().map(|(v, n)| (v, n * i)).collect();
            return Ok(Poly::one(base, fac.pow(i)));
        }
        if i > 0 && i < 4 {
            return Ok(self.pow_n(i as u32));
        }
        Ok(Poly::one(vec![(builder.poly(self), i.into())], 1.into()))
    }
    pub fn pow_n(mut self, mut n: u32) -> Poly {
        let mut p = Poly::int(1);
        while n > 1 {
            if n & 1 == 1 {
                p = p * self.clone();
            }
            n /= 2;
            self = self.clone() * self.clone();
        }
        if n == 1 {
            p = p * self
        }
        p
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
            1 => self.elements.get(&vec![]).cloned(),
            _ => None
        }
    }
    pub fn to_int(&self) -> Option<Int> {
        self.as_rational().and_then(|r| r.to_int())
    }
    // split into vec of polynoms, each containing one summand
    pub fn split(&self) -> Vec<Poly> {
        let mut out = Vec::with_capacity(self.elements.len());
        for (base, fac) in self.elements.iter() {
            out.push(Poly::one(base.clone(), fac.clone()));
        }
        out
    }
    pub fn factorize(&self) -> Option<(Poly, Poly)> {
        // find the common factor
        if self.elements.len() < 2 { return None; }

        let mut factors = self.factors();
        let mut common: Base = factors.next().unwrap().0.clone();
        
        for (ref bv, _) in factors {
            for &mut (ref v, ref mut n) in common.iter_mut() {
                if let Some(&(_, ref m)) = bv.iter().find(|&&(ref w, _)| w == v) {
                    *n = min(m, n).clone();
                } else {
                    *n = 0.into();
                }
            }
            common.retain(|&(_, ref n)| *n != 0);
        }
        if common.len() == 0 {
            return None;
        }

        // common now contains the common factor
        let elements = self.factors().map(|(bv, rat)| {
            (
                base(bv.iter().map(|&(ref v, ref n)| {
                    match common.iter().find(|&&(ref w, _)| w == v) {
                        Some(&(_, ref m)) => (v.clone(), n.clone() - m),
                        None => (v.clone(), n.clone())
                    }
                })),
                rat.clone()
            )
        }).collect();
        
        let poly = Poly { elements };
        let common_poly = Poly::one(common.into_iter().collect(), 1.into());
        Some((common_poly, poly))
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
        for ((a_base, a_fac), (b_base, b_fac)) in self.elements.iter().cartesian_product(rhs.elements.iter()) {
            // multiply base vector by adding powers
            let mut base = a_base.clone();
            for &(ref v, ref n) in b_base.iter() {
                match base.iter().position(|b| *v == b.0) {
                    Some(i) => {
                        base[i].1 += &n;
                        if base[i].1 == 0 {
                            base.swap_remove(i);
                        }
                    }
                    None => base.push((v.clone(), n.clone()))
                }
            }
            base.sort_by(|a, b| match a.0.cmp(&b.0) {
                Ordering::Equal => a.1.cmp(&b.1),
                o => o
            });
            add_to(elements.entry(base), a_fac * b_fac);
        }
        Poly { elements }
    }
}

impl<T: Into<Int>> Mul<T> for Poly {
    type Output = Poly;
    fn mul(mut self, rhs: T) -> Poly {
        self *= rhs.into();
        self
    }
}
impl MulAssign<Int> for Poly {
    fn mul_assign(&mut self, rhs: Int) {
        *self *= Rational::from(rhs);
    }
}
impl MulAssign<Rational> for Poly {
    fn mul_assign(&mut self, rhs: Rational) {
        for fac in self.elements.values_mut() {
            *fac *= &rhs;
        }
    }
}

impl PartialEq for Poly {
    fn eq(&self, rhs: &Poly) -> bool {
        (self.elements.len() == rhs.elements.len()) && self.elements.iter().all(|(k, v)| rhs.elements.get(k) == Some(v))
    }
}
impl Eq for Poly {}

impl PartialOrd for Poly {
    fn partial_cmp(&self, rhs: &Poly) -> Option<Ordering> {
        Some(cmp_poly(self, rhs))
    }
}
impl Ord for Poly {
    fn cmp(&self, rhs: &Poly) -> Ordering {
        cmp_poly(self, rhs)
    }
}

fn cmp_poly(a: &Poly, b: &Poly) -> Ordering {
    match a.elements.len().cmp(&b.elements.len()) {
        Ordering::Equal => {
            let mut e_a: Vec<_> = a.elements.iter().collect();
            let mut e_b: Vec<_> = b.elements.iter().collect();
            e_a.sort();
            e_b.sort();
            for (a, b) in e_a.iter().zip(e_b.iter()) {
                match cmp_base(a.0, b.0) {
                    Ordering::Equal => continue,
                    o => return o
                }
            }
            Ordering::Equal
        },
        o => o
    }
}

pub fn cmp_base(a: &[(NodeRc, Int)], b: &[(NodeRc, Int)]) -> Ordering {
    match a.len().cmp(&b.len()) {
        Ordering::Equal => {},
        o => return o
    }
    for (a, b) in a.iter().zip(b.iter()) {
        match a.0.cmp(&b.0) {
            Ordering::Equal => continue,
            o => return o
        }
    }
    for (a, b) in a.iter().zip(b.iter()) {
        match a.1.cmp(&b.1) {
            Ordering::Equal => continue,
            o => return o
        }
    }
    Ordering::Equal
}

impl Hash for Poly {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for f in self.factors() {
            f.hash(state);
        }
    }
}

impl fmt::Display for Poly {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        use display::*;
        Tokens::poly(self, &Mode::Text).fmt(w)
    }
}
