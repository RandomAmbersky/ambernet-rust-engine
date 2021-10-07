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
	pub fn draw (&self){
		self.logger.debug("Render draw...")
	}
}
