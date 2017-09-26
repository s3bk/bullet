use super::surface::Format;

pub trait GpuPrimitive {
    fn name(&self) -> &'static str;
    fn format(&self) -> Format;
}
macro_rules! gpu_prim {
    ($($t:ty : $f:ident),*) => ($(
        impl GpuPrimitive for $t {
            fn name(&self) -> &'static str { stringify!($t) }
            fn format(&self) -> Format { $f }
        }
    )*)
}
gpu_prim!(
    f32: Float, f64: Double,
    u8: u8, i8: I8,
    u16: U16, i16: I16,
    u32: U32, i32: I32,
    u64: U32, i64: I64
);
