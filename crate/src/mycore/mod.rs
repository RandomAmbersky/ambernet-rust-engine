mod logger;
mod render;

use std::sync::Arc;
pub use logger::Logger as Logger;
pub use logger::LogLevel;

pub use render::Render as Render;

pub fn new_logger (log_level: LogLevel) -> Arc<Logger> {
	let logger = logger::new(log_level);
	Arc::new(logger)
}

pub fn new_render (logger: &Arc<Logger>) -> Arc<Render> {
	let render = render::new(logger);
	Arc::new(render)
}
