#![feature(proc_macro, const_fn)]

extern crate math_macros;
use math_macros::math;

fn main() {
    let x = 5.0;
    let y = 2.0;
    let z = math!(2 * x + 3 * (y + 2 * x));
    println!("{}", z);
}
