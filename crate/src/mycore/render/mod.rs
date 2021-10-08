use std::sync::{Arc, Mutex};
use crate::mycore::Logger;

pub struct Render {
	logger: Arc<Mutex<Logger>>
}

pub fn new (logger: &Arc<Mutex<Logger>>) -> Render {
	Render {
		logger: Arc::clone(logger)
	}
}

impl Render {
	pub fn resize (&self, width: i32, height: i32){
		let mess = format!("Render resize {} x {}", width, height);
		self.logger.lock().unwrap().trace(&mess);
	}
	pub fn draw (&self){
		self.logger.lock().unwrap().trace("Render draw...")
	}
}
