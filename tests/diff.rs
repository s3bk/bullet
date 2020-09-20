use bullet::{builder::Builder, diff::diff};
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
