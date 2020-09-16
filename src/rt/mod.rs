#[cfg(target_feature="avx")]
pub mod simd_jit;

#[cfg(target_feature="avx")]
pub mod x86_64;

#[cfg(feature="nvidia")]
pub mod ptx;

#[cfg(feature="nvidia")]
pub use cuda;
