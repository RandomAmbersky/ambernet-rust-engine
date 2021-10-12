mod gl_utils;

use std::sync::{Arc, Mutex};
use crate::mycore::Logger;
use gl_utils::GL;
use web_sys::{WebGlProgram, WebGlBuffer};

pub struct Screen {
	w: i32,
	h: i32
}

#[allow(dead_code)]
pub struct Render {
	pub logger: Arc <Mutex<Logger>>,
	pub gl: GL,
	scr: Screen
}

impl Render {
	pub fn gl (&self) -> &GL {
		&self.gl
	}
	pub fn resize (&mut self, width: i32, height: i32){
		self.scr = Screen { w: width, h: height };
		gl_utils::resize(&self.gl, width, height);
		let mess = format!("Render resize {} x {}", width, height);
		self.logger.lock().unwrap().trace(&mess);
	}
	pub fn draw (&self){
		gl_utils::clear(&self.gl);
		self.logger.lock().unwrap().trace("Render draw...")
	}
	pub fn link_program (&self, vert: &str, frag: &str) -> WebGlProgram {
		gl_utils::link_program(&self.gl, vert, frag).unwrap()
	}
	pub fn load_vertex_buffer (&self, buf: &[f64]) -> WebGlBuffer {
		gl_utils::load_vertex_buffer(&self.gl, buf)
	}
	pub fn load_index_buffer (&self, buf: &[u16]) -> WebGlBuffer {
		gl_utils::load_index_buffer(&self.gl, buf)
	}
}

pub fn new_render (logger: &Arc<Mutex<Logger>>) -> Arc<Mutex<Render>> {
	let gl = gl_utils::get_webgl_context().unwrap();
	let render = Render {
		logger: logger.clone(),
		gl,
		scr: Screen { w: 0, h: 0 }
	};
	Arc::new(Mutex::new(render))
}
