mod amber_net_engine;
mod logger;
pub(crate) mod render;
mod math;

pub use amber_net_engine::AmberNetEngine;
pub use logger::Logger;
use crate::amberskynet::render::RenderContext;

pub fn get_engine() -> AmberNetEngine {
    let a = AmberNetEngine::new();
    let mess = format!("get_engine Ok");
    logger::Logger::log(&mess);
    a
}

pub fn get_render_ctx() -> RenderContext {
    render::get_render_ctx()
}

pub fn log(mess: &str) {
    logger::Logger::log(&mess);
}
