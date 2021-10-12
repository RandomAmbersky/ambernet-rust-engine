use std::sync::{Arc, Mutex};
use crate::render::Render;

use web_sys::{WebGlProgram};

pub struct ColorQuad {
	program: WebGlProgram
}

impl ColorQuad {
	pub fn new(render: &Render) -> Self {
		let vertices: [f64; 12] = [
			-0.5, 0.5, 0.0,
			-0.5, -0.5, 0.0,
			0.5, -0.5, 0.0,
			0.5, 0.5, 0.0,
		];
		let vertices_array = render.load_vertex_buffer(&vertices);

		let indices: [u16; 6] = [3, 2, 1, 3, 1, 0];
		let index_array = render.load_index_buffer(&indices);

		let colors: [f32; 12] = [0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0];

		// vertex shader source code
		let vert_code = r#"attribute vec3 coordinates;
attribute vec3 color;
varying vec3 vColor;
void main(void) {
   gl_Position = vec4(coordinates, 1.0);
   vColor = color;
}
"#;

		let frag_code = r#"precision mediump float;
varying vec3 vColor;
void main(void) {
    gl_FragColor = vec4(vColor, 1.);
}"#;

		render.logger.lock().unwrap().info("ColorQuad new");
		let program = render.link_program(vert_code, frag_code);
		Self {
			program
		}
	}
	pub fn draw(&self, render: &Render) {
		render.logger.lock().unwrap().trace("ColorQuad draw");
	}
}

pub fn new (render: &Arc<Mutex<Render>>) -> ColorQuad {
	let render_ptr = &*render.lock().unwrap();
	ColorQuad::new(render_ptr)
}
