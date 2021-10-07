mod utils {
	extern crate wasm_bindgen;
	use wasm_bindgen::prelude::*;

	#[wasm_bindgen]
	extern "C" {
		#[wasm_bindgen(js_namespace = console)]
		pub fn log(s: &str);
	}
}

#[repr(u8)]
#[allow(dead_code)]
pub enum LogLevel {
	Fatal = 0,
	Error = 1,
	Warn = 2,
	Info = 3,
	Debug = 4,
	Trace = 5
}

pub struct Logger {
	log_level: u8
}

impl Logger {
	#[allow(dead_code)]
	pub fn fatal(&self, mess: &str) {
		utils::log(mess);
	}
	#[allow(dead_code)]
	pub fn error(&self, mess: &str) {
		if self.log_level < LogLevel::Error as u8{
			return;
		}
		utils::log(mess)
	}
	#[allow(dead_code)]
	pub fn warn(&self, mess: &str) {
		if self.log_level < LogLevel::Warn as u8{
			return;
		}
		utils::log(mess)
	}
	#[allow(dead_code)]
	pub fn info(&self, mess: &str) {
		if self.log_level < LogLevel::Info as u8 {
			return;
		}
			utils::log(mess)
	}
	#[allow(dead_code)]
	pub fn debug(&self, mess: &str) {
		if self.log_level < LogLevel::Debug as u8{
			return;
		}
		utils::log(mess)
	}
	#[allow(dead_code)]
	pub fn trace(&self, mess: &str) {
		if self.log_level < LogLevel::Trace as u8{
			return;
		}
		utils::log(mess)
	}
}

pub fn new (level: LogLevel) -> Logger {
	Logger {
		log_level: level as u8
	}
}
