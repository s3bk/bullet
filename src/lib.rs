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

macro_rules! todo {
    ($desc:expr) => ({return Err(Error::Todo($desc));})
}

pub mod error;
//pub mod expr;
#[allow(unused_extern_crates)]
pub mod lang { include!(concat!(env!("OUT_DIR"), "/lang.rs")); }      // the parser
pub mod diff;      // analytical differentiation
pub mod node;      // function graph
pub mod func;      // analytical functions and operators
pub mod rational;  // rational numbers
pub mod compiler;  // compiles function graph for the vm
pub mod vm;        // the virtual machine
pub mod poly;      // polynomial representation
pub mod builder;   // helps you crate function graphs
pub mod eval;      // enables to actually get "values"
pub mod integrate; // numerical integration
pub mod rt;        // runtime (various jit compilers, gpu integration)

mod consts;        // numerical constants
mod display;       // function graph representation
mod util;          // utiliy functions


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
