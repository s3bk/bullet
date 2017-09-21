extern crate bullet;
use bullet::builder::Builder;
use bullet::vm::glsl::glsl;

fn main() {
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
