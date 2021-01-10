pub mod logger;
pub mod render;

use super::api;
use logger::log as webgl_log;

pub struct Logger {}

impl api::LoggerApi for Logger {
    fn log(&self, mess: &str) {
        webgl_log(mess)
    }
}

pub struct Render {}

impl api::RenderApi for Render {
    fn new() -> Self {
        Self {}
    }
    fn draw(&self) {}
}
