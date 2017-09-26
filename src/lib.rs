#![feature(const_fn)]
#![feature(concat_idents)]
#![feature(trace_macros)]
#![feature(box_syntax)]
#![feature(i128_type)]
#![feature(inclusive_range)]
#![feature(inclusive_range_syntax)]
#![feature(cfg_target_feature)]
#![feature(slice_patterns)]
#![feature(box_patterns)]
#![feature(proc_macro)]
#![feature(asm)]
#![feature(unique)]
#![feature(try_trait)]

extern crate tuple;
extern crate itertools;
extern crate lalrpop_util;
#[macro_use] extern crate quote;
extern crate math_traits;

#[cfg(feature="jit")]
extern crate memmap;
#[cfg(feature="jit")]
extern crate simd;

pub mod integrate;
pub mod rt;
pub mod vm;

macro_rules! todo {
    ($desc:expr) => ({return Err(Error::Todo($desc));})
}

pub mod error;
//pub mod expr;
#[allow(unused_extern_crates)]
pub mod lang;
pub mod diff;
pub mod node;
pub mod func;
pub mod rational;
pub mod compiler;
pub mod poly;
pub mod builder;
mod consts;
mod display;
mod util;

pub mod prelude {
    pub use error::Error;
    pub use math_traits::*;
    pub use tuple::*;
    pub use util::*;
    pub use node::*;
    pub use builder::Builder;
    pub use itertools::Itertools;
    pub use rational::Rational;
}
