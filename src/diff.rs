use std::iter::once;

use func::Func;
use node::Node;

pub fn diff(node: &Node, var: &str) -> Node {
    let out = match *node {
        Node::Int(_) => Node::Int(0),
        Node::Sum(ref parts) => Node::Sum(parts.iter().map(|n| diff(n, var)).collect()),
        Node::Prod(ref parts) => {
            Node::Sum(
                (0 .. parts.len()).map(|i| {
                    Node::Prod(
                        parts.iter().enumerate().map(|(j, f)| {
                            if i == j {
                                diff(f, var)
                            } else {
                                f.clone()
                            }
                        }).collect()
                    )
                }).collect()
            )
        },
        Node::Pow(box (ref f, ref g)) => {
            // f(x)^g(x) ( log f(x) · g'(x) + g(x) f'(x) f(x)^-1 )
            Node::Prod(vec![
                Node::Pow(box( // f(x)^g(x)
                    f.clone(),
                    g.clone()
        	)),
                Node::Sum(vec![ // log f(x) · g'(x) + g(x) f'(x) f(x)^-1
                    Node::Prod(vec![ // log f(x) · g'(x)
                        Node::Func(Func::Log, box f.clone()), // log f(x)
                        diff(g, var)
                    ]),
                    Node::Prod(vec![ // g(x) f'(x) f(x)^-1
                        g.clone(),
                        diff(f, var),
                        Node::Pow(box(f.clone(), Node::Int(-1))) // f(x)^-1
                    ])
                ])
            ])
        },
        Node::Func(f, box ref g) => {
            match f {
                Func::Sin => Node::Prod(vec![
                    Node::Func(Func::Cos, box g.clone()),
                    diff(g, var)
                ]),
                Func::Cos => Node::Prod(vec![
                    Node::Int(-1),
                    Node::Func(Func::Sin, box g.clone()),
                    diff(g, var)
                ]),
                Func::Log => Node::Prod(vec![
                    Node::Pow(box(g.clone(), Node::Int(-1))),
                    diff(g, var)
                ]),
                Func::Exp => Node::Prod(vec![
                    Node::Func(Func::Exp, box g.clone()),
                    diff(g, var)
                ])
            }
        },
        Node::Var(ref s) => {
            if s == var {
                Node::Int(1)
            } else {
                Node::Int(0)
            }
        },
    };
    println!("d/d{} {} = {}", var, node, out);
    out
}
