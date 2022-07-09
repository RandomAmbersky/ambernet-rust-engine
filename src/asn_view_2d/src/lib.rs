mod utils;

use web_sys::{WebGlBuffer, WebGlProgram, WebGlTexture, WebGlUniformLocation};
use asn_render_webgl::{ RenderContext };
use web_sys::WebGlRenderingContext as GL;

pub struct View2D {
	texture: WebGlTexture,
	program: WebGlProgram,
	a_coordinates: u32,
	a_texture_coord: u32,
	u_sampler: WebGlUniformLocation,
	vertices_buf: WebGlBuffer,
	texture_coords_buf: WebGlBuffer,
	indices_buf: WebGlBuffer,
	indices_len: i32,
	w_cells: i32,
	h_cells: i32
}

pub fn new_item (
	ctx: &RenderContext, w_cells: i32, h_cells: i32
) -> View2D {

	let vertices_buf = asn_render_webgl::load_buffer(ctx, &utils::VERTICES);
	let texture_coords_buf = asn_render_webgl::load_buffer(ctx, &utils::TEXTURE_COORDINATES);

	let indices_buf = asn_render_webgl::load_index_buffer(ctx, &utils::INDICES);

	let program = asn_render_webgl::link_program(ctx, utils::VERTEX_SHADER, utils::FRAG_SHADER);

	let texture = asn_render_webgl::load_empty_texture(ctx);

	let a_coordinates = ctx.gl.get_attrib_location(&program, "aCoordinates") as u32;
	let a_texture_coord = ctx.gl.get_attrib_location(&program, "aTextureCoord") as u32;
	let u_sampler =  ctx.gl.get_uniform_location(&program, "uSampler").unwrap();

	View2D {
		program,
		a_coordinates,
		a_texture_coord,
		u_sampler,
		vertices_buf,
		indices_buf,
		texture_coords_buf,
		texture,
		indices_len: utils::INDICES.len() as i32,
		w_cells,
		h_cells
	}
}

pub fn draw(ctx: &RenderContext, item: &View2D) {
	ctx.gl.use_program(Some(&item.program));

	ctx.gl.bind_buffer( GL::ARRAY_BUFFER, Some(&item.vertices_buf));
	ctx.gl.vertex_attrib_pointer_with_i32(item.a_coordinates, 3, GL::FLOAT, false, 0, 0);
	ctx.gl.enable_vertex_attrib_array(item.a_coordinates);
	ctx.gl.bind_buffer( GL::ARRAY_BUFFER, None);

	ctx.gl.bind_buffer( GL::ARRAY_BUFFER, Some(&item.texture_coords_buf));
	ctx.gl.vertex_attrib_pointer_with_i32(item.a_texture_coord, 2, GL::FLOAT, false, 0, 0);
	ctx.gl.enable_vertex_attrib_array(item.a_texture_coord);
	ctx.gl.bind_buffer( GL::ARRAY_BUFFER, None);

	ctx.gl.active_texture(GL::TEXTURE0);
	ctx.gl.bind_texture(GL::TEXTURE_2D, Some(&item.texture));
	ctx.gl.uniform1i(Some(&item.u_sampler), 0);

	ctx.gl.bind_buffer( GL::ELEMENT_ARRAY_BUFFER, Some(&item.indices_buf));

	ctx.gl.draw_elements_with_i32(
		GL::TRIANGLES,
		item.indices_len,
		GL::UNSIGNED_SHORT,
		0,
	);

	ctx.gl.bind_texture(GL::TEXTURE_2D, None);
	// ctx.gl.delete_texture(Some(&item.texture));
	ctx.gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, None);
	ctx.gl.use_program(None);
}
