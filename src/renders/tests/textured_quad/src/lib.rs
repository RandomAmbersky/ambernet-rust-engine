mod utils;

use web_sys::{WebGlBuffer, WebGlProgram, WebGlTexture};
use asn_render_webgl::{ RenderContext };
use web_sys::WebGlRenderingContext as GL;

pub struct TexturedQuad {
	texture: WebGlTexture,
	program: WebGlProgram,
	vertices_buf: WebGlBuffer,
	texture_coords_buf: WebGlBuffer,
	indices_buf: WebGlBuffer,
	indices_len: i32
}

pub fn new_item (
	ctx: &RenderContext
) -> TexturedQuad {

	let vertices_buf = asn_render_webgl::load_buffer(ctx, &utils::VERTICES);
	let texture_coords_buf = asn_render_webgl::load_buffer(ctx, &utils::TEXTURE_COORDINATES);

	let indices_buf = asn_render_webgl::load_index_buffer(ctx, &utils::INDICES);

	let program = asn_render_webgl::link_program(ctx, utils::VERTEX_SHADER, utils::FRAG_SHADER).unwrap();

	let texture = asn_render_webgl::load_texture(ctx, utils::TEXTURE);

	TexturedQuad {
		program,
		vertices_buf,
		indices_buf,
		texture_coords_buf,
		texture,
		indices_len: utils::INDICES.len() as i32
	}
}

pub fn draw(ctx: &RenderContext, item: &TexturedQuad) {
	ctx.gl.use_program(Some(&item.program));

	ctx.gl.bind_buffer( GL::ELEMENT_ARRAY_BUFFER, Some(&item.indices_buf));

	ctx.gl.bind_buffer( GL::ARRAY_BUFFER, Some(&item.vertices_buf));
	let coord = ctx.gl.get_attrib_location(&item.program, "coordinates") as u32;
	ctx.gl.vertex_attrib_pointer_with_i32(coord, 3, GL::FLOAT, false, 0, 0);
	ctx.gl.enable_vertex_attrib_array(coord);

	ctx.gl.bind_buffer( GL::ARRAY_BUFFER, Some(&item.texture_coords_buf));
	let texture_coord = ctx.gl.get_attrib_location(&item.program, "aTextureCoord") as u32;
	ctx.gl.vertex_attrib_pointer_with_i32(texture_coord, 2, GL::FLOAT, false, 0, 0);
	ctx.gl.enable_vertex_attrib_array(texture_coord);

	let u_sampler =  ctx.gl.get_uniform_location(&item.program, "uSampler").unwrap();
	ctx.gl.active_texture(GL::TEXTURE0);
	ctx.gl.bind_texture(GL::TEXTURE_2D, Some(&item.texture));
	ctx.gl.uniform1i(Some(&u_sampler), 0);

	ctx.gl.draw_elements_with_i32(
		GL::TRIANGLES,
		item.indices_len,
		GL::UNSIGNED_SHORT,
		0,
	);
}
