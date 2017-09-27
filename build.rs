extern crate lalrpop;
use lalrpop::*;

fn main() {
    Configuration::new()
        .use_cargo_dir_conventions()
        .process()
        .unwrap();
}
