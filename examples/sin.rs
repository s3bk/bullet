#[cfg(feature="jit")]
fn main() {
    use bullet::builder::Builder;
    use bullet::simd;

    let b = Builder::new();

    println!("{}", simd::simd_asm(&[b.parse("sin(x)").unwrap()], &["x"]));
    println!("{}", simd::simd_asm(&[b.parse("cos(x)").unwrap()], &["x"]));
}

#[cfg(not(feature="jit"))]
fn main() {
    use bullet::{builder::Builder, compiler::Compiler, vm::syn::Syn};

    let b = Builder::new();
    let node = b.parse("sin(x)").unwrap();
    let tokens = syn(node);
    println!("{tokens}");
}
