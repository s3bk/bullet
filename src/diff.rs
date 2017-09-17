use func::Func;
use node::{Node, NodeRc};
use builder::{Builder, Error};
use poly::{Poly, PolyError};

type DiffError = Error<'static>;

pub fn diff(builder: &Builder, node: &NodeRc, var: &str) -> Result<NodeRc, DiffError> {
    let out = match **node {
        Node::Func(f, ref g) => {
            let dg = diff(builder, g, var)?;
            builder.mul(
                match f {
                    Func::Sin => // d/dx sin(g(x)) = cos(g(x)) g'(x)
                        builder.func(Func::Cos, g.clone())?, 
                    Func::Cos => // d/dx cos(g(x)) = - sin(g(x)) g'(x)
                        builder.mul(builder.int(-1), builder.func(Func::Sin, g.clone())?)?,
                    Func::Log => // d/dx log(g(x)) = g'(x) / g(x)
                        builder.pow_i(g.clone(), -1)?,
                    Func::Exp => // d/dx exp(g(x)) = exp(g(x)) g'(x)
                        builder.func(Func::Exp, g.clone())?,
                },
                dg
            )?
        },
        Node::Var(ref s) => builder.int((s == var) as i64),
        Node::Poly(ref p) => builder.poly(diff_poly(builder, p, var)?),
        Node::Tuple(ref parts) => {
            let parts: Result<Vec<_>, _> = parts.iter().map(|p| diff(builder, p, var)).collect();
            builder.tuple(parts?)
        }
    };

    Ok(out)
}

pub fn diff_poly(builder: &Builder, poly: &Poly, var: &str) -> Result<Poly, DiffError> {
    let mut sum = Poly::int(0);
    for (base, &fac) in poly.factors() {
        let f: Result<Vec<Poly>, PolyError> = base.iter().map(|&(ref f, n)| {
            Poly::from_node(f.clone()).pow_i(builder, n as i32)
        }).collect(); // [f₀, f₁, f₂, ...]
        let df: Result<Vec<Poly>, DiffError> = base.iter().map(|&(ref f, n)| Ok(
            Poly::from_node(f.clone()).pow_i(builder, n as i32 -1)?
            * n * Poly::from_node(diff(builder, f, var)?) // ∂ₓ f(x)ⁿ = n f(x)ⁿ⁻¹ ∂ₓ f(x)
        )).collect(); // [∂ₓ f₀, ∂ₓ f₁, ∂ₓ f₂, ...]

        let (f, df) = (f?, df?);
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

    Ok(sum)
}
