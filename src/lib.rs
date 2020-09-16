#![feature(const_fn)]
#![feature(concat_idents)]
#![feature(trace_macros)]
#![feature(box_syntax)]
#![feature(slice_patterns)]
#![feature(box_patterns)]
#![feature(asm)]
#![feature(try_trait)]
#![feature(proc_macro_hygiene)]
#![cfg_attr(feature="jit", feature(stdsimd))]
#![cfg_attr(feature="nvidia", feature(ptr_internals))]

extern crate tuple;
extern crate itertools;
extern crate lalrpop_util;
extern crate math_traits;
extern crate num_bigint;
extern crate num_traits;
extern crate num_rational;

#[cfg(feature="wasm")]
extern crate parity_wasm;

#[macro_use] extern crate log;

#[cfg(feature="codegen")]
#[macro_use] extern crate quote;

#[cfg(feature="codegen")]
extern crate proc_macro2;

#[cfg(feature="jit")]
extern crate memmap;

#[cfg(feature="jit")]
extern crate packed_simd;

macro_rules! todo {
    ($desc:expr) => ({return Err(Error::Todo($desc));})
}
macro_rules! bug {
    ($desc:expr) => ({return Err(Error::Bug($desc));})
}

pub mod error;
//pub mod expr;
#[allow(warnings)]
pub mod lang { include!(concat!(env!("OUT_DIR"), "/lang.rs")); }      
pub mod diff;      // analytical differentiation
pub mod node;      // function graph
pub mod func;      // analytical functions and operators
pub mod compiler;  // compiles function graph for the vm
pub mod vm;        // the virtual machine
pub mod poly;      // polynomial representation
pub mod builder;   // helps you crate function graphs
pub mod eval;      // enables to actually get "values"
pub mod integrate; // numerical integration
pub mod numbers;
#[cfg(feature="jit")]
pub mod rt;        // runtime (various jit compilers, gpu integration)
pub mod data;

mod consts;        // numerical constants
pub mod display;       // function graph representation
mod util;          // utiliy functions


pub mod prelude {
    pub use error::Error;
    pub use math_traits::*;
    pub use tuple::*;
    pub use util::*;
    pub use node::*;
    pub use builder::Builder;
    pub use itertools::Itertools;
    pub use numbers::*;
}
