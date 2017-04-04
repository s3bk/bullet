#![feature(non_ascii_idents)]
#![feature(conservative_impl_trait)]
#![feature(core_intrinsics)]
#![feature(box_syntax)]

extern crate tuple;
extern crate num;
extern crate math;

use tuple::T2;
use num::Num;
use math::integrate::Integration;
use math::real::Real;


#[allow(non_snake_case)]
#[inline]
fn duffing(ɛ: f32, λ: f32, Ω: f32, α: f32, β: f32)
 -> impl Fn(f32, T2<f32, f32>) -> T2<f32, f32>
{
    use std::intrinsics::{fmul_fast, cosf32};
    move |t, s| {
        unsafe {
            T2(
                s.1,
                fmul_fast(ɛ, cosf32(t))
                - fmul_fast(λ, s.1)
                - fmul_fast(s.0, α + fmul_fast(fmul_fast(s.0, s.0), β))
            )
        }
    }
}

fn main() {
    for p in Integration::new(
        duffing(7.72, 0.2, 1.0, 0.0, 1.0), // the function to integrate
        T2(1.0, 1.0), // initial value
        0.0, // inital time
        1.0e-3 // step size
    ).take(100) {
        println!("{:?}", p);
    }
}
