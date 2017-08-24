use std::collections::hash_map::{HashMap, Entry};
use node::Node;

pub struct Assembler<'a, V: Vm + 'a> {
    uses: HashMap<&'a Node, usize>,
    storage: HashMap<&'a Node, V::Storage>,
    sources: HashMap<&'a str, V::Var>,
    vm: &'a mut V
}
impl<'a, V: Vm + 'a> Assembler<'a, V> {
    fn visit(&mut self, node: &'a Node) -> Vec<&'a str> {
        let mut sources = vec![];
        let mut queue = vec![node];
        while let Some(node) = queue.pop() {
            match self.uses.entry(node) {
                Entry::Vacant(v) => {
                    v.insert(1);
                    match *node {
                        Node::Poly(ref p) => {
                            for (base, _) in p.factors() {
                                queue.extend(base.iter().map(|b| &*b.0));
                            }
                        },
                        Node::Func(_, ref g) => queue.push(g),
                        Node::Var(ref name) => sources.push(name.as_str()),
                    }
                },
                Entry::Occupied(mut o) => *o.get_mut() += 1
            }
        }
        sources
    }

    pub fn run(vm: &'a mut V, root: &'a Node) -> V::Var {
        Assembler {
            uses: HashMap::new(),
            storage: HashMap::new(),
            sources: HashMap::new(),
            vm: vm
        }.assemble(root)
    }
    
    fn assemble(mut self, root: &'a Node) -> V::Var {
        // walk all nodes
        let mut sources = self.visit(root);
        sources.sort();

        for name in sources {
            let var = self.vm.make_source(name);
            self.sources.insert(name, var);
        }

        // build it
        self.generate(root)

    }
        
    fn generate(&mut self, node: &'a Node) -> V::Var {
        if let Some(stored) = self.storage.get(node) {
            return self.vm.load(stored); // already computed
        } 
        let mut var = match *node {
            Node::Poly(ref poly) => {
                if let Some(i) = poly.as_int() {
                    return self.vm.make_int(i);
                }
                let mut sum: Vec<_> = poly.factors().map(|(base, &fac)| {
                    // multible cases here..
                    let fac = match fac.as_int() {
                        Some(1) => None,
                        Some(i) => Some(self.vm.make_int(i)),
                        None => Some(self.vm.make_const(fac.to_f64()))
                    };

                    let base = match base.len() {
                        0 => None,
                        _ => {
                            let prod = base.iter().map(|vn| {
                                let v = self.generate(&*vn.0);
                                match vn.1 {
                                    0 => panic!("power of 0"),
                                    1 => v,
                                    i if i > 0 => self.vm.pow_n(v, i as u32),
                                    _ => unimplemented!("negative powers are not implemented yet")
                                }
                            }).collect();
                            Some(self.vm.make_product(prod))
                        }
                    };   
                    match (fac, base) {
                        (None, None) => self.vm.make_int(1),
                        (Some(f), None) => f,
                        (None, Some(b)) => b,
                        (Some(f), Some(b)) => self.vm.mul(f, b)
                    }
                }).collect();

                match sum.len() {
                    0 => self.vm.make_int(0),
                    1 => sum.pop().unwrap(),
                    _ => self.vm.make_sum(sum)
                }
            },
            Node::Var(ref name) => self.sources.remove(name.as_str()).expect("source was already used"),
            _ => unimplemented!()
        };
        match self.uses[node] {
            0 => unreachable!(),
            1 => {},
            n => {
                self.storage.insert(node, self.vm.store(&mut var, n));
            }
        }
        var
    }
}

pub trait Vm {
    type Var;
    type Storage;
    
    fn make_const(&mut self, f64) -> Self::Var;
    fn make_source(&mut self, name: &str) -> Self::Var;
    fn make_sum(&mut self, parts: Vec<Self::Var>) -> Self::Var;
    fn make_product(&mut self, parts: Vec<Self::Var>) -> Self::Var;
    fn store(&mut self, var: &mut Self::Var, uses: usize) -> Self::Storage;
    fn load(&mut self, storage: &Self::Storage) -> Self::Var;
    fn copy(&mut self, var: &mut Self::Var) -> Self::Var {
        let s = self.store(var, 1);
        self.load(&s)
    }
    fn mul(&mut self, a: Self::Var, b: Self::Var) -> Self::Var {
        self.make_product(vec![a, b])
    }
    fn add(&mut self, a: Self::Var, b: Self::Var) -> Self::Var {
        self.make_sum(vec![a, b])
    }

    fn make_int(&mut self, i: i64) -> Self::Var {
        self.make_const(i as f64)
    }
    fn pow_n(&mut self, mut x: Self::Var, mut n: u32) -> Self::Var {
        assert!(n > 0, "attempted to calculate x^0: this is a bug in the optimizer");

        // handle trailing powers (replace x by x²ⁿ)
        for _ in 0 .. n.trailing_zeros() {
            let x2 = self.copy(&mut x);
            x = self.mul(x, x2);
            n /= 2;
        }

        // for powers of two, the computation is complete
        if n == 1 {
            return x;
        }
        
        let mut y = self.copy(&mut x); // holds the power so far
        while n > 1 {
            if n & 1 == 1 {
                let x2 = self.copy(&mut x);
                y = self.mul(y, x2);
            }

            let x2 = self.copy(&mut x);
            x = self.mul(x, x2);
            n /= 2;
        }

        assert_eq!(n, 1);
        self.mul(x, y) // final multiplication
    }

}

mod avx_asm;
pub use self::avx_asm::math_avx;

mod syn;
pub use self::syn::math_syn;
