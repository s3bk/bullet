extern crate bullet;
extern crate env_logger;
extern crate itertools;

use std::fs;
use itertools::Itertools;


#[cfg(feature="wasm")]
fn main() {
    use bullet::builder::Builder;
    use bullet::vm::wasm::Wasm;
    use bullet::compiler::Compiler;

    env_logger::init();
    let mut args = std::env::args().skip(1);
    let output_file = args.next().expect("No output file");
    let input = args.next().expect("no input expr");
    let builder = Builder::new();
    let root = builder.parse(&input).expect("can't parse expr");
    println!("{}", root);
    let data = Wasm::compile(&root, &["x", "y"]).expect("can't compile");

    fs::write(&output_file, &data).expect("can't write output");
}

#[cfg(not(feature="wasm"))]
fn main() {

}