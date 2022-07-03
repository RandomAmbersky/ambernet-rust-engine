mod shaders;
mod buffer;

use web_sys::WebGlRenderingContext as GL;
use web_sys::{WebGlBuffer, WebGlProgram};
use asn_render_webgl::RenderContext;

#[allow(dead_code)]
pub struct View2D {
	program: WebGlProgram,
	buffer: WebGlBuffer,
	w_cells: i32,
	h_cells: i32
}

pub fn new_item (ctx: &RenderContext, w_cells: i32, h_cells: i32) -> View2D {
	let program = asn_render_webgl::link_program(ctx, shaders::VERT_SHADER, shaders::FRAG_SHADER);
	let buffer = asn_render_webgl::load_buffer(ctx, &buffer::SIMPLE_BUF);

	View2D {
		w_cells, h_cells,
		program,
		buffer
	}
}

pub fn draw (ctx: &RenderContext, item: &View2D) {
	let gl = &ctx.gl;
	gl.clear(GL::COLOR_BUFFER_BIT);
	gl.use_program(Some(&item.program));

	let a_position: u32 = ctx.gl.get_attrib_location(&item.program, "a_Position") as u32;
	ctx.gl.vertex_attrib_pointer_with_i32(a_position, 3, GL::FLOAT, false, 0, 0);
	ctx.gl.enable_vertex_attrib_array(3);

	// let min_size = gl.get_parameter(GL::ALIASED_POINT_SIZE_RANGE);
	// let a_Position: u32 = gl.get_attrib_location(&item.program, "a_Position");
	// gl.vertex_attrib_pointer_with_float_dimension(a_Position, 3., GL::FLOAT, false, 0, 0);

	gl.bind_buffer(GL::ARRAY_BUFFER, Some(&item.buffer));
	gl.draw_arrays(GL::TRIANGLES, 0, 6);
}
