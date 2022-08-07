mod utils;

use web_sys::{WebGlBuffer, WebGlProgram};
use asn_render_webgl::RenderContext;
use web_sys::WebGlRenderingContext as GL;

pub struct Triangle {
	program: WebGlProgram,
	a_coordinates: u32,
	vertices_buf: WebGlBuffer,
	indices_buf: WebGlBuffer,
	indices_len: i32
}

pub fn new_item (
	ctx: &RenderContext
) -> Result<Triangle, String> {

	let vertices_buf = asn_render_webgl::load_buffer(ctx, &utils::VERTICES);

	let indices_buf = asn_render_webgl::load_index_buffer(ctx, &utils::INDICES);

	let program = asn_render_webgl::link_program(ctx, utils::VERTEX_SHADER, utils::FRAG_SHADER)?;
	let a_coordinates = ctx.gl.get_attrib_location(&program, "aCoordinates") as u32;

	let triangle = Triangle {
		program,
		a_coordinates,
		vertices_buf,
		indices_buf,
		indices_len: utils::INDICES.len() as i32
	};
	Ok(triangle)
}

pub fn draw(ctx: &RenderContext, item: &Triangle) {
	ctx.gl.use_program(Some(&item.program));

	ctx.gl.bind_buffer( GL::ARRAY_BUFFER, Some(&item.vertices_buf));
	ctx.gl.vertex_attrib_pointer_with_i32(item.a_coordinates, 3, GL::FLOAT, false, 0, 0);
	ctx.gl.enable_vertex_attrib_array(item.a_coordinates);

	ctx.gl.bind_buffer( GL::ELEMENT_ARRAY_BUFFER, Some(&item.indices_buf));
	ctx.gl.draw_elements_with_i32(
		GL::TRIANGLES,
		item.indices_len,
		GL::UNSIGNED_SHORT,
		0,
	);

	ctx.gl.bind_buffer( GL::ELEMENT_ARRAY_BUFFER, None);
	ctx.gl.use_program(None);
}
