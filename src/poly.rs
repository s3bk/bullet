use prelude::*;
use rational::Rational;
use std::iter::once;
use std::collections::hash_map::{HashMap, Entry, Iter};
use std::ops::{Add, Mul, MulAssign};
use std::cmp::{min, max, PartialEq, Eq, PartialOrd, Ord, Ordering};
use std::hash::{Hash, Hasher};
use std::fmt;

pub type Base = Vec<(NodeRc, i64)>;
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
            *o.get_mut() += r;
            if (*o.get()).is_zero() {
                o.remove();
            }
        }
    }
}

fn base<I>(bv: I) -> Base where I: IntoIterator<Item=(NodeRc, i64)>
{
    let mut base: Vec<_> = bv.into_iter().filter(|&(_, n)| n != 0).collect();
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
    pub fn int(i: i64) -> Poly {
        Poly::rational(i.into())
    }
    pub fn from_node(node: NodeRc) -> Poly {
        if let Node::Poly(ref p) = *node {
            return p.clone();
        }
                
        Poly::one(vec![(node, 1)], 1.into())
    }
    pub fn pow_i(self, builder: &Builder, i: i32) -> Result<Poly> {
        if i == 0 {
            return Ok(Poly::int(1));
        }
        if let Some(r) = self.as_rational() {
            if r.is_zero() && i < 0 {
                return Err(PolyError::DivZero.into());
            } else {
                return Ok(Poly::rational(r.pow(i)));
            }
        }

        if self.elements.len() == 1 {
            let (base, fac) = self.elements.into_iter().next().unwrap();
            let base = base.into_iter().map(|(v, n)| (v, n * i as i64)).collect();
            let fac = fac.pow(i);
            return Ok(Poly::one(base, fac));
        }
        if i > 0 && i < 4 {
            return Ok(self.pow_n(i as u32));
        }
        Ok(Poly::one(vec![(builder.poly(self), i as i64)], 1.into()))
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
            out.push(Poly::one(base.clone(), fac));
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
                if let Some(&(_, m)) = bv.iter().find(|&&(ref w, _)| w == v) {
                    *n = min(m, *n);
                } else {
                    *n = 0;
                }
            }
            common.retain(|&(_, n)| n != 0);
        }
        if common.len() == 0 {
            return None;
        }

        // common now contains the common factor
        let elements = self.factors().map(|(bv, &rat)| {
            (
                base(bv.iter().map(|&(ref v, n)| {
                    match common.iter().find(|&&(ref w, _)| w == v) {
                        Some(&(_, m)) => (v.clone(), n - m),
                        None => (v.clone(), n)
                    }
                })),
                rat
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
            base.sort_by(|a, b| match a.0.cmp(&b.0) {
                Ordering::Equal => a.1.cmp(&b.1),
                o => o
            });
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

pub fn cmp_base(a: &[(NodeRc, i64)], b: &[(NodeRc, i64)]) -> Ordering {
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
        use display::Tokens;
        Tokens::poly(self).fmt(w)
    }
}
