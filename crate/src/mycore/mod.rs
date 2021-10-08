use std::sync::{Arc, Mutex};

pub use logger::Logger as Logger;
pub use logger::LogLevel;

mod logger;

extern crate console_error_panic_hook;

pub fn new_logger (log_level: LogLevel) -> Arc<Mutex<Logger>> {
	let logger = logger::new(log_level);
	Arc::new(Mutex::new(logger))
}

pub fn set_panic_hook() {
	#[cfg(feature = "console_error_panic_hook")]
		console_error_panic_hook::set_once();
}
