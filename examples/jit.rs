extern crate math;
extern crate simd;
extern crate tuple;

use math::builder::Builder;
use math::instr::avx::jit;
use simd::x86::avx::f32x8;
use tuple::T8;

fn main() {
    let b = Builder::new();
    let n = b.parse("sin(x)").unwrap();

    let c = jit(n);
    for n in 0 .. 1000i32 {
        let x = n as f32 / 100.;
        let T8(y, ..) = c.call1(f32x8::splat(x)).into();
        println!("{} {}", x, y);
    }
}
