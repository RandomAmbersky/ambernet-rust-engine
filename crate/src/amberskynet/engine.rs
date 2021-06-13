use crate::amberskynet::logger::LoggerWebGl;
use crate::amberskynet::render::RenderWebGl;
use crate::amberskynet::api;
use crate::amberskynet::api::RenderApi;

pub struct Engine {
    logger: LoggerWebGl,
    render: RenderWebGl
}

impl api::AmberNetApi<LoggerWebGl, RenderWebGl> for Engine {
    fn new() -> Self {
        let log = LoggerWebGl{};
        Self {
            logger: log,
            render: RenderWebGl::new()
        }
    }
    fn get_log(&self) -> &LoggerWebGl {
        &self.logger
    }
    fn get_render(&self) -> &RenderWebGl {
        &self.render
    }
    fn update(&self, _time: f32) {}
}
