#![feature(proc_macro, asm, repr_simd, const_fn)]
extern crate math_macros;
extern crate simd;
use math_macros::math_avx;
use simd::x86::avx::f32x8;
use std::mem::transmute;

fn main() {
    let x = f32x8::splat(5.0);
    let y = f32x8::splat(2.0);
    println!("{:?}", foo(x, y));
    println!("{:?}", bar(x, y));
    println!("{:?}", baz(x));
}

#[inline(never)]
#[no_mangle]
pub fn foo(x: f32x8, y: f32x8) -> f32x8 {
    math_avx!(2 * x + 3 * (y + 2 * x))
    // 2 x + 3 y + 6x = 8x + 3y
    // 2x
    // (2x+y) * c + 2x
}
#[inline(never)]
#[no_mangle]
pub fn bar(x: f32x8, y: f32x8) -> f32x8 {
    f32x8::splat(2.0) * x + f32x8::splat(3.0) * (y + f32x8::splat(2.0) * x)
}

#[inline(never)]
fn baz(x: f32x8) -> f32x8 {
    let data = [0xc5, 0xfc, 0x59, 0xc0, 0x3c];
    unsafe {
        let f: fn(f32x8) -> f32x8 = transmute(&data);
        f(x)
    }
}
