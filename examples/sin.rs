#[cfg(feature="jit")]
fn main() {
    use bullet::builder::Builder;
    use bullet::simd;

    let b = Builder::new();
    println!("{}", simd::simd_asm(&[b.parse("sin(x)").unwrap()], &["x"]));
    println!("{}", simd::simd_asm(&[b.parse("cos(x)").unwrap()], &["x"]));
}

#[cfg(not(feature="jit"))]
fn main() {}
