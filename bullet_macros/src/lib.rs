extern crate bullet;
extern crate proc_macro;


use proc_macro::TokenStream;
use bullet::vm;
use bullet::prelude::{NodeRc, Builder};

fn parse(t: TokenStream) -> NodeRc {
    let input = t.to_string();
    let builder = Builder::new();
    let node = builder.parse(&input).expect("failed to parse");
    node
}

#[proc_macro]
pub fn math(t: TokenStream) -> TokenStream {
    vm::syn::syn(parse(t)).to_string().parse().expect("failed to parse")
}

#[cfg(feature="simd")]
#[proc_macro]
pub fn math_asm(t: TokenStream) -> TokenStream {
    vm::simd::simd_asm(&[parse(t)], &["x"]).to_string().parse().expect("failed to parse")
}
