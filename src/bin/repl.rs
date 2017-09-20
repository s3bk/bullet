extern crate math;

use std::io::{stdin, BufRead};
use math::eval::EvalContext;

fn main() {
    let mut ctx = EvalContext::new();
    let stdin = stdin();
    let mut lines = stdin.lock().lines();
    while let Some(Ok(line)) = lines.next() {
        match ctx.run(&line) {
            Ok(Some(s)) => println!("{}", s),
            Ok(None) => {},
            Err(e) => println!("{}", e),
        }
    }
}
