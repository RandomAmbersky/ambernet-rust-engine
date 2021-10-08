mod gl_utils;

use std::sync::{Arc, Mutex};
use crate::mycore::Logger;
use gl_utils::GL;

#[allow(dead_code)]
pub struct Render {
	logger: Arc<Mutex<Logger>>,
	gl: GL
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
	let gl = gl_utils::get_webgl_context().unwrap();
	let render = Render {
		logger: logger.clone(),
		gl
	};
	Arc::new(Mutex::new(render))
}
