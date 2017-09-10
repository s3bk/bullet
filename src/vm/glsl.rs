use compiler::Compiler;
use vm::{Vm, Round};
use node::NodeRc;
use quote::{Tokens, Ident};
use std::mem;

struct Glsl {
    shader: String,
    decl: String,
    stored: usize,
}
impl Glsl {
    pub fn new() -> Glsl {
        Glsl {
            shader: String::new(),
            decl: String::new(),
            stored: 0,
        }
    }
}

impl Vm for Glsl {
    type Var = String;
    type Storage = String;

    fn make_int(&mut self, i: i64) -> Self::Var {
        format!("{}", i)
    }
    fn make_const(&mut self, x: f64) -> Self::Var {
        format!("{}", x)
    }
    fn make_source(&mut self, name: &str) -> Self::Var {
        let name = match name {
            "x" => "pos.x",
            "y" => "pos.y",
            "t" => "u_time",
            name => name
        };
        format!("{}", name)
    }
    fn make_sum(&mut self, parts: Vec<Self::Var>) -> Self::Var {
        let start = parts[0].to_string();
        parts.iter().skip(1).fold(start, |acc, ref x| {
            acc.clone() + " + " + x.as_str()
        })
    }
    fn make_product(&mut self, parts: Vec<Self::Var>) -> Self::Var {
        let start = parts[0].to_string();
        parts.iter().skip(1).fold(start, |acc, ref x| {
            acc.clone() + " * " + x.as_str()
        })
    }
    fn store(&mut self, var: &mut Self::Var, _uses: usize) -> Self::Storage {
        let name = format!("storage_{}", self.stored);
        self.stored += 1;
        let var = mem::replace(var, self.load(&name));
        self.decl += format!("float {} = {};\n", name, var).as_str();
        name
    }
    fn load(&mut self, name: &Self::Storage) -> Self::Var {
        name.to_string()
    }
    fn round(&mut self, x: Self::Var, mode: Round) -> Self::Var {
        match mode {
            Round::Up => format!("ceil({})", x),
            Round::Down => format!("floor({})", x),
        }
    }
    fn div(&mut self, a: Self::Var, b: Self::Var) -> Self::Var {
        format!("{}/{}", a, b)
    }
    fn inv(&mut self, a: Self::Var) -> Self::Var {
        format!("1/{}", a)
    }
    fn step_at(&mut self, at: Self::Var, x: Self::Var) -> Self::Var {
        format!("step({}, {})", at, x)
    }
}

/// Returns (vert shader, frag shader)
pub fn glsl(input: NodeRc) -> (String, String) {
    let mut glsl = Glsl::new();
    let shader_code = Compiler::run(&mut glsl, &input);

    let vert = "\
#version 330
in vec2 v_pos;
out vec2 pos;

void main() {
    pos = v_pos;
    gl_Position = vec4(v_pos, 0, 1);
}
".to_string();

    let frag = format!("\
#version 330
in vec2 pos;
uniform float u_time;
out vec4 final_col;
{}
void main() {{
    final_col = vec4(vec3({}), 1);
}}",
        glsl.decl,
        shader_code);

    (vert, frag)
}
