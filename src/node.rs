use itertools::Itertools;
use std::fmt;
use func::Func;

#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum Node {
    Int(i64),
    Var(String),
    Sum(Vec<Node>),
    Prod(Vec<Node>),
    Pow(Box<(Node, Node)>),
    Func(Func, Box<Node>)
}
impl Node {
    pub fn simplify(self) -> Node {
        use simplify::simplify;
        simplify(self)
    }
}

impl fmt::Display for Node {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Node::Int(i) => write!(w, "{}", i),
            Node::Sum(ref parts) => {
                match parts.len() {
                    0 => write!(w, "0"),
                    1 => write!(w, "{}", parts[0]),
                    _ => {
                        write!(w, "({}", parts[0])?;
                        for n in &parts[1..] {
                            match *n {
                                Node::Int(i) if i < 0 => write!(w, " - {}", -i)?,
                                _ => write!(w, " + {}", n)?
                            }
                        }
                        write!(w, ")")
                    }
                }
            },
            Node::Prod(ref parts) => {
                let (mut num, mut denom) = (vec![], vec![]);
                for n in parts.iter() {
                    match *n {
                        Node::Pow(box (ref f, Node::Int(-1))) => denom.push(f.clone()),
                        Node::Pow(box (ref f, Node::Int(i))) if i < 0 => denom.push(Node::Pow(box (f.clone(), Node::Int(-i)))),
                        _ => num.push(n.clone())
                    }
                }
                match num.len() {
                    0 => write!(w, "1"),
                    1 => write!(w, "{}", num[0]),
                    _ => write!(w, "({})", num.iter().join(" · "))
                }?;
                match denom.len() {
                    0 => Ok(()),
                    1 => write!(w, " / {}", denom[0]),
                    _ => write!(w, " / ({})", denom.iter().join(" · "))
                }
            },
            Node::Pow(box (ref f, ref g)) => match *g {
                Node::Int(i) => write!(w, "{}{}", f, int_super(i)),
                ref g => write!(w, "{}^{}", f, g)
            }
            Node::Func(f, box ref g) => write!(w, "{}({})", f, g),
            Node::Var(ref s) => write!(w, "{}", s)
        }
    }
}
impl fmt::UpperHex for Node {
fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Node::Int(i) => write!(w, "{}", i),
            Node::Sum(ref parts) => {
                match parts.len() {
                    0 => write!(w, "0"),
                    1 => write!(w, "{:X}", parts[0]),
                    _ => {
                        write!(w, r"\left( {:X}", parts[0])?;
                        for n in &parts[1..] {
                            match *n {
                                Node::Int(i) if i < 0 => write!(w, " - {}", -i)?,
                                _ => write!(w, " + {:X}", n)?
                            }
                        }
                        write!(w, r" \right)")
                    }
                }
            },
            Node::Prod(ref parts) => {
                let (mut num, mut denom) = (vec![], vec![]);
                for n in parts.iter() {
                    match *n {
                        Node::Pow(box (ref f, Node::Int(-1))) => denom.push(f.clone()),
                        Node::Pow(box (ref f, Node::Int(i))) if i < 0 => denom.push(Node::Pow(box (f.clone(), Node::Int(-i)))),
                        _ => num.push(n.clone())
                    }
                }
                match (num.len(), denom.len()) {
                    (0, 0) => write!(w, "1"),
                    (1, 0) => write!(w, "{}", num[0]),
                    (_, 0) => write!(w, r"\left( {} \right)", num.iter().map(|n| format!("{:X}", n)).join(" ")),
                    (_, _) => write!(w, r"\frac{{{}}}{{{}}}",
                                     num.iter().map(|n| format!("{:X}", n)).join(" "),
                                     denom.iter().map(|n| format!("{:X}", n)).join(" "))
                }
            },
            Node::Pow(box (ref f, ref g)) => write!(w, "{:X}^{{{:X}}}", f, g),
            Node::Func(f, box ref g) => write!(w, r"\{} \left( {:X} \right)", f, g),
            Node::Var(ref s) => write!(w, "{}", s)
        }
    }
}

fn int_super(i: i64) -> String {
    i.to_string().chars().map(|c| match c {
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
    }).collect()
}
