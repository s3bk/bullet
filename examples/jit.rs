extern crate bullet;
extern crate packed_simd as simd;

use bullet::rt::jit;
use bullet::prelude::Builder;
use simd::f32x8;

fn main() {
    let b = Builder::new();
    let f = b.parse("x+1").unwrap();
    let g = b.parse("x-1").unwrap();

    let c = jit(&[f, g], &["x"]).unwrap();
    for n in -10 .. 10i32 {
        let x = f32x8::splat(n as f32 / 100.);
        let mut out = [f32x8::splat(0.0); 2];
        c.call(&[x], &mut out);
        println!("{:?} {:?} {:?}", x, out[0], out[1]);
    }
}
