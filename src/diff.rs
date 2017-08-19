use func::Func;
use node::Node;
use simplify::{sum, simplify_prod, simplify_sum, product, power, function};

pub fn diff(node: &Node, var: &str) -> Node {
    let out = match *node {
        Node::Int(_) => Node::Int(0),
        Node::Sum(ref parts) => simplify_sum(parts.iter().map(|n| diff(n, var))),
        Node::Prod(ref parts) => simplify_sum(
            (0 .. parts.len()).map(|i| {
                simplify_prod(
                    parts.iter().enumerate().map(|(j, f)| {
                        if i == j {
                            diff(f, var)
                        } else {
                            f.clone()
                        }
                    })
                )
            })
        ),
        Node::Pow(box (ref f, ref g)) => {
            // f(x)^g(x) ( log f(x) · g'(x) + g(x) f'(x) f(x)^-1 )
            product((
                power( // f(x)^g(x)
                    f.clone(),
                    g.clone()
        	),
                sum(( // log f(x) · g'(x) + g(x) f'(x) f(x)^-1
                    product(( // log f(x) · g'(x)
                        function(Func::Log, f.clone()), // log f(x)
                        diff(g, var)
                    )),
                    product(( // g(x) f'(x) f(x)^-1
                        g.clone(),
                        diff(f, var),
                        power(f.clone(), Node::Int(-1)) // f(x)^-1
                    ))
                ))
            ))
        },
        Node::Func(f, box ref g) => {
            match f {
                Func::Sin => product((
                    function(Func::Cos, g.clone()), // d/dx sin(g(x)) = cos(g(x)) g'(x)
                    diff(g, var)
                )),
                Func::Cos => product(( // d/dx cos(g(x)) = - sin(g(x)) g'(x)
                    Node::Int(-1),
                    function(Func::Sin, g.clone()),
                    diff(g, var)
                )),
                Func::Log => product(( // d/dx log(g(x)) = g'(x) / g(x)
                    power(g.clone(), Node::Int(-1)),
                    diff(g, var)
                )),
                Func::Exp => product(( // d/dx exp(g(x)) = exp(g(x)) g'(x)
                    function(Func::Exp, g.clone()),
                    diff(g, var)
                ))
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
