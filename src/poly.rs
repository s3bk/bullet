use node::NodeRc;
use rational::Rational;
use std::iter::once;
use std::collections::hash_map::{HashMap, Entry, Iter};
use std::ops::{Add, Mul, MulAssign};
use std::fmt;
use std::cmp::{max, PartialEq, Eq};
use std::hash::{Hash, Hasher};

pub type Base = Vec<(NodeRc, i64)>;
#[derive(Debug)]
pub struct Poly {
    // never contains a zero
    elements: HashMap<Base, Rational>,
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
        Poly {
            elements: once((vec![(node, 1)], 1.into())).collect()
        }
    }
    pub fn pow_i(self, i: i32) -> Poly {
        let elements = self.elements.into_iter()
            .map(|((v, n), fac)| ((v, n * i), fac.pow(i)))
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
            1 => self.elements.get(vec![]).map(|r| Some(r)).unwrap_or(None),
            _ => None
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        self.as_rational().and_then(|r| r.as_int())
    }
}
        
impl Add for Poly {
    type Output = Poly;
    fn add(mut self, rhs: Poly) -> Poly {
        for (base, fac) in rhs.into_iter() {
            match self.entry(base) {
                Entry::Vacant(v) => {
                    v.insert(fac);
                },
                Entry::Occupied(mut o) => {
                    *o.get_mut() += fac;
                    if (*o.get()).is_zero() {
                        o.remove();
                    }
                }
            }
        }
        self
    }
}
impl Mul for Poly {
    type Output = Poly;
    fn mul(self, rhs: Poly) -> Poly {
        let mut elements = HashMap::with_capacity(max(self.elements.len(), rhs.elements.len()));
        for ((a_base, a_fac), (b_base, b_fac)) in self.elements.iter().catesian_product(rhs.elements.iter()) {
            // multiply base vector by adding powers
            let mut base = a_base.clone();
            for (v, n) in b_base.iter() {
                *base.entry(v).or_insert(0) += n;
            }
            base.sort();

            *elements.entry(base).or_insert(0.into()) += a_fac * b_fac;
        }
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
        match self.elements.entry(vec![]) {
            Entry::Occupied(mut o) => *o.get_mut() *= rhs,
            _ => {} // zero-case
        }
    }
}

impl PartialEq for Poly {
    fn eq(&self, rhs: &Poly) -> bool {
        (self.elements.len() == rhs.elements.len()) && self.elements.iter().all(|k, v| rhs.elements.get(k) == Some(v))
    }
}
impl Eq for Poly {}

impl fmt::Display for Poly {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.elements.len() > 0 {
            write!(f, "(")?;
        }
        for (base, fac) in self.elements.iter() {
            fac.fmt(f)?;
            for (v, n) in base.iter() {
                write!(f, " ")?;
                v.fmt(f)?;
                if n != 1 {
                    int_super(n, f)?;
                }
            }
        }
        if self.elements.len() > 0 {
            write!(f, ")");
        }
        Ok(())
    }
}
impl Hash for Poly {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for f in self.factors() {
            f.hash(state);
        }
    }
}

fn int_super(i: i64, f: &mut fmt::Formatter) {
    for c in i.to_string().chars() {
        f.write_char(
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
        )?;
    }
    Ok(())
}
