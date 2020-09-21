use bullet::error::Error;
use bullet::{builder::Builder, diff::diff, func::Transient::*};
use bullet_macros::math;

#[test]
fn sample_0() -> Result<(), Error> {
    /*
        1 + 1 == 2
    */
    let b = Builder::new();

    let one = b.int(1);
    let two = b.int(2);

    let one_plus_one = b.add(one.clone(), one.clone())?;

    assert_eq!(one_plus_one, two);

    Ok(())
}

#[test]
fn sample_1() -> Result<(), Error> {
    /*
        1 - 1 == 0
        x - x == 0
    */
    let b = Builder::new();
    let x = b.var("x");
    let one = b.int(1);
    let zero = b.int(0);

    let one_minus_one = b.sub(one.clone(), one.clone())?;
    let x_minus_x = b.sub(x.clone(), x.clone())?;

    assert_eq!(one_minus_one, zero);
    assert_eq!(x_minus_x, zero);

    Ok(())
}

#[test]
fn sample_2() -> Result<(), Error> {
    /*
        x / x == 1
        2 / 2 == 1
    */
    let b = Builder::new();
    let x = b.var("x");
    let one = b.int(1);
    let two = b.int(2);

    let two_over_two = b.div(two.clone(), two.clone())?;
    let x_over_x = b.div(x.clone(), x.clone())?;

    assert_eq!(two_over_two, one.clone());
    assert_eq!(x_over_x, one.clone());

    Ok(())
}

#[test]
fn sample_3() -> Result<(), Error> {
    /*
        (1 + x) / x == 1 + 1 / x
    */
    let b = Builder::new();
    let x = b.var("x");
    let one = b.int(1);

    let y = b.div(b.add(one.clone(), x.clone())?, x.clone())?;
    let z = b.add(one.clone(), b.div(one.clone(), x.clone())?)?;

    assert_eq!(y, z);

    Ok(())
}

#[test]
fn sample_4() -> Result<(), Error> {
    /*
        (1 + x + x * x) / x == 1 + x + 1 / x
    */
    let b = Builder::new();

    let x = b.var("x");
    let one = b.int(1);

    let x_square = b.mul(x.clone(), x.clone())?;
    let x_inv = b.div(one.clone(), x.clone())?;

    let y = b.div(
        b.add(b.add(one.clone(), x.clone())?, x_square.clone())?,
        x.clone(),
    )?;
    let z = b.add(b.add(one.clone(), x.clone())?, x_inv.clone())?;

    assert_eq!(y, z);

    Ok(())
}

#[test]
fn sample_5() -> Result<(), Error> {
    /*
        ( x * x + 1 ) * x == x^3 + x
    */
    let b = Builder::new();

    let x = b.var("x");
    let one = b.int(1);
    let three = b.int(3);

    let x_square = b.mul(x.clone(), x.clone())?;

    let y = b.mul(b.add(one.clone(), x_square.clone())?, x.clone())?;
    let z = b.add(b.pow(x.clone(), three.clone())?, x.clone())?;

    assert_eq!(y, z);

    Ok(())
}
