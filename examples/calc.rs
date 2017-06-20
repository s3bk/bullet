extern crate math;
use math::lang;


fn main() {
    println!("{}", lang::parse_Expr("1 + 2 / 6").unwrap());
}
