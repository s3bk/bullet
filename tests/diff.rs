use bullet::{builder::Builder, diff::diff, func::Transient::*};
use bullet::error::Error;
use bullet_macros::math;

#[test]
fn second_order_derivative() -> Result<(), Error>{
    let b = Builder::new();
    let x = b.var("x");
    let one = b.int(1);
    let two = b.int(2);

    let square_polynomium = b.add(b.mul(x.clone(), x.clone())?, one)?;

    let slope = diff(&b, &square_polynomium, "x")?;
    
    assert_eq!(slope, b.mul(x, two).unwrap());

    Ok(())
}

#[test]
fn derivative_chain_rule_sample_1() -> Result<(), Error>{
    let b = Builder::new();
    let x = b.var("x");
    let two = b.int(2);

    let x_square = b.mul(x.clone(), x.clone())?;
    let f = b.func(Sin.into(), x_square.clone())?;
    let df = diff(&b, &f, "x")?;

    let expected = b.mul(b.mul(x, two)?, b.func(Cos.into(), x_square.clone())?).unwrap();
    
    assert_eq!(df, expected);

    Ok(())
}

#[test]
fn derivative_chain_rule_sample_2() -> Result<(), Error>{
    let b = Builder::new();
    let x = b.var("x");
    let one = b.int(1);
    let two = b.int(2);

    let sin_x = b.func(Sin.into(), x.clone())?;
    let cos_x = b.func(Cos.into(), x.clone())?;
    let f = b.mul(sin_x.clone(), sin_x.clone())?;
    let df = diff(&b, &f, "x")?;

    let expected = b.mul(b.mul(two, cos_x.clone())?, sin_x.clone()).unwrap();
    
    assert_eq!(df, expected);

    Ok(())
}
