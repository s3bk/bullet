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
    for (base, &factor) in poly.factors() {
        // ∂ₓ ∏ᵢ fᵢ(x)ⁿ = ∏ᵢ n fᵢ'(x) / ∏ᵢ fᵢ(x)
        let mut df_prod = Poly::rational(factor);

        for &(ref f, n) in base.iter() {
            df_prod = df_prod * Poly::from_node(diff(builder, f, var)) * n;
            
            if df_prod.is_zero() {
                break;
            }
        }
        sum = sum + df_prod;
    }
    sum * poly.clone().pow_i(-1)
}
