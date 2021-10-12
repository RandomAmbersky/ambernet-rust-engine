use std::sync::{Arc, Mutex};
use crate::render::Render;

use web_sys::WebGlRenderingContext as GL;
use web_sys::{WebGlProgram, WebGlBuffer};

pub struct ColorQuad {
	program: WebGlProgram,
	vertices_buf: WebGlBuffer,
	indices_buf: WebGlBuffer,
	colors_buf: WebGlBuffer,
	indices_len: i32
}

impl ColorQuad {
	pub fn new(render: &Render) -> Self {
		let vertices: [f32; 12] = [
			-0.5, 0.5, 0.0,
			-0.5, -0.5, 0.0,
			0.5, -0.5, 0.0,
			0.5, 0.5, 0.0,
		];
		let vertices_buf = render.load_vertex_buffer(&vertices);

		let indices: [u16; 6] = [3, 2, 1, 3, 1, 0];
		let indices_buf = render.load_index_buffer(&indices);

		let colors: [f32; 12] = [
			0.0, 0.0, 0.0,
			1.0, 0.0, 0.0,
			0.0, 1.0, 0.0,
			0.0, 0.0, 1.0
		];
		let colors_buf = render.load_vertex_buffer(&colors);

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
			program,
			vertices_buf,
			indices_buf,
			colors_buf,
			indices_len: indices.len() as i32
		}
	}
	pub fn draw(&self, render: &Render) {
		// Bind vertex buffer object
		render.gl.use_program(Some(&self.program));

		render.gl.bind_buffer( GL::ARRAY_BUFFER, Some(&self.vertices_buf));
		render.gl.bind_buffer( GL::ELEMENT_ARRAY_BUFFER, Some(&self.indices_buf));

		// Get the attribute location
		let coord = render.gl.get_attrib_location(&self.program, "coordinates") as u32;
		// Point an attribute to the currently bound VBO
		render.gl.vertex_attrib_pointer_with_i32(coord, 3, GL::FLOAT, false, 0, 0);
		// Enable the attribute
		render.gl.enable_vertex_attrib_array(coord);

		// bind the color buffer
		render.gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.colors_buf));
		// get the attribute location
		let color = render.gl.get_attrib_location(&self.program, "color") as u32;
		// point attribute to the volor buffer object
		render.gl.vertex_attrib_pointer_with_i32(color, 3, GL::FLOAT, false, 0, 0);
		// enable the color attribute
		render.gl.enable_vertex_attrib_array(color);

		// Clear the canvas
		render.gl.clear_color(0.5, 0.5, 0.5, 1.0);

		// Enable the depth test
		render.gl.enable(GL::DEPTH_TEST);

		// Clear the color buffer bit
		render.gl.clear(GL::COLOR_BUFFER_BIT);

		render.gl.draw_elements_with_i32(
			GL::TRIANGLES,
			self.indices_len,
			GL::UNSIGNED_SHORT,
			0,
		);

		render.logger.lock().unwrap().trace("ColorQuad draw");
	}
}

pub fn new (render: &Arc<Mutex<Render>>) -> ColorQuad {
	let render_ptr = &*render.lock().unwrap();
	ColorQuad::new(render_ptr)
}
