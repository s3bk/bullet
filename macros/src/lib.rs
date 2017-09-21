#![feature(proc_macro)]

extern crate bullet_core;
extern crate proc_macro;

use proc_macro::TokenStream;
use bullet_core::vm;
use bullet_core::prelude::{NodeRc, Builder};

fn parse(t: TokenStream) -> NodeRc {
    let input = t.to_string();
    let builder = Builder::new();
    let node = builder.parse(&input).expect("failed to parse");
    node
}

#[proc_macro]
pub fn math(t: TokenStream) -> TokenStream {
    vm::syn::syn(parse(t)).parse().expect("failed to parse output")
}

#[proc_macro]
pub fn math_asm(t: TokenStream) -> TokenStream {
    vm::simd::simd_asm(&[parse(t)], &["x"]).parse().expect("failed to parse output")
}
