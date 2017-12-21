extern crate bullet;

use bullet::builder::Builder;
use bullet::vm::simd;

fn main() {
    let b = Builder::new();
    println!("{}", simd::simd_asm(&[b.parse("sin(x)").unwrap()], &["x"]));
    println!("{}", simd::simd_asm(&[b.parse("cos(x)").unwrap()], &["x"]));
}
