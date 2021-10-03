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
pub enum LogLevel {
	Fatal = 0,
	Error = 1,
	Debug = 2,
	Info = 3
}

pub struct Logger {
	log_level: u8
}

impl Logger {
	pub fn info(&self, mess: &str) {
		if self.log_level < LogLevel::Info as u8 {
			return;
		}
			utils::log(mess)
	}
	pub fn debug(&self, mess: &str) {
		if self.log_level < LogLevel::Debug as u8{
			return;
		}
		utils::log(mess)
	}
	pub fn error(&self, mess: &str) {
		if self.log_level < LogLevel::Error as u8{
			return;
		}
		utils::log(mess)
	}
	pub fn fatal(&self, mess: &str) {
		utils::log(mess)
	}
}

pub fn new (level: LogLevel) -> Logger {
	Logger {
		log_level: level as u8
	}
}
