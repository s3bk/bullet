extern crate math;
use math::builder::Builder;

#[test]
fn text_expr() {
    let exprs = [
        "1 + 2", "2 x", "sin(x)", "sin(2x)", "sin(x * y)", "sin(2 x * y)", "x^y", "2 x ^ y",
        "sin(x) * log(y)"
    ];
    let b = Builder::new();
    for expr in &exprs {
        match b.parse(expr) {
            Ok(n) => println!("{}", n),
            Err(e) => panic!("failed to parse {}", expr)
        }
    }
}
