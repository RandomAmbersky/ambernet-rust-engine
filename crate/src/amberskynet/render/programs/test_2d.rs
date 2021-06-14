use web_sys::{WebGlProgram, WebGlBuffer};
use crate::amberskynet::render::utils::GL;
use crate::amberskynet::render::RenderProgram;
use crate::amberskynet::logger::LoggerWebGl;

pub struct Test2D {
    pub program: WebGlProgram,
    pub buffer: WebGlBuffer
}

impl RenderProgram for Test2D {
    fn render(&self, _gl: &GL) {
        LoggerWebGl::log(&"RenderProgram for Test2D".to_string());
    }
}
