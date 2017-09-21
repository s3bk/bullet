#![feature(asm, cfg_target_feature)]
#![feature(proc_macro)]

extern crate bullet_core;
extern crate bullet_macros;

#[cfg(feature="nvidia")]
extern crate cuda;

#[cfg(feature="jit")]
extern crate memmap;
#[cfg(feature="jit")]
extern crate simd;

pub mod integrate;
pub mod rt;

pub use bullet_core::*;
pub use bullet_macros::*;
