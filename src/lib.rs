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

extern crate syn;
extern crate simd;
extern crate rand;
extern crate tuple;
extern crate optimization;
extern crate fmath;
extern crate typenum;
extern crate itertools;
extern crate petgraph;
extern crate lalrpop_util;

#[macro_use] extern crate quote;

pub mod integrate;
pub mod real;
//pub mod expr;
pub mod lang;
pub mod cast;
pub mod diff;
pub mod node;
pub mod func;
pub mod rational;
pub mod eval;
pub mod instr;
pub mod poly;
pub mod builder;

pub mod prelude {
    pub use integrate::*;
    pub use real::*;
    pub use tuple::*;
    pub use cast::*;
}
