use prelude::*;
use poly::Poly;
use func::*;
use func::Transient::*;

pub fn diff(builder: &Builder, node: &NodeRc, var: &str) -> Result<NodeRc, Error> {
    match **node {
        Node::Apply(ref f, ref g) => {
            match **f {
                Node::Op(Func::Transient(f)) => {
                    let dg = diff(builder, g, var)?;
                    builder.mul(
                        match f {
                            Sin => // d/dx sin(g(x)) = cos(g(x)) g'(x)
                                builder.func(Cos.into(), g.clone())?, 
                            Cos => // d/dx cos(g(x)) = - sin(g(x)) g'(x)
                                builder.neg(builder.func(Sin.into(), g.clone())?)?,
                            Log => // d/dx log(g(x)) = g'(x) / g(x)
                                builder.pow_i(g.clone(), -1)?,
                            Exp => // d/dx exp(g(x)) = exp(g(x)) g'(x)
                                builder.func(Exp.into(), g.clone())?,
                        },
                        dg       
                    )
                },
                _ => todo!("diff non-ops")
            }
        },
        Node::Var(ref s) => Ok(builder.int(s == var)),
        Node::Poly(ref p) => Ok(builder.poly(diff_poly(builder, p, var)?)),
        Node::Tuple(ref parts) => builder.tuple(parts.iter().map(|p| diff(builder, p, var))),
        _ => todo!("diff ???")
    }
}

pub fn diff_poly(builder: &Builder, poly: &Poly, var: &str) -> Result<Poly, Error> {
    let mut sum = Poly::int(0);
    for (base, fac) in poly.factors() {
        let f: Result<Vec<Poly>, Error> = base.iter().map(|&(ref f, ref n)| {
            Poly::from_node(f.clone()).pow(builder, n.clone())
        }).collect(); // [f₀, f₁, f₂, ...]
        let df: Result<Vec<Poly>, Error> = base.iter().map(|&(ref f, ref n)| Ok(
            Poly::from_node(f.clone()).pow(builder, n.clone() - 1)?
            * n.clone() * Poly::from_node(diff(builder, f, var)?) // ∂ₓ f(x)ⁿ = n f(x)ⁿ⁻¹ ∂ₓ f(x)
        )).collect(); // [∂ₓ f₀, ∂ₓ f₁, ∂ₓ f₂, ...]

        let (f, df) = (f?, df?);
        for (f, df) in f.iter().zip(df.iter()) {
            println!("d/d{} {} = {}", var, f, df);
        }

        for i in 0 .. f.len() {
            let mut prod = Poly::rational(fac.clone());
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
