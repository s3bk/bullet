#![feature(proc_macro)]

extern crate math;
extern crate proc_macro;

use proc_macro::TokenStream;
use math::instr;
use math::node::NodeRc;
use math::builder::Builder;

fn parse(t: TokenStream) -> NodeRc {
    let input = t.to_string();
    let builder = Builder::new();
    let node = builder.parse(&input).expect("failed to parse");
    node
}

#[proc_macro]
pub fn math(t: TokenStream) -> TokenStream {
    instr::syn::syn(parse(t)).parse().expect("failed to parse output")
}

#[proc_macro]
pub fn math_avx(t: TokenStream) -> TokenStream {
    instr::avx::asm(parse(t)).parse().expect("failed to parse output")
}
