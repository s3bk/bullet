extern crate bullet;
use bullet::builder::Builder;

#[cfg(feature="glsl")]
fn main() {
    use bullet::vm::glsl::glsl;

    let expr = "(x+y)/2";
    println!("Expr: {}\n", expr);
    let builder = Builder::new();
    match builder.parse(expr) {
        Ok(tokens) => {
            let (vert, frag) = glsl(tokens);
            println!("Vert:\n{}\n\nFrag:\n{}\n", vert, frag);
        },
        Err(e) => eprintln!("error: {:?}", e),
    }
}


#[cfg(not(feature="glsl"))]
fn main() {}
