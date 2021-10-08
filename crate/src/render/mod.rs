use std::sync::{Arc, Mutex};
use crate::mycore::Logger;

pub struct Render {
	logger: Arc<Mutex<Logger>>
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

pub fn new_render (logger: &Arc<Mutex<Logger>>) -> Arc<Mutex<Render>> {
	let render = Render {
		logger: logger.clone()
	};
	Arc::new(Mutex::new(render))
}
