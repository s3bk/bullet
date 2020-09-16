use parity_wasm::elements::{
    Instructions,
    Instruction, Local, ValueType,
    Module
};
use parity_wasm::builder;

use vm::{Vm, Round};
use compiler::Compiler;
use node::NodeRc;
use error::Error;

pub struct Wasm<'a> {
    instructions: Vec<Instruction>,
    inputs: &'a [&'a str],
    locals: u32,
}

#[derive(Debug)]
pub struct LocalVar(u32);

impl<'a> Wasm<'a> {
    fn fold(&mut self, parts: Vec<Vec<Instruction>>, instruction: Instruction) -> Vec<Instruction> {
        let mut parts = parts.into_iter();
        let mut instr = parts.next().unwrap();
        for mut other in parts {
            instr.append(&mut other);
            instr.push(instruction.clone());
        }
        instr
    }

    fn to_module(self) -> Module {
        builder::module()
            .function()
                .signature()
                    .with_params(vec![ValueType::F64; self.inputs.len()])
                    .with_return_type(Some(ValueType::F64))
                    .build()
                .body()
                    .with_locals(vec![Local::new(self.locals, ValueType::F64)])
                    .with_instructions(Instructions::new(self.instructions))
                    .build()
                .build()
            .export()
                .field("f")
                .internal().func(0)
                .build()
            .build()
    }

    pub fn compile(node: &NodeRc, inputs: &[&str]) -> Result<Vec<u8>, Error> {
        let mut w =  Wasm {
            instructions: vec![],
            inputs: inputs.into(),
            locals: 0
        };
        let mut instr = Compiler::run(&mut w, node)?;
        w.instructions.append(&mut instr);
        w.instructions.push(Instruction::End);
        dbg!(&w.instructions);
        Ok(w.to_module().to_bytes().expect("can't build module"))
    }
}
impl<'a> Vm for Wasm<'a> {
    type Var = Vec<Instruction>;
    type Storage = LocalVar;
    
    fn make_const(&mut self, c: f64) -> Self::Var {
        vec![Instruction::F64Const(c.to_bits())]
    }
    fn make_source(&mut self, name: &str) -> Self::Var {
        let idx = self.inputs.iter().position(|&n| n == name).expect("input is not defined");
        vec![Instruction::GetLocal(idx as u32)]
    }
    fn sub(&mut self, a: Self::Var, mut b: Self::Var) -> Self::Var {
        let mut instr = a;
        instr.append(&mut b);
        instr.push(Instruction::F64Sub);
        instr
    }
    fn make_sum(&mut self, parts: Vec<Self::Var>) -> Self::Var {
        self.fold(parts, Instruction::F64Add)
    }
    fn make_product(&mut self, parts: Vec<Self::Var>) -> Self::Var {
        self.fold(parts, Instruction::F64Mul)
    }
    fn store(&mut self, var: &mut Self::Var, uses: usize) -> Self::Storage {
        let idx = self.locals;
        self.locals += 1;
        self.instructions.extend_from_slice(&**var);
        self.instructions.push(Instruction::SetLocal(idx));
        
        *var = vec![Instruction::GetLocal(idx)];
        
        LocalVar(idx)
    }
    fn load(&mut self, storage: &Self::Storage) -> Self::Var {
        let LocalVar(idx) = *storage;
        vec![Instruction::GetLocal(idx)]
    }
    fn round(&mut self, mut x: Self::Var, mode: Round) -> Self::Var {
        let instr = match mode {
            Round::Up => Instruction::F64Ceil,
            Round::Down => Instruction::F64Floor
        };
        x.push(instr);
        x
    }
    fn step_at(&mut self, mut at: Self::Var, mut x: Self::Var) -> Self::Var {
        let mut instr = vec![Instruction::F64Const(0.0f64.to_bits()), Instruction::F64Const(1.0f64.to_bits())];
        instr.append(&mut at);
        instr.append(&mut x);
        instr.extend_from_slice(&[Instruction::F64Ge, Instruction::Select]);
        instr
    }
    fn div(&mut self, a: Self::Var, mut b: Self::Var) -> Self::Var {
        let mut instr = a;
        instr.append(&mut b);
        instr.push(Instruction::F64Div);
        instr
    }
}
