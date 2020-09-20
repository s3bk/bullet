use bullet::eval::EvalContext;
use itertools::Itertools;

fn main() {
    env_logger::init();
    let mut ctx = EvalContext::new();
    let input: String = std::env::args().skip(1).format(" ").to_string();
    
    match ctx.run(&input) {
        Ok(Some(s)) => println!("{}", s),
        Ok(None) => {},
        Err(e) => println!("{}", e),
    }
}
