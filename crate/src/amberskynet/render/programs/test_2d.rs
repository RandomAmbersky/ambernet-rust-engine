use web_sys::{WebGlProgram, WebGlBuffer};
use crate::amberskynet::render::utils::GL;

pub struct Test2D {
    pub program: WebGlProgram,
    pub buffer: WebGlBuffer
}

impl Test2D {
    fn render(&self, _gl: GL) {

    }
}
