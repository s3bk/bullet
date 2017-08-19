use node::Node;
use rational::Rational;
use func::Func;
use tuple::TupleElements;
use std::collections::HashMap;

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

    fn to_node(self) -> Node {
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

pub fn simplify_sum<I>(parts: I) -> Node where I: Iterator<Item=Node> {
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
            Node::Pow(box (n, pow)) => match (n, pow) {
                (Node::Int(1), _) => {},
                (Node::Int(i), Node::Int(j)) if j > 0 && j < 100 => self.mul *= i.pow(j as u32),
                (Node::Int(i), Node::Int(j)) if j < 0 && j > -100 => self.mul /= i.pow(-j as u32),
                (n, pow) => self.mul_power(n, pow)
            },
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

pub fn simplify_prod<I>(parts: I) -> Node where I: Iterator<Item=Node> {
    ProductPowers::from_product(parts).to_node()
}

/// f^g
pub fn power(f: Node, g: Node) -> Node {
    match (f, g) {
        (Node::Int(1), _) |
        (Node::Int(0), Node::Int(0)) => Node::Int(1),
        (Node::Int(0), Node::Int(_)) => Node::Int(0),
        (f, Node::Int(1)) => simplify(f),
        (_, Node::Int(0)) => Node::Int(1),
        (Node::Pow(box (f, g)), h) => power(f, product((g, h))), // (f^g)^h = f^(g h)
        (f, g) => Node::Pow(box (f, g))
    }
}

pub fn function(f: Func, g: Node) -> Node {
    Node::Func(f, box simplify(g))
}

pub fn simplify(n: Node) -> Node {
    let r = match n.clone() {
        Node::Prod(parts) => ProductPowers::from_product(parts.into_iter()).to_node(),
        Node::Sum(parts) => simplify_sum(parts.into_iter()),
        Node::Pow(box (f, g)) => power(f, g),
        Node::Func(f, box g) => function(f, g),
        n => n
    };
    if n != r {
        println!("{:?} = {:?}", n, r);
        println!("{} = {}", n, r);
    }
    r
}

pub fn sum<T>(sum: T) -> Node where T: TupleElements<Element=Node> {
    simplify_sum(sum.into_elements())
}
pub fn product<T>(sum: T) -> Node where T: TupleElements<Element=Node> {
    simplify_prod(sum.into_elements())
}

