extern crate math;
use math::lang;
use math::diff::*;

use std::io::{stdin, BufRead};

fn main() {
    let mut stdin = stdin();
    let mut lines = stdin.lock().lines();
    while let Some(Ok(line)) = lines.next() {
        let e = lang::parse_Expr(line.trim()).unwrap();
        println!("{}", e);
    
        let f = e.to_node();
        println!("f: {:?}", f);

        let df = diff(&f);
        println!("d/dx f(x): {:?}", df);
        println!("d/dx f(x): {:?}", simplify(df));
    }
}
