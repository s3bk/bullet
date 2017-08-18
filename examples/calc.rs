extern crate math;
use math::lang;
use math::diff::*;

use std::io::{stdin, BufRead};

fn main() {
    let mut stdin = stdin();
    let mut lines = stdin.lock().lines();
    while let Some(Ok(line)) = lines.next() {
        if let Ok(e) = lang::parse_Expr(line.trim()) {
            println!("{}", e);
            
            if let Ok(mut f) = e.to_node() {
                println!("f: {}", f);
                f = simplify(f);
                println!("f: {}", f);
                
                let df = diff(&f);
                println!("d/dx f(x): {}", df);
                println!("d/dx f(x): {}", simplify(df));
            }
        }
    }
}
