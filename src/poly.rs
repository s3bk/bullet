use node::Node;
use rational::Rational;
use std::iter::once;
use std::collections::hash_map::{HashMap, Entry};
use cast::Cast;
use itertools::Itertools;
use simplify;

type Powers = Vec<i64>;
type PolyElements = HashMap<Powers, Rational>;
pub struct Poly {
    variables: Vec<String>,
    elements: PolyElements
}


// returns true if the node can be represented
fn scan(node: &Node, vars: &mut Vec<String>) -> bool {
    match *node {
        Node::Int(_) => true,
        Node::Var(ref name) => {
            if !vars.contains(name) {
                vars.push(name.clone())
            }
            true
        },
        Node::Sum(ref parts) | Node::Prod(ref parts) => parts.iter().all(|p| scan(p, vars)),
        Node::Pow(box (ref f, Node::Int(i))) => Cast::<i32>::cast(i).is_some() && scan(f, vars),
        _ => false
    }
}

fn translate(vars: &[String], node: &Node) -> PolyElements {
    match *node {
        Node::Int(i) => once((vec![0; vars.len()], i.into())).collect(),
        Node::Var(ref name) => {
            let var_idx = vars.iter().position(|v| v == name).unwrap();
            let base = (0 .. vars.len()).map(|n| (n == var_idx) as i64).collect();
            once((base, 1.into())).collect()
        },
        Node::Sum(ref parts) => {
            let mut sum = translate(vars, &parts[0]);
            for n in &parts[1..] {
                add(&mut sum, translate(vars, n));
            }
            sum
        },
        Node::Prod(ref parts) => {
            let mut prod = translate(vars, &parts[0]);
            for n in &parts[1..] {
                prod = mul(&prod, &translate(vars, n));
            }
            prod
        },
        Node::Pow(box (ref f, Node::Int(i))) => {
            pow(translate(vars, f), i)
        },
        _ => unreachable!()
    }
}

fn pow(elements: PolyElements, i: i64) -> PolyElements {
    elements.into_iter().map(|(mut pow, fac)| {
        for p in pow.iter_mut() {
            *p *= i;
        }
        (pow, fac.pow(i as i32))
    }).collect()
}
fn mul(elements_a: &PolyElements, elements_b: &PolyElements) -> PolyElements {
    let mut out = HashMap::new();
    for ((pow_a, &fac_a), (pow_b, &fac_b)) in elements_a.iter().cartesian_product(elements_b.iter()) {
        let pow = pow_a.iter().zip(pow_b.iter()).map(|(a, b)| a + b).collect();
        let fac = fac_a * fac_b;
        *out.entry(pow).or_insert_with(|| Rational::from(1)) *= fac;
    }
    out
}
fn add(elements_a: &mut PolyElements, elements_b: PolyElements) {
    for (pow, fac) in elements_b.into_iter() {
        match elements_a.entry(pow) {
            Entry::Vacant(v) => {
                v.insert(fac);
            },
            Entry::Occupied(mut o) => {
                *o.get_mut() *= fac;
            }
        }
    }
}

impl Poly {
    pub fn from_node(node: &Node) -> Option<Poly> {
        let mut variables = Vec::new();
        
        // scan the tree and collect the list of variables
        if !scan(node, &mut variables) {
            return None;
        }

        // all good.
        variables.sort(); // completely unnessary, but why not...

        Some(Poly {
            elements: translate(&variables, node),
            variables
        })
    }

    pub fn to_node(&self) -> Node {
        Node::Sum(
            self.elements.iter().map(|(powers, fac)| Node::Prod(
                once(fac.to_node().expect("...")).chain(
                    self.variables.iter().zip(powers.iter())
                        .filter(|&(_, &pow)| pow != 0)
                        .map(|(var, &pow)| simplify::power(Node::Var(var.clone()), Node::Int(pow)))
                ).collect()
            )).collect()
        )
    }
}
        
 
