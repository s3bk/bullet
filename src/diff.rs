#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Func {
    Sin,
    Cos,
    Log,
    Exp
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Node {
    Sum(Vec<Node>),
    Prod(Vec<Node>),
    Pow(Box<(Node, Node)>),
    Int(i64),
    Func(Func, Box<Node>),
    X
}
pub fn diff(node: &Node) -> Node {
    match *node {
        Node::Int(_) => Node::Int(0),
        Node::Sum(ref parts) => Node::Sum(parts.iter().map(diff).collect()),
        Node::Prod(ref parts) => {
            Node::Sum(
                (0 .. parts.len()).map(|i| {
                    Node::Prod(
                        parts.iter().enumerate().map(|(j, f)| {
                            if i == j {
                                diff(f)
                            } else {
                                f.clone()
                            }
                        }).collect()
                    )
                }).collect()
            )
        },
        Node::Pow(box (ref f, ref g)) => {
            match *g {
                Node::Int(n) => match n {
                    0 => Node::Int(0),
                    1 => diff(f),
                    n => Node::Prod(vec![
                        Node::Int(n),
                        Node::Pow(box (f.clone(), Node::Int(n-1)))
                    ]),
                },
                ref g => {
                    // g(x) f(x) ^ {g(x) - 1} f'(x) + f(x) ^ g(x) log f(x) g'(x)
                    Node::Sum(vec![
                        Node::Prod(vec![ // g(x) f(x) ^ {g(x) - 1} f'(x)
                            g.clone(), // g(x)
                            // f(x) ^ {g(x) - 1}
                            Node::Pow(box(
                            	f.clone(),
                            	Node::Sum(vec![g.clone(), Node::Int(-1)])
        		    )),
                            diff(f) // f'(x)
                        ]),
                        Node::Prod(vec![
                            Node::Pow(box (f.clone(), g.clone())), // f(x) ^ g(x)
                            Node::Func(Func::Log, box f.clone()), // log f(x)
                            diff(g) // f'(x)
                        ])
                    ])
                }
            }
        },
        Node::Func(f, box ref g) => {
            match f {
                Func::Sin => Node::Prod(vec![
                    Node::Func(Func::Cos, box g.clone()),
                    diff(g)
                ]),
                Func::Cos => Node::Prod(vec![
                    Node::Int(-1),
                    Node::Func(Func::Sin, box g.clone()),
                    diff(g)
                ]),
                Func::Log => Node::Prod(vec![
                    Node::Pow(box(g.clone(), Node::Int(-1))),
                    diff(g)
                ]),
                Func::Exp => Node::Prod(vec![
                    Node::Func(Func::Exp, box g.clone()),
                    diff(g)
                ])
            }
        },
        Node::X => Node::Int(1)
    }
}

pub fn simplify(n: Node) -> Node {
    match n {
        Node::Prod(parts) => {
            let mut parts: Vec<Node> = parts.into_iter().map(simplify).filter(|n| n != &Node::Int(1)).collect();
            
            if parts.contains(&Node::Int(0)) {
                Node::Int(0)
            } else {
                match parts.len() {
                    0 => Node::Int(1),
                    1 => parts.pop().unwrap(),
                    _ => Node::Prod(parts)
                }
            }
        },
        Node::Sum(parts) => {
            let mut parts: Vec<Node> = parts.into_iter().map(simplify).filter(|n| n != &Node::Int(0)).collect();
            match parts.len() {
                0 => Node::Int(0),
                1 => parts.pop().unwrap(),
                _ => Node::Sum(parts)
            }
        },
        Node::Pow(box fg) => match fg {
            (Node::Int(1), _) => Node::Int(1),
            (f, Node::Int(1)) => f,
            (f, Node::Int(0)) => Node::Int(1),
            (f, g) => Node::Pow(box (f, g))
        },  
        n => n
    }
}
