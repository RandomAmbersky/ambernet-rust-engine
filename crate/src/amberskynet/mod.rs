pub mod api;
mod webgl;

use api::{
    AmberNetApi,
    LoggerApi,
    RenderApi
};

use webgl::Logger as LoggerWebGl;
use webgl::Render as RenderWebGl;

pub struct AmberNet<LoggerType, RenderType> {
    logger: LoggerType,
    render: RenderType
}

impl api::AmberNetApi<LoggerWebGl, RenderWebGl> for AmberNet<LoggerWebGl, RenderWebGl> {
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


pub struct AppWebGl {
    engine: AmberNet<LoggerWebGl, RenderWebGl>
}

impl AppWebGl {
    pub fn new() -> Self {
        Self {
            engine: AmberNet::new()
        }
    }
    pub fn get_engine(&self) -> &AmberNet<LoggerWebGl, RenderWebGl> {
        &self.engine
    }
}

pub fn app() -> AppWebGl {
    AppWebGl::new()
}
