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
                                queue.extend(base.iter().map(|&(v, _)| &*v));
                            }
                        },
                        Node::Func(_, ref g) => queue.push(g),
                        Node::Var(ref name) => sources.push(name.as_str()),
                        _ => {}
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
        let var = match *node {
            Node::Poly(ref p) => {
                if let Some(i) = p.as_int() {
                    self.vm.make_const(i as f32)
                } else {
                    unimplemented!()
                }
            },
            Node::Var(ref name) => self.sources.remove(name.as_str()).expect("source was already used"),
            _ => unimplemented!()
        };
        match self.uses[node] {
            0 => unreachable!(),
            1 => var,
            n => {
                let (stored, var) = self.vm.store(var, n);
                self.storage.insert(node, stored);
                var
            }
        }
    }
}

pub trait Vm {
    type Var;
    type Storage;
    
    fn make_const(&mut self, f32) -> Self::Var;
    fn make_source(&mut self, name: &str) -> Self::Var;
    fn make_sum(&mut self, parts: Vec<Self::Var>) -> Self::Var;
    fn make_product(&mut self, parts: Vec<Self::Var>) -> Self::Var;
    fn store(&mut self, var: Self::Var, uses: usize) -> (Self::Storage, Self::Var);
    fn load(&mut self, storage: &Self::Storage) -> Self::Var;
}

mod avx_asm;
pub use self::avx_asm::math_avx;

mod syn;
pub use self::syn::math_syn;
