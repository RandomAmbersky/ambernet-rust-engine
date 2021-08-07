mod utils;
mod shader_system;
// mod font_system;

use crate::amberskynet::api;
use utils::GL as GL;
// use web_sys::WebGlProgram;
// use shader_system::test_2d::Test2D;
use shader_system::ShaderProgram;

pub struct RenderWebGl {
    gl: GL,
    shaders: ShaderProgram
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
        let prog_id = &self.shaders.curr_program_id;
        if prog_id.is_nil() {
            return;
        }
        let prog = self.shaders.programs.get(prog_id)
            .unwrap();
        prog.render(&self.gl);
    }
}

impl RenderWebGl {
    pub fn new() -> Self {
        let gl = utils::get_webgl_context().unwrap();
        Self {
            gl,
            shaders: ShaderProgram::new()
        }
    }
    pub fn get_gl(&self) -> &GL {
        &self.gl
    }
    pub fn get_shaders(&self) -> &ShaderProgram {
        &self.shaders
    }
}
