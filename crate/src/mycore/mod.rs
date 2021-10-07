mod logger;
mod render;

use std::sync::{Arc, Mutex};
pub use logger::Logger as Logger;
pub use logger::LogLevel;

pub use render::Render as Render;

pub fn new_logger (log_level: LogLevel) -> Arc<Mutex<Logger>> {
	let logger = logger::new(log_level);
	Arc::new(Mutex::new(logger))
}

pub fn new_render (logger: &Arc<Mutex<Logger>>) -> Arc<Mutex<Render>> {
	let render = render::new(logger);
	Arc::new(Mutex::new(render))
}
