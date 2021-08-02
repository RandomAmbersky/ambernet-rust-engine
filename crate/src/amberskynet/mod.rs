mod amber_net_engine;
mod logger;
pub(crate) mod render;

pub use amber_net_engine::AmberNetEngine;
pub use logger::Logger;

pub fn get_engine() -> AmberNetEngine {
    let a = AmberNetEngine::new();
    let mess = format!("get_engine Ok");
    logger::Logger::log(&mess);
    a
}

pub fn log(mess: &str) {
    logger::Logger::log(&mess);
}
