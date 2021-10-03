mod logger;

pub use logger::Logger as Logger;
pub use logger::LogLevel;

pub fn new_logger (log_level: LogLevel) -> Logger {
	logger::new(log_level)
}
