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
    programs: HashMap<Uuid, RenderProgramBox>
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
    }
}

impl RenderWebGl {
    pub fn new() -> Self {
        Self {
            programs: HashMap::new(),
            gl: utils::get_webgl_context().unwrap()
        }
    }
    pub fn compile_program(&self, vert: &str, frag: &str) -> WebGlProgram {
        utils::link_program(&self.gl, vert, frag)
            .unwrap()
    }
    pub fn upload_program(&mut self, prog: RenderProgramBox) -> Uuid {
        let uuid = Uuid::new_v4();
        self.programs.insert(uuid, prog);
        uuid
    }
    pub fn load_2d_program(
        &self,
        vert: &str,
        frag: &str,
        mesh: &[f64]) -> Test2D {
        let prog = self.compile_program(vert, frag);
        let buf = utils::load_buffer(&self.gl, mesh);
        Test2D {
            program: prog,
            buffer: buf
        }
    }
}
