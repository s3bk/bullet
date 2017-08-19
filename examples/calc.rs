extern crate math;
use math::lang;
use math::diff::*;

use std::io::{stdin, BufRead};

fn main() {
    let mut stdin = stdin();
    let mut lines = stdin.lock().lines();
    while let Some(Ok(line)) = lines.next() {
        match lang::parse_Expr(line.trim()) {
            Ok(e) => {
                println!("{}", e);
                
                match e.to_node() {
                    Ok(mut f) => {
                        println!("f: {:?}", f);
                        f = f.simplify();
                        println!("f: {}", f);
                        
                        let df = diff(&f, "x").simplify();
                        println!("d/dx f(x): {:?}", df);
                        println!("d/dx f(x): {}", df);
                    },
                    Err(e) => println!("{}", e)
                }
            },
            Err(e) => println!("{:?}", e)
        }
    }
}
