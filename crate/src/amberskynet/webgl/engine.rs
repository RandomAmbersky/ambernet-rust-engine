use super::super::api as api;
use super::logger::Logger as LoggerWebGl;
use super::render::Render as RenderWebGl;
use crate::amberskynet::api::RenderApi;

pub struct Engine {
    logger: LoggerWebGl,
    render: RenderWebGl
}

impl api::AmberNetApi<LoggerWebGl, RenderWebGl> for Engine {
    fn new() -> Self {
        Self {
            logger: LoggerWebGl{},
            render: RenderWebGl::new()
        }
    }

    fn get_log(&self) -> &LoggerWebGl {
        &self.logger
    }

    fn get_render(&self) -> &RenderWebGl {
        &self.render
    }
}
