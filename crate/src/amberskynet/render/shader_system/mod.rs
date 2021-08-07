use std::collections::HashMap;
use uuid::Uuid;

use crate::amberskynet::render::utils::GL;
use web_sys::WebGlProgram;
use crate::amberskynet::render::shader_system::test_2d::Test2D;

pub mod test_2d;
mod utils;

pub trait RenderProgram {
    fn render(&self, gl: &GL);
}
pub type RenderProgramBox = Box<dyn RenderProgram>;

pub struct ShaderProgram {
    pub(crate) programs: HashMap<Uuid, RenderProgramBox>,
    pub(crate) curr_program_id: Uuid,
}

impl ShaderProgram {
    pub fn new() -> Self {
        Self {
            programs: HashMap::new(),
            curr_program_id: Uuid::nil()
        }
    }
    pub fn upload_program(&mut self, prog: RenderProgramBox) -> Uuid {
        let uuid = Uuid::new_v4();
        self.programs.insert(uuid, prog);
        self.curr_program_id = uuid;
        uuid
    }
    pub fn compile_program(&self, gl: &GL, vert: &str, frag: &str) -> WebGlProgram {
        utils::link_program(gl, vert, frag)
            .unwrap()
    }
    pub fn load_render_2d_program(
        &self,
        gl: &GL,
        vert: &str,
        frag: &str,
        mesh: &[f64]) -> Test2D {
        let program = self.compile_program(gl, vert, frag);
        let buf = utils::load_buffer(gl, mesh);
        Test2D {
            u_color: gl.get_uniform_location(&program, "uColor").unwrap(),
            u_opacity: gl.get_uniform_location(&program, "uOpacity").unwrap(),
            u_transform: gl.get_uniform_location(&program, "uTransform").unwrap(),
            buffer: buf,
            program,
        }
    }
}
