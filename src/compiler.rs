use prelude::*;
use std::collections::hash_map::{HashMap, Entry};
use node::{NodeRc, Node};
use func::{Func, Transient};
use vm::Vm;

pub struct Compiler<'a, V: Vm + 'a> {
    uses: HashMap<&'a Node, usize>,
    storage: HashMap<&'a Node, V::Storage>,
    sources: HashMap<&'a str, V::Var>,
    vm: &'a mut V
}
impl<'a, V: Vm + 'a> Compiler<'a, V> {
    fn visit(&mut self, node: &'a Node) -> Result<Vec<&'a str>, Error> {
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
                        Node::Apply(ref f, ref g) => match **f {
                            Node::Op(Func::Transient(_)) => queue.push(g),
                            _ => bug!("only transients are allowed as left argument of apply"),
                        }
                        Node::Var(ref name) => vars.push(name.as_str()),
                        Node::Tuple(ref parts) => queue.extend(parts.iter().map(|n| &**n)),
                        Node::Op(_) => bug!("no transients allowd outside of apply")
                    }
                },
                Entry::Occupied(mut o) => *o.get_mut() += 1
            }
        }
        Ok(vars)
    }

    pub fn new(vm: &'a mut V) -> Compiler<'a, V> {
        Compiler {
            uses: HashMap::new(),
            storage: HashMap::new(),
            sources: HashMap::new(),
            vm: vm
        }
    }

    pub fn run(vm: &'a mut V, root: &'a Node) -> Result<V::Var, Error> {
        let mut comp = Compiler::new(vm);
        let mut vars = comp.visit(root)?;
        vars.sort();

        for name in vars {
            let var = comp.vm.make_source(name);
            comp.sources.insert(name, var);
        }

        comp.generate(root)
    }

    /// f is called for every node
    pub fn compile(vm: &mut V, nodes: &[NodeRc], vars: &[&str]) -> Result<Vec<V::Var>, Error>
    {
        let mut comp = Compiler::new(vm);
        
        // walk all nodes
        for n in nodes.iter() {
            comp.visit(&**n)?;
        }
        
        for &name in vars.iter() {
            let var = comp.vm.make_source(name);
	    println!("source {} @ {:?}", name, var);
            comp.sources.insert(name, var);
        }

        for (n, u) in comp.uses.iter() {
            println!("{}: {}", u, n);
        }
        // build it
        let mut vars = Vec::with_capacity(nodes.len());
        for n in nodes.iter() {
            vars.push(comp.generate(&**n)?);
        }
        Ok(vars)
    }
    
    fn generate(&mut self, node: &'a Node) -> Result<V::Var, Error> {
        if let Some(stored) = self.storage.get(node) {
            return Ok(self.vm.load(stored)); // already computed
        }
        println!("{}", node);
        let mut var = match *node {
            Node::Poly(ref poly) => {
                if let Some(i) = poly.to_int() {
                    return Ok(self.vm.make_int(i.as_i64().ok_or(Error::Overflow)?));
                }
                let mut sum = Vec::new();
                for (base, fac) in poly.factors() {
                    let fac = match fac.as_i64() {
                        Some(1) => None,
                        Some(i) => Some(self.vm.make_int(i)),
                        None => Some(self.vm.make_const(fac.as_f64()))
                    };
                    let base = match base.len() {
                        0 => None,
                        _ => {
                            let mut prod = Vec::with_capacity(base.len());
                            for &(ref v, ref n) in base.iter() {
                                let v = self.generate(v)?;
                                let n = n.as_i32().ok_or(Error::Overflow)?;
                                prod.push(match n {
                                    0 => continue, // skip it
                                    1 => v,
                                    i if i > 0 => self.vm.pow_n(v, i as u32),
                                    i => {
                                        let p = self.vm.pow_n(v, -i as u32);
                                        self.vm.inv(p)
                                    }
                                });
                            }
                            Some(self.vm.make_product(prod))
                        }
                    };   
                    sum.push(match (fac, base) {
                        (None, None) => self.vm.make_int(1),
                        (Some(f), None) => f,
                        (None, Some(b)) => b,
                        (Some(f), Some(b)) => self.vm.mul(f, b)
                    });
                }

                match sum.len() {
                    0 => self.vm.make_int(0),
                    1 => sum.pop().unwrap(),
                    _ => self.vm.make_sum(sum)
                }
            },
            Node::Var(ref name) => {
	        println!("use {}", name);
	        self.sources.remove(name.as_str()).ok_or(Error::Undefined(name.clone()))?
	    },
            Node::Apply(ref f, ref g) => match **f {
                Node::Op(Func::Transient(f)) => { 
                    use self::Transient::*;
                    let x = self.generate(g)?;
                    match f {
                        Sin => self.vm.sin(x),
                        Cos => self.vm.cos(x),
                        _ => todo!("implement all functions for avx")
                    }
                },
                _ => todo!("implement non-transient apply ops")
            },
            Node::Op(_) => bug!("operators are not allowed outside apply"),
            Node::Tuple(_) => todo!("implement tuples")
        };
        println!("{} uses for {} (stored in {:?})", self.uses[node], node, var);
        match self.uses[node] {
            0 => unreachable!(),
            1 => {},
            n => {
                self.storage.insert(node, self.vm.store(&mut var, n-1));
            }
        }
        Ok(var)
    }
}
