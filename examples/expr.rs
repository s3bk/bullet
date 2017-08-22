extern crate math;

use math::lang::parse_Expr;

fn main() {
    let exprs = [
        "1 + 2", "2 x", "sin(x)", "sin(2x)", "sin(x * y)", "sin(2 x * y)", "x^y", "2 x ^ y",
        "sin(x) * log(y)"
    ];
    for expr in &exprs {
        println!("{}", expr);
        match parse_Expr(expr) {
            Ok(e) => {
                println!("{}", e.to_node().unwrap().simplify())
            },
            Err(e) => panic!("failed to parse {}", expr)
        }
    }
}
