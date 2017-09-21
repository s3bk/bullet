#![feature(non_ascii_idents)]
#![feature(conservative_impl_trait)]
#![feature(core_intrinsics)]
#![feature(box_syntax)]
#![feature(proc_macro)]

extern crate tuple;
extern crate bullet_macros;
extern crate math_traits;

use tuple::T2;
use math::integrate::Integration;
use bullet_macros::math;
use math_traits::Real;


#[allow(non_snake_case)]
#[inline]
fn duffing(ɛ: f32, λ: f32, Ω: f32, α: f32, β: f32)
 -> impl Fn(f32, T2<f32, f32>) -> T2<f32, f32>
{
    move |t, T2(x, y)| T2(x, math!(ɛ cos(t) - λ y - α x - β x^3))
}

fn main() {
    for p in Integration::new(
        duffing(7.72, 0.2, 1.0, 0.0, 1.0), // the function to integrate
        T2(1.0, 1.0), // initial value
        0.0, // inital time
        1.0e-3, // step size,
        f32::PI // wrap point for t
    ).take(100) {
        println!("{:?}", p);
    }
}
