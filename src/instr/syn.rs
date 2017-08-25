use builder::Builder;
use instr::{Compiler, Vm};
use quote::{Tokens, Ident};
use std::mem;

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

    fn make_int(&mut self, i: i64) -> Self::Var {
        quote! { #i }
    }
    fn make_const(&mut self, x: f64) -> Self::Var {
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
    fn store(&mut self, var: &mut Self::Var, _uses: usize) -> Self::Storage {
        let name = format!("storage_{}", self.stored).into();
        self.stored += 1;

        let var = mem::replace(var, self.load(&name));
        self.tokens.append(quote! { let #name = #var; });

        name
    }
    fn load(&mut self, name: &Self::Storage) -> Self::Var {
        quote! { #name }
    }
}

pub fn syn(input: String) -> Tokens {
    let builder = Builder::new();
    let node = builder.parse(&input).expect("failed to parse");

    let mut syn = Syn::new();
    let inner = Compiler::run(&mut syn, &node);
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
    
    out
}
