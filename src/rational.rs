use std::ops::{MulAssign, AddAssign, DivAssign, Mul};
use std::cmp::Ordering;
use std::fmt;


fn gcd(mut ab: (i64, i64)) -> i64 {
    loop {
        match ab {
            (a, 0) => break a,
            (a, b) => ab = (b, a % b)
        }
    }
}
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Rational {
    num: i64,
    denom: i64
}
impl AddAssign<i64> for Rational {
    fn add_assign(&mut self, i: i64) {
        self.num += i * self.denom;
    }
}
impl AddAssign for Rational {
    fn add_assign(&mut self, rhs: Rational) {
        let denom = self.denom * rhs.denom;
        self.num = self.num * rhs.denom + rhs.num * self.denom;
        self.denom = denom;
    }
}
impl MulAssign<i64> for Rational {
    fn mul_assign(&mut self, n: i64) {
        self.num *= n;
    }
}
impl MulAssign for Rational {
    fn mul_assign(&mut self, rhs: Rational) {
        self.num *= rhs.num;
        self.denom *= rhs.denom;
        self.normalize();
    }
}
impl Mul for Rational {
    type Output = Rational;
    fn mul(self, rhs: Rational) -> Rational {
        Rational::new(self.num * rhs.num, self.denom * rhs.denom)
    }
}
impl DivAssign for Rational {
    fn div_assign(&mut self, rhs: Rational) {
        self.num *= rhs.denom;
        self.denom *= rhs.num;
        self.normalize();
    }
}
impl DivAssign<i64> for Rational {
    fn div_assign(&mut self, n: i64) {
        self.denom *= n;
        self.normalize();
    }
}
impl Rational {
    pub fn new(num: i64, denom: i64) -> Rational {
        let mut r = Rational { num, denom };
        r.normalize();
        r
    }
    fn normalize(&mut self) {
        let (n, d) = match (self.num, self.denom) {
            (0, 0) => panic!("undefined rational"),
            (_, 0) => panic!("infinite rational"),
            (0, _) => (0, 1),
            (n, 1) => (n, 1),
            (1, -1) => (1, 1),
            (mut n, mut d) => {
                if d < 0 {
                    n = -n;
                    d = -d;
                }
                let c = gcd((n, d));
                (n/c, d/c)
            }
        };

        self.num = n;
        self.denom = d;
    }
    pub fn frac(&self) -> (i64, i64) {
        (self.num, self.denom)
    }
    pub fn as_int(&self) -> Option<i64> {
        match self.frac() {
            (i, 1) => Some(i),
            _ => None
        }
    }
    pub fn is_zero(&self) -> bool {
        self.num == 0
    }

    pub fn is_negative(&self) -> bool {
        (self.num < 0) ^ (self.denom < 0)
    }
    pub fn pow(&self, i: i32) -> Rational {
        match i.cmp(&0) {
            Ordering::Greater => Rational {
                num: self.num.pow(i as u32),
                denom: self.denom.pow(i as u32)
            },
            Ordering::Equal => Rational::from(1),
            Ordering::Less => Rational {
                num: self.denom.pow(-i as u32),
                denom: self.num.pow(-i as u32)
            }
        }
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
impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.frac() {
            (n, 1) => write!(f, "{}", n),
            (n, d) => write!(f, "{}/{}", n, d),
        }
    }
}
