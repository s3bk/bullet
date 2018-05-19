use std::ops::{MulAssign, AddAssign, DivAssign, Add, Sub, Mul, Div};
use std::cmp::{Ordering, Ord, PartialEq};
use std::fmt;
use num_rational::{BigRational};
use num_bigint::{BigInt, Sign};
use num_traits::{ToPrimitive, Zero, One, Signed};
use error::Error;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct Int(BigInt);
impl Int {
    pub fn bits(&self) -> usize {
        self.0.bits()
    }
    pub fn as_i32(&self) -> Option<i32> {
        self.0.to_i32()
    }
    pub fn as_i64(&self) -> Option<i64> {
        self.0.to_i64()
    }
    pub fn as_f64(&self) -> f64 {
        self.0.to_f64().unwrap_or(::std::f64::NAN)
    }
    pub fn is_negative(&self) -> bool {
        self.0.sign() == Sign::Minus
    }
    pub fn parse(s: &str, radix: u32) -> Result<Int, Error> {
        match BigInt::parse_bytes(s.as_bytes(), radix) {
            Some(i) => Ok(Int(i)),
            None => Err(Error::IntegerError)
        }
    }
    pub fn square(&mut self) {
        self.0 = &self.0 * &self.0;
    }
    pub fn pow(&self, mut n: u32) -> Int {
        if n == 0 {
            return Int::from(0);
        }
        let mut x = self.clone();
        // handle trailing powers (replace x by x²ⁿ)
        for _ in 0 .. n.trailing_zeros() {
            x.square();
            n /= 2;
        }

        // for powers of two, the computation is complete
        if n == 1 {
            return x;
        }
        
        let mut y = x.clone(); // holds the power so far
        while n > 1 {
            if n & 1 == 1 {
                y *= &x;
            }

            x.square();
            n /= 2;
        }

        assert_eq!(n, 1);
        x * y // final multiplication
    }
    pub fn abs(self) -> Int {
        Int(self.0.abs())
    }
}
impl From<i32> for Int {
    fn from(i: i32) -> Int {
        Int(i.into())
    }
}
impl From<i64> for Int {
    fn from(i: i64) -> Int {
        Int(i.into())
    }
}
impl From<bool> for Int {
    fn from(i: bool) -> Int {
        Int::from(i as i64)
    }
}
impl<'a> Add<&'a Int> for Int {
    type Output = Int;
    fn add(self, rhs: &'a Int) -> Int {
        Int(self.0 + &rhs.0)
    }
}
impl<'a, 'b> Add<&'a Int> for &'b Int {
    type Output = Int;
    fn add(self, rhs: &'a Int) -> Int {
        Int(&self.0 + &rhs.0)
    }
}
impl<T> Add<T> for Int where T: Into<Int> {
    type Output = Int;
    fn add(self, rhs: T) -> Int {
        Int(self.0 + rhs.into().0)
    }
}
impl<'a> Sub<&'a Int> for Int {
    type Output = Int;
    fn sub(self, rhs: &'a Int) -> Int {
        Int(self.0 - &rhs.0)
    }
}
impl<T> Sub<T> for Int where T: Into<Int> {
    type Output = Int;
    fn sub(self, rhs: T) -> Int {
        Int(self.0 - rhs.into().0)
    }
}
impl<'a> Mul<&'a Int> for Int {
    type Output = Int;
    fn mul(self, rhs: &'a Int) -> Int {
        Int(self.0 * &rhs.0)
    }
}
impl<'a, 'b> Mul<&'a Int> for &'b Int {
    type Output = Int;
    fn mul(self, rhs: &'a Int) -> Int {
        Int(&self.0 * &rhs.0)
    }
}
impl<T> Mul<T> for Int where T: Into<Int> {
    type Output = Int;
    fn mul(self, rhs: T) -> Int {
        Int(self.0 * rhs.into().0)
    }
}
impl<'a> AddAssign<&'a Int> for Int {
    fn add_assign(&mut self, other: &'a Int) {
        self.0 = &self.0 + &other.0;
    }
}
impl<'a> MulAssign<&'a Int> for Int {
    fn mul_assign(&mut self, other: &'a Int) {
        self.0 = &self.0 * &other.0;
    }
}
impl PartialEq<i64> for Int {
    fn eq(&self, other: &i64) -> bool {
        match self.as_i64() {
            Some(i) => i.eq(other),
            None => false
        }
    }
}
impl fmt::Display for Int {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.to_str_radix(10))
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct Rational(BigRational);

impl AddAssign<Int> for Rational {
    fn add_assign(&mut self, i: Int) {
        self.0 = &self.0 * i.0;
    }
}
impl AddAssign for Rational {
    fn add_assign(&mut self, rhs: Rational) {
        self.0 = &self.0 + rhs.0;
    }
}
impl MulAssign<Int> for Rational {
    fn mul_assign(&mut self, n: Int) {
        self.0 = &self.0 * n.0;
    }
}
impl MulAssign for Rational {
    fn mul_assign(&mut self, rhs: Rational) {
        self.0 = &self.0 * rhs.0;
    }
}
impl<'a> MulAssign<&'a Rational> for Rational {
    fn mul_assign(&mut self, rhs: &'a Rational) {
        self.0 = &self.0 * &rhs.0;
    }
}
impl Mul for Rational {
    type Output = Rational;
    fn mul(self, rhs: Rational) -> Rational {
        Rational(self.0 * rhs.0)
    }
}
impl<'a, 'b> Mul<&'b Rational> for &'a Rational {
    type Output = Rational;
    fn mul(self, rhs: &'b Rational) -> Rational {
        Rational(&self.0 * &rhs.0)
    }
}
impl DivAssign for Rational {
    fn div_assign(&mut self, rhs: Rational) {
        self.0 = &self.0 / rhs.0;
    }
}
impl DivAssign<Int> for Rational {
    fn div_assign(&mut self, n: Int) {
        self.0 = &self.0 / n.0;
    }
}
impl Div<Rational> for Rational {
    type Output = Rational;
    fn div(mut self, rhs: Rational) -> Rational {
        Rational(self.0 / rhs.0)
    }
}
impl Rational {
    pub fn new(num: Int, denom: Int) -> Rational {
        Rational(BigRational::new(num.0, denom.0))
    }
    pub fn frac(&self) -> (Int, Int) {
        let (num, denom) = self.0.clone().into();
        (Int(num), Int(denom))
    }
    pub fn to_int(&self) -> Option<Int> {
        if self.0.denom().is_one() {
            Some(Int(self.0.numer().clone()))
        } else {
            None
        }
    }
    pub fn as_i64(&self) -> Option<i64> {
        if self.0.denom().is_one() {
            self.0.numer().to_i64()
        } else {
            None
        }
    }
    pub fn as_f64(&self) -> f64 {
        match (self.0.numer().to_f64(), self.0.denom().to_f64()) {
            (Some(n), Some(d)) => n / d,
            _ => ::std::f64::NAN
        }
    }
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    pub fn is_negative(&self) -> bool {
        (self.0.numer().sign() == Sign::Minus) ^ (self.0.denom().sign() == Sign::Minus)
    }
    pub fn pow(&self, i: i32) -> Rational {
        match i.cmp(&0) {
            Ordering::Greater => {
                let (num, denom) = self.frac();
                let n = i as u32;
                Rational::new(num.pow(n), denom.pow(n))
            },
            Ordering::Equal => Rational::from(1),
            Ordering::Less => {
                let (denom, num) = self.frac();
                let n = -i as u32;
                Rational::new(num.pow(n), denom.pow(n))
            }
        }
    }
}
impl From<i64> for Rational {
    fn from(i: i64) -> Rational {
        Rational::new(i.into(), 1.into())
    }
}
impl From<Int> for Rational {
    fn from(i: Int) -> Rational {
        Rational::new(i, 1.into())
    }
}
        
impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (n, d) = self.frac();
        if d == 1 {
            write!(f, "{}", n)
        } else {
            write!(f, "{}/{}", n, d)
        }
    }
}
