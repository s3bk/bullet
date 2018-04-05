use prelude::*;
use compiler::Compiler;
use vm::{Vm, Round};
use quote::{Tokens};
use std::mem;
use std::iter::once;
use proc_macro2::{Term, Span};

struct Syn {
    tokens: Tokens,
    stored: usize,
    inputs: Vec<Term>
}
impl Syn {
    pub fn new() -> Syn {
        Syn {
            tokens: Tokens::new(),
            stored: 0,
            inputs: Vec::new()
        }
    }
}

impl Vm for Syn {
    type Var = Tokens;
    type Storage = Term;

    fn make_int(&mut self, i: i64) -> Self::Var {
        let i: i16 = i.cast().unwrap();
        quote! { <T as Real>::int(#i) }
    }
    fn make_const(&mut self, x: f64) -> Self::Var {
        quote! { <T as Real>::float(#x) }
    }
    fn make_source(&mut self, name: &str) -> Self::Var {
        let var = Term::new(name, Span::call_site());
        self.inputs.push(var.clone());
        quote! { #var }
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
        // `var` contains an expression.
        // This expression needs to be assigned to a variable, so it can be used again later.

        // make a new variable name
        let name = Term::new(&format!("storage_{}", self.stored), Span::call_site());
        self.stored += 1;

        // replace `var` by the variable name (load does that) and get ownership of the original expression
        let var = mem::replace(var, self.load(&name));

        // now actually assign the expression to the variable (this is fine, as var can't be used until this function returns)
        self.tokens.append_all(once(quote! { let #name = #var; }));

        name
    }
    fn load(&mut self, name: &Self::Storage) -> Self::Var {
        quote! { #name }
    }
    fn round(&mut self, x: Self::Var, mode: Round) -> Self::Var {
        match mode {
            Round::Up => quote! { T::ceil(#x) },
            Round::Down => quote! { T::floor(#x) }
        }
    }
    fn div(&mut self, a: Self::Var, b: Self::Var) -> Self::Var {
        quote! { #a / #b }
    }
    fn inv(&mut self, a: Self::Var) -> Self::Var {
        quote! { <T as Real>::inv(#a) }
    }
    fn step_at(&mut self, at: Self::Var, x: Self::Var) -> Self::Var {
        quote! {
            #x.ge(#at).select(f32x8::splat(1.0), f32::splat(0.0))
        }
    }
    /*
    fn sin(&mut self, x: Self::Var) -> Self::Var {
        quote! { #x.sin() }
    }
    fn cos(&mut self, x: Self::Var) -> Self::Var {
        quote! { #x.cos() }
    }
    */
}

pub fn syn(node: NodeRc) -> Tokens {
    let mut syn = Syn::new();
    let inner = Compiler::run(&mut syn, &node).unwrap();
    let store = syn.tokens;
    let args = &syn.inputs;

    let out = quote! {
        #[allow(unused_imports)]
        {
            extern crate math_traits;
            use math_traits::Real;
            use std::ops::*;
            
            fn f<T: Real>(#(#args: T),*) -> T {    
                #store
                #inner
            }
            f(#(#args),*)
        }
    };
    {
        use std::fs::OpenOptions;
        use std::io::Write;
        writeln!(
            OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open("/tmp/out.rs")
                .unwrap(),
            "{}", out
        ).unwrap();
    }
    out
}
