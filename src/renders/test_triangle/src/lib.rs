mod utils;

use web_sys::{WebGlBuffer, WebGlProgram};
use asn_render_webgl::RenderContext;
use web_sys::WebGlRenderingContext as GL;

pub struct ColorQuad {
	program: WebGlProgram,
	vertices_buf: WebGlBuffer,
	indices_buf: WebGlBuffer,
	colors_buf: WebGlBuffer,
	indices_len: i32
}

pub fn new_item (
	ctx: &RenderContext
) -> ColorQuad {

	let vertices_buf = asn_render_webgl::load_buffer(ctx, &utils::VERTICES);

	let indices_buf = asn_render_webgl::load_index_buffer(ctx, &utils::INDICES);

	let colors_buf = asn_render_webgl::load_buffer(ctx, &utils::COLORS);

	let program = asn_render_webgl::link_program(ctx, utils::VERTEX_SHADER, utils::FRAG_SHADER).unwrap();

	ColorQuad {
		program,
		vertices_buf,
		indices_buf,
		colors_buf,
		indices_len: utils::INDICES.len() as i32
	}
}

pub fn draw(ctx: &RenderContext, item: &ColorQuad) {
	// Bind vertex buffer object
	ctx.gl.use_program(Some(&item.program));

	ctx.gl.bind_buffer( GL::ARRAY_BUFFER, Some(&item.vertices_buf));
	ctx.gl.bind_buffer( GL::ELEMENT_ARRAY_BUFFER, Some(&item.indices_buf));

	// Get the attribute location
	let coord = ctx.gl.get_attrib_location(&item.program, "coordinates") as u32;
	// Point an attribute to the currently bound VBO
	ctx.gl.vertex_attrib_pointer_with_i32(coord, 3, GL::FLOAT, false, 0, 0);
	// Enable the attribute
	ctx.gl.enable_vertex_attrib_array(coord);

	// bind the color buffer
	ctx.gl.bind_buffer(GL::ARRAY_BUFFER, Some(&item.colors_buf));
	// get the attribute location
	let color = ctx.gl.get_attrib_location(&item.program, "color") as u32;
	// point attribute to the volor buffer object
	ctx.gl.vertex_attrib_pointer_with_i32(color, 3, GL::FLOAT, false, 0, 0);
	// enable the color attribute
	ctx.gl.enable_vertex_attrib_array(color);

	ctx.gl.draw_elements_with_i32(
		GL::TRIANGLES,
		item.indices_len,
		GL::UNSIGNED_SHORT,
		0,
	);
}
