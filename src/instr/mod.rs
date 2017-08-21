use std::collections::hash_map::{HashMap, Entry};
use node::Node;

pub struct Assembler<'a, V: Vm + 'a> {
    uses: HashMap<&'a Node, usize>,
    storage: HashMap<&'a Node, V::Storage>,
    vm: &'a mut V
}
impl<'a, V: Vm + 'a> Assembler<'a, V> {
    fn visit(&mut self, node: &'a Node) {
        let mut queue = vec![node];
        while let Some(node) = queue.pop() {
            match self.uses.entry(node) {
                Entry::Vacant(v) => {
                    v.insert(1);
                    match *node {
                        Node::Sum(ref parts) | Node::Prod(ref parts) => queue.extend(parts.iter()),
                        Node::Pow(box (ref f, ref g)) => {
                            queue.push(f);
                            queue.push(g);
                        },
                        Node::Func(_, box ref g) => queue.push(g),
                        _ => {}
                    }
                },
                Entry::Occupied(mut o) => *o.get_mut() += 1
            }
        }
    }

    pub fn run(vm: &'a mut V, root: &'a Node) -> V::Var {
        let mut assembler = Assembler {
            uses: HashMap::new(),
            storage: HashMap::new(),
            vm: vm
        };
        // walk all nodes
        assembler.visit(root);

        // build it
        assembler.generate(root)
    }

    fn generate(&mut self, node: &'a Node) -> V::Var {
        if let Some(stored) = self.storage.get(node) {
            return self.vm.load(stored); // already computed
        } 
        let var = match *node {
            Node::Int(i) => self.vm.make_const(i as f32),
            Node::Prod(ref parts) => {
                let parts = parts.iter().map(|n| self.generate(n)).collect();
                self.vm.make_product(parts)
            },
            Node::Sum(ref parts) => {
                let parts = parts.iter().map(|n| self.generate(n)).collect();
                self.vm.make_sum(parts)
            },
            Node::Var(ref name) => self.vm.make_source(name),
            _ => unimplemented!()
        };
        if self.uses[node] > 1 {
            let (stored, var) = self.vm.store2(var);
            self.storage.insert(node, stored);
            var
        } else {
            var
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
    fn store(&mut self, var: Self::Var) -> Self::Storage;
    fn store2(&mut self, var: Self::Var) -> (Self::Storage, Self::Var) {
        let stored = self.store(var);
        let var = self.load(&stored);
        (stored, var)
    }
    fn load(&mut self, storage: &Self::Storage) -> Self::Var;
    fn forget(&mut self, storage: Self::Storage);
}

mod avx_asm;
pub use self::avx_asm::math_avx;

mod syn;
pub use self::syn::math_syn;
