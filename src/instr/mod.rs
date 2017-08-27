use std::collections::hash_map::{HashMap, Entry};
use node::Node;
use func::Func;

pub struct Compiler<'a, V: Vm + 'a> {
    uses: HashMap<&'a Node, usize>,
    storage: HashMap<&'a Node, V::Storage>,
    sources: HashMap<&'a str, V::Var>,
    vm: &'a mut V
}
impl<'a, V: Vm + 'a> Compiler<'a, V> {
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
        Compiler {
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
            Node::Func(f, ref g) => {
                let x = self.generate(g);
                match f {
                    Func::Sin => self.vm.sin(x),
                    _ => unimplemented!()
                }
            }
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


pub mod vm;
pub mod syn;

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

#[cfg(target_feature = "avx")]
pub mod avx;

pub use self::vm::*;
