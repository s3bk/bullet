use lang::parse_Expr;
use instr::{Assembler, Vm};
use quote::{Tokens, Ident};
use proc_macro::TokenStream;

struct Syn {
    tokens: Tokens,
    stored: usize
}
impl Syn {
    pub fn new() -> Syn {
        Syn {
            tokens: Tokens::new(),
            stored: 0
        }
    }
}

impl Vm for Syn {
    type Var = Tokens;
    type Storage = Ident;
    
    fn make_const(&mut self, x: f32) -> Self::Var {
        quote! { #x }
    }
    fn make_source(&mut self, name: &str) -> Self::Var {
        let name = Ident::from(name);
        quote! { #name }
    }
    fn make_sum(&mut self, parts: Vec<Self::Var>) -> Self::Var {
        let first = &parts[0];
        let others = &parts[1..];
        quote! { #first #(.add(#others) )* }
    }
    fn make_product(&mut self, parts: Vec<Self::Var>) -> Self::Var {
        let first = &parts[0];
        let others = &parts[1..];
        quote! { #first #( .mul( #others ) )* }
    }
    fn store(&mut self, var: Self::Var) -> Self::Storage {
        let name = format!("storage_{}", self.stored).into();
        self.stored += 1;
        self.tokens.append(quote! { let #name = #var; });
        name
    }
    fn load(&mut self, name: &Self::Storage) -> Self::Var {
        quote! { #name }
    }
    fn forget(&mut self, _storage: Self::Storage) {}
}

pub fn math_syn(input: TokenStream) -> TokenStream {
    let node = parse_Expr(&input.to_string()).expect("failed to parse")
        .to_node().expect("can't convert to node");

    let mut syn = Syn::new();
    let inner = Assembler::run(&mut syn, &node);
    let store = syn.tokens;
    let out = quote! {
        {
            use std::ops::{Add, Mul};
            #store
            #inner
        }
    };
    use std::fs::File;
    use std::io::Write;
    writeln!(File::create("/tmp/out").unwrap(), "{}", out).unwrap();
    
    out.parse().expect("failed to parse output")
}
