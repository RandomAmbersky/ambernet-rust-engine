use crate::amberskynet::logger::LoggerWebGl;
use crate::amberskynet::render::{RenderWebGl, RenderProgramBox};
use crate::amberskynet::api;
use crate::amberskynet::api::{LoggerApi};

pub struct Engine {
    logger: LoggerWebGl,
    render: RenderWebGl
}

impl Engine {
    pub(crate) fn upload_render_program (&mut self, prog: RenderProgramBox) {
        self.render.upload_program(prog);
    }
}

impl api::AmberNetApi<LoggerWebGl, RenderWebGl> for Engine {
    fn new() -> Self {
        let logger = LoggerWebGl{};
        let render = RenderWebGl::new();
        Self {
            logger,
            render
        }
    }
    fn update(&self, _time: f32) {
        let mess = format!("engine update: {}", _time);
        self.get_log().log(&mess)
    }
    fn get_log(&self) -> &LoggerWebGl {
        &self.logger
    }
    fn get_render(&self) -> &RenderWebGl {
        &self.render
    }
}
