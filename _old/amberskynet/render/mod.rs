mod utils;
mod programs;

use std::collections::HashMap;
use uuid::Uuid;
use crate::amberskynet::api;
use utils::GL as GL;
use web_sys::WebGlProgram;
use programs::test_2d::Test2D;

pub trait RenderProgram {
    fn render(&self, gl: &GL);
}

pub type RenderProgramBox = Box<dyn RenderProgram>;

pub struct RenderWebGl {
    gl: GL,
    programs: HashMap<Uuid, RenderProgramBox>,
    curr_program_id: Uuid
}

impl api::RenderApi for RenderWebGl {
    fn resize(&self, _width: f32, _height: f32) {
        self.gl.enable(GL::BLEND);
        self.gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
        self.gl.clear_color(1.0, 0.0, 1.0, 1.0); //RGBA
        self.gl.clear_depth(1.0);
    }

    fn draw(&self) {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT );
        if self.curr_program_id.is_nil() {
            return;
        }
        let prog = self.programs.get(&self.curr_program_id)
            .unwrap();
        prog.render(&self.gl);
    }
}

impl RenderWebGl {
    pub fn new() -> Self {
        Self {
            programs: HashMap::new(),
            gl: utils::get_webgl_context().unwrap(),
            curr_program_id: Uuid::nil()
        }
    }
    pub fn upload_program(&mut self, prog: RenderProgramBox) -> Uuid {
        let uuid = Uuid::new_v4();
        self.programs.insert(uuid, prog);
        self.curr_program_id = uuid;
        uuid
    }
    pub fn compile_program(&self, vert: &str, frag: &str) -> WebGlProgram {
        utils::link_program(&self.gl, vert, frag)
            .unwrap()
    }
    pub fn load_render_2d_program(
        &self,
        vert: &str,
        frag: &str,
        mesh: &[f64]) -> Test2D {
        let program = self.compile_program(vert, frag);
        let buf = utils::load_buffer(&self.gl, mesh);
        Test2D {
            u_color: self.gl.get_uniform_location(&program, "uColor").unwrap(),
            u_opacity: self.gl.get_uniform_location(&program, "uOpacity").unwrap(),
            u_transform: self.gl.get_uniform_location(&program, "uTransform").unwrap(),
            buffer: buf,
            program,
        }
    }
}
