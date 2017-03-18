#![feature(associated_consts)]
#![feature(const_fn)]
#![feature(concat_idents)]

extern crate simd;
extern crate rand;

use simd::{Simd, Basic};
use simd::x86::avx::*;
use std::ops::{Add, Mul};

#[inline(always)]
fn poly<S>(arr: &[S::Elem], x: S) -> S where 
    S: Simd + Basic + Mul<Output=S> + Add<Output=S> + Clone,
    S::Elem: Clone
{
    let mut t = S::splat(arr[0].clone());
    for a in &arr[1 ..] {
        t = S::splat(a.clone()) + x.clone() * t;
    }
    t
}

pub trait Trig<T> {
    /// valid for [-Pi, Pi]
    fn cos(self) -> Self;
}

impl<S> Trig<f32> for S where
    S: Simd<Elem=f32> + Basic + Mul<Output=S> + Add<Output=S> + Clone
{
    fn cos(self) -> S {
        poly(&[
        -9.77507131527006498114e-12,
            2.06207503915813519567e-09,
        -2.75369918573799545860e-07,
            2.48006913718665260256e-05,
        -1.38888674687691339750e-03,
            4.16666641590361985136e-02,
        -4.99999998886526927002e-01,
            9.99999999919365479957e-01
        ], self.clone() * self)
    }
}

impl<S> Trig<f64> for S where
    S: Simd<Elem=f64> + Basic + Mul<Output=S> + Add<Output=S> + Clone
{
    fn cos(self) -> S {
        poly(&[
        -9.77507131527006498114e-12,
            2.06207503915813519567e-09,
        -2.75369918573799545860e-07,
            2.48006913718665260256e-05,
        -1.38888674687691339750e-03,
            4.16666641590361985136e-02,
        -4.99999998886526927002e-01,
            9.99999999919365479957e-01
        ], self.clone() * self)
    }
}

#[macro_export]
#[macro_use]
pub mod tuple;
pub mod integrate;
pub mod real;

pub mod prelude {
    pub use integrate::*;
    pub use real::*;
    pub use tuple::*;
    pub use super::Trig;
}

#[test]
fn test_cos() {
    use std::f64::consts::PI;
    println!("{:?}", f64x4::new(0.0, -PI, PI, PI/2.0).cos());
}
