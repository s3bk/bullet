extern crate math;
use math::builder::Builder;

#[test]
fn expr() {
    let exprs = [
        "1 + 2", "2 x", "sin(x)", "sin(2x)", "sin(x * y)", "sin(2 x * y)", "x^y", "2 x ^ y",
        "sin(x) * log(y)",
        "a^(2+b)", "2x", "a*2b",
        "sin(x) + sin(x)^2", "a^(2+b)-b*6c",
        "(a, b, 3)",
        "(a + 1, b - 1) + (a - 1, b + 1)"
    ];
    let b = Builder::new();
    for expr in &exprs {
        match b.parse(expr) {
            Ok(n) => println!("{} -> {}", expr, n),
            Err(e) => panic!("failed to parse {}: {}", expr, e)
        }
    }
}
