use std::collections::hash_map::{HashMap, Entry};
use node::{NodeRc, Node};
use func::Func;
use tuple::{TupleElements, Map};
use vm::Vm;

pub struct Compiler<'a, V: Vm + 'a> {
    uses: HashMap<&'a Node, usize>,
    storage: HashMap<&'a Node, V::Storage>,
    sources: HashMap<&'a str, V::Var>,
    vm: &'a mut V
}
impl<'a, V: Vm + 'a> Compiler<'a, V> {
    fn visit(&mut self, node: &'a Node) -> Vec<&'a str> {
        let mut vars = vec![];
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
                        Node::Var(ref name) => vars.push(name.as_str()),
                        _ => unimplemented!()
                    }
                },
                Entry::Occupied(mut o) => *o.get_mut() += 1
            }
        }
        vars
    }

    pub fn new(vm: &'a mut V) -> Compiler<'a, V> {
        Compiler {
            uses: HashMap::new(),
            storage: HashMap::new(),
            sources: HashMap::new(),
            vm: vm
        }
    }

    pub fn run(vm: &'a mut V, root: &'a Node) -> V::Var {
        let mut comp = Compiler::new(vm);
        let mut vars = comp.visit(root);
        vars.sort();

        for name in vars {
            let var = comp.vm.make_source(name);
            comp.sources.insert(name, var);
        }

        comp.generate(root)
    }

    /// f is called for every node
    pub fn compile<T, U, F>(vm: &mut V, nodes: T, vars: U, mut f: F)
        where T: TupleElements<Element=&'a NodeRc> + Map<V::Var>, U: TupleElements<Element=&'a str>,
              F: FnMut(&mut V, V::Var)
    {
        let mut comp = Compiler::new(vm);
        
        // walk all nodes
        for n in nodes.elements() {
            comp.visit(&**n);
        }
        
        for name in vars.into_elements() {
            let var = comp.vm.make_source(name);
	    println!("source {} @ {:?}", name, var);
            comp.sources.insert(name, var);
        }

        for (n, u) in comp.uses.iter() {
            println!("{}: {}", u, n);
        }
        // build it
        for n in nodes.into_elements() {
            let var = comp.generate(&**n);
            f(&mut comp.vm, var);
        }
    }
    
    fn generate(&mut self, node: &'a Node) -> V::Var {
        if let Some(stored) = self.storage.get(node) {
            return self.vm.load(stored); // already computed
        }
        println!("{}", node);
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
                            let prod = base.iter().map(|&(ref v, n)| {
                                let v = self.generate(v);
                                match n {
                                    0 => panic!("power of 0"),
                                    1 => v,
                                    i if i > 0 => self.vm.pow_n(v, i as u32),
                                    i => {
                                        let p = self.vm.pow_n(v, -i as u32);
                                        self.vm.inv(p)
                                    }
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
            Node::Var(ref name) => {
	        println!("use {}", name);
	        self.sources.remove(name.as_str()).expect("source was already used")
	    },
            Node::Func(f, ref g) => {
                let x = self.generate(g);
                match f {
                    Func::Sin => self.vm.sin(x),
                    Func::Cos => self.vm.cos(x),
                    _ => unimplemented!()
                }
            },
            _ => unimplemented!()

        };
        println!("{} uses for {} (stored in {:?})", self.uses[node], node, var);
        match self.uses[node] {
            0 => unreachable!(),
            1 => {},
            n => {
                self.storage.insert(node, self.vm.store(&mut var, n-1));
            }
        }
        var
    }
}
