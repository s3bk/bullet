extern crate bullet;

use bullet::builder::Builder;
use bullet::vm::simd;

fn main() {
    let b = Builder::new();
    let e = b.parse("sin(x)").unwrap();
    println!("{}", simd::simd_asm(&[e], &["x"]));
}
