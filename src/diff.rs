use func::Func;
use node::{Node, NodeRc};
use builder::Builder;
use poly::Poly;

pub fn diff(builder: &Builder, node: &NodeRc, var: &str) -> NodeRc {
    let out = match **node {
        Node::Func(f, ref g) => {
            let dg = diff(builder, g, var);
            builder.mul(
                match f {
                    Func::Sin => // d/dx sin(g(x)) = cos(g(x)) g'(x)
                        builder.func(Func::Cos, g.clone()), 
                    Func::Cos => // d/dx cos(g(x)) = - sin(g(x)) g'(x)
                        builder.mul(builder.int(-1), builder.func(Func::Sin, g.clone())),
                    Func::Log => // d/dx log(g(x)) = g'(x) / g(x)
                        builder.pow_i(g.clone(), -1),
                    Func::Exp => // d/dx exp(g(x)) = exp(g(x)) g'(x)
                        builder.func(Func::Exp, g.clone()),
                },
                dg
            )
        },
        Node::Var(ref s) => builder.int((s == var) as i64),
        Node::Poly(ref p) => builder.poly(diff_poly(builder, p, var))
    };
    println!("d/d{} {} = {}", var, node, out);
    out
}

pub fn diff_poly(builder: &Builder, poly: &Poly, var: &str) -> Poly {
    let mut sum = Poly::int(0);
    for (base, &fac) in poly.factors() {
        let f: Vec<_> = base.iter().map(|&(ref f, n)| {
            Poly::from_node(f.clone()).pow_i(n as i32)
        }).collect(); // [f₀, f₁, f₂, ...]
        let df: Vec<_> = base.iter().map(|&(ref f, n)| {
            Poly::from_node(f.clone()).pow_i(n as i32 -1) * n * Poly::from_node(diff(builder, f, var)) // ∂ₓ f(x)ⁿ = n f(x)ⁿ⁻¹ ∂ₓ f(x)
        }).collect(); // [∂ₓ f₀, ∂ₓ f₁, ∂ₓ f₂, ...]

        for (f, df) in f.iter().zip(df.iter()) {
            println!("d/d{} {} = {}", var, f, df);
        }

        for i in 0 .. f.len() {
            let mut prod = Poly::rational(fac);
            for j in 0 .. f.len() {
                if i == j {
                    prod = prod * df[j].clone();
                } else {
                    prod = prod * f[j].clone();
                }
            }
            sum = sum + prod;
        }
    }

    sum
}
