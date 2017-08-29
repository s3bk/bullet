extern crate math;
use math::instr::avx;
use math::builder::Builder;

fn main() {
    let n = Builder::new().parse("sin(x)").unwrap();
    println!("{}", avx::asm(n));
}
