#![feature(concat_idents)]
#![feature(trace_macros)]
#![feature(box_patterns)]
#![feature(proc_macro_hygiene)]
#![cfg_attr(feature="jit", feature(portable_simd))]
#![cfg_attr(feature="nvidia", feature(ptr_internals))]

#[macro_use] extern crate log;

#[cfg(feature="codegen")]
#[macro_use] extern crate quote;

#[cfg(feature="codegen")]
extern crate proc_macro2;

#[cfg(feature="jit")]
extern crate memmap;

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
    pub use crate::error::Error;
    pub use math_traits::*;
    pub use tuple::*;
    pub use crate::util::*;
    pub use crate::node::*;
    pub use crate::builder::Builder;
    pub use itertools::Itertools;
    pub use crate::numbers::*;
}
