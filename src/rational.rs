use node::Node;
use std::ops::{MulAssign, AddAssign, DivAssign};

fn gcd(mut ab: (i64, i64)) -> i64 {
    loop {
        match ab {
            (a, 0) => break a,
            (a, b) => ab = (b, a % b)
        }
    }
}
#[derive(Copy, Clone)]
pub struct Rational {
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
    pub fn frac(&self) -> Result<(i64, i64), &'static str> {
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
