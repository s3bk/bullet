extern crate math;
use math::builder::Builder;
use math::diff::diff;

fn main() {
    let exprs = [
        "1 + 2", "2 x", "sin(x)", "sin(2x)", "sin(x * y)", "sin(2 x * y)", "x^y", "2 x ^ y",
        "sin(x) * log(y)", "(x + y) * (x - y)"
    ];
    let builder = Builder::new();
    for expr in &exprs {
        println!("{}", expr);
        match builder.parse(expr) {
            Ok(n) => {
                println!(" -> {}", n);
                println!(" d/dx -> {}", diff(&builder, &n, "x"));
            },
            Err(_) => panic!("failed to parse {}", expr)
        }
    }
}
