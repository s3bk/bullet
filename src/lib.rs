#![feature(associated_consts)]
#![feature(const_fn)]
#![feature(concat_idents)]
#![feature(trace_macros)]
#![feature(box_syntax)]
#![feature(try_from)]
#![feature(i128_type)]
#![feature(inclusive_range)]
#![feature(inclusive_range_syntax)]
#![feature(cfg_target_feature)]

extern crate simd;
extern crate rand;
extern crate tuple;
extern crate optimization;

pub mod integrate;
pub mod real;
pub mod expr;
pub mod lang;
pub mod cast;

pub mod prelude {
    pub use integrate::*;
    pub use real::*;
    pub use tuple::*;
    pub use cast::*;
}
