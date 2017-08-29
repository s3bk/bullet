extern crate math;
extern crate simd;
extern crate tuple;

use math::builder::Builder;
use math::avx::avx_jit;
use simd::x86::avx::f32x8;
use tuple::T8;

fn main() {
    let b = Builder::new();
    let f = b.parse("x+1").unwrap();
    let g = b.parse("x-1").unwrap();

    let c = avx_jit((&f, &g), ("x", ));
    for n in -10 .. 10i32 {
        let x = f32x8::splat(n as f32 / 100.);
        let (f, g) = c.call(&[x]);
        println!("{:?} {:?} {:?}", x, f, g);
    }
}
