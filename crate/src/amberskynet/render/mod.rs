mod utils;

use crate::amberskynet::api;
use utils::GL as GL;

pub struct RenderWebGl {
    gl: GL
}

impl api::RenderApi for RenderWebGl {
    fn new() -> Self {
        Self {
            gl: utils::get_webgl_context().unwrap()
        }
    }

    fn resize(&self, _width: f32, _height: f32) {
        self.gl.enable(GL::BLEND);
        self.gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
        self.gl.clear_color(1.0, 0.0, 1.0, 1.0); //RGBA
        self.gl.clear_depth(1.0);
    }

    fn draw(&self) {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT );
    }

    fn compile_program(&self, vert: &str, frag: &str) {
        utils::link_program(&self.gl, vert, frag)
            .unwrap();
    }
}
