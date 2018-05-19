use prelude::*;
use poly::{Poly, cmp_base};
use itertools::Itertools;
use std::fmt::{self, Display};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Mode {
    Text,
    LaTeX
}
use self::Mode::*;

fn int_super(i: &Int) -> String {
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

pub struct Tokens {
    content: Vec<String>
}
impl Display for Tokens {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut iter = self.content.iter();
        if let Some(first) = iter.next() {
            f.write_str(&first)?;
            for s in iter {
                f.write_str(" ")?;
                f.write_str(&s)?;
            }
        }
        Ok(())
    }
}
fn wrap_poly(p: &Poly, mode: &Mode) -> String {
    let tokens = Tokens::poly(p, mode);
    if tokens.len() > 1 {
        match *mode {
            Text => format!("({})", tokens),
            LaTeX => format!("\\left( {} \\right)", tokens)
        }
    } else {
        tokens.to_string()
    }
}

impl Tokens {
    pub fn new() -> Tokens {
        Tokens { content: vec![] }
    }
    pub fn len(&self) -> usize {
        self.content.len()
    }
    pub fn push<T: Display>(&mut self, t: T) {
        self.content.push(t.to_string());
    }
    // add a fraction
    pub fn push_frac<N: Display, D: Display>(&mut self, nom: N, denom: D, mode: &Mode) {
        match *mode {
            Text => {
                self.push(nom);
                self.push("/");
                self.push(denom);
            },
            LaTeX => self.push(format!("\\frac{{{}}}{{{}}}", nom, denom))
        }
    }
    pub fn poly(p: &Poly, mode: &Mode) -> Tokens {
        let mut tokens = Tokens::new();
        let mut elements: Vec<_> = p.factors().collect();
        elements.sort_by(|a, b| cmp_base(&a.0, &b.0));

        for (n, &(base, fac)) in elements.iter().enumerate() {
            let mut mid = Tokens::new();
            for (i, &(ref v, ref n)) in base.iter().enumerate() {
                match *mode {
                    LaTeX if i > 0 => mid.push("\\,"),
                    _ => {}
                }
                mid.push(match (&**v, *mode) {
                    (v, _) if *n == 1 => format!("{}", Tokens::node(v, mode)),
                    (&Node::Poly(ref p), Text) => format!("{}{}", wrap_poly(p, mode), int_super(n)),
                    (&Node::Poly(ref p), LaTeX) => format!("{{{}}}^{{{}}}", wrap_poly(p, mode), n),
                    (v, Text) => format!("{}{}", Tokens::node(v, mode), int_super(n)),
                    (v, LaTeX) => format!("{{{}}}^{{{}}}", Tokens::node(v, mode), n)
                });
            }
            
            let (nom, denom) = fac.frac();

            if nom.is_negative() {
                tokens.push("−");
            } else if n != 0 {
                tokens.push("+");
            }

            let nom = nom.abs();
            let nom_is_one = nom == 1;
            let denom_is_one = denom == 1;
            match (nom_is_one, denom_is_one, mid.len(), *mode) {
                (_,    true, 0, _) => tokens.push(nom),
                (true, true, _, _) => tokens.push(mid),
                (true, _,    0, Text) => tokens.push_frac(1, denom, mode),
                (true, _,    _, Text) => tokens.push_frac(mid, denom, mode),
                (_,    true, _, _) => {
                    tokens.push(nom);
                    tokens.push(mid);
                },
                (_,    _,    _, _) => {
                    tokens.push_frac(nom, denom, mode);
                    tokens.push(mid);
                },
            }
        }
        if tokens.len() == 0 {
            tokens.push("0");
        }
        tokens
    }
    pub fn node(n: &Node, mode: &Mode) -> Tokens {
        let mut tokens = Tokens::new();
        match (&*n, *mode) {
            (&Node::Op(ref f), _) => tokens.push(f),
            (&Node::Apply(ref f, ref g), Text) => tokens.push(format!("{}({})", Tokens::node(f, mode), Tokens::node(g, mode))),
            (&Node::Apply(ref f, ref g), LaTeX) => tokens.push(format!("{} \\left( {} \\right)", Tokens::node(f, mode), Tokens::node(g, mode))),
            (&Node::Poly(ref p), _) => {
                match p.factorize() {
                    Some((p, q)) => {
                        tokens.push(wrap_poly(&p, mode));
                        tokens.push(wrap_poly(&q, mode));
                    },
                    None => tokens.push(Tokens::poly(p, mode))
                }
            }
            (&Node::Var(ref name), _) => tokens.push(name),
            (&Node::Tuple(ref parts), Text) => tokens.push(format!("({})", parts.iter().map(|n| Tokens::node(n, mode)).join(", "))),
            (&Node::Tuple(ref parts), LaTeX) => tokens.push(format!(r"\left( {} \right)", parts.iter().map(|n| Tokens::node(n, mode)).join(", "))),
        }
        tokens
    }
}
