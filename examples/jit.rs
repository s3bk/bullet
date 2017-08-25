extern crate math;
extern crate simd;

use math::builder::Builder;
use math::instr::avx::jit;
use simd::x86::avx::f32x8;

fn main() {
    let b = Builder::new();
    let n = b.parse("2x^2 - 5").unwrap();
    let c = jit(n);
    let r = c.call1(f32x8::splat(0.5));
    println!("{:?}", r);
}
