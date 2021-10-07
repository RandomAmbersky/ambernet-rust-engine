use std::sync::Arc;
use crate::mycore::Logger;

pub struct Render {
	logger: Arc<Logger>
}

pub fn new (logger: &Arc<Logger>) -> Render {
	Render {
		logger: Arc::clone(logger)
	}
}

impl Render {
	pub fn resize (&self, width: i32, height: i32){
		let mess = format!("Render resize {} x {}", width, height);
		self.logger.trace(&mess);
	}
	pub fn draw (&self){
		self.logger.trace("Render draw...")
	}
}
