extern crate core;

mod utils;

use web_sys::{WebGlBuffer, WebGlProgram, WebGlTexture, WebGlUniformLocation};
use asn_render_webgl::{ RenderContext };
use web_sys::WebGlRenderingContext as GL;

pub struct View2D {
	texture: WebGlTexture,
	program: WebGlProgram,
	a_position: u32,
	u_image0: WebGlUniformLocation,
	u_image1: WebGlUniformLocation,
	vertices_buf: WebGlBuffer,
	// w_cells: i32,
	// h_cells: i32
}

pub fn new_item (
	ctx: &RenderContext, _w_cells: i32, _h_cells: i32
) -> View2D {

	let vertices_buf = asn_render_webgl::load_buffer(ctx, &utils::VERTICES);

	let program = asn_render_webgl::link_program(ctx, utils::VERTEX_SHADER, utils::FRAG_SHADER);

	let texture = asn_render_webgl::load_empty_texture(ctx);

	let a_position = ctx.gl.get_attrib_location(&program, "aPosition") as u32;
	let u_image0 =  ctx.gl.get_uniform_location(&program, "u_image0").unwrap();
	let u_image1 =  ctx.gl.get_uniform_location(&program, "u_image1").unwrap();

	View2D {
		program,
		a_position,
		texture,
		u_image0,
		u_image1,
		vertices_buf,
		// w_cells,
		// h_cells
	}
}

pub fn set_tiles (ctx: &RenderContext, item: &View2D, buf: &[u8]) {
	asn_render_webgl::update_texture(ctx, Some(&item.texture), buf);
}

pub fn draw(ctx: &RenderContext, item: &View2D) {
	ctx.gl.use_program(Some(&item.program));

	ctx.gl.bind_buffer( GL::ARRAY_BUFFER, Some(&item.vertices_buf));
	ctx.gl.vertex_attrib_pointer_with_i32(item.a_position, 2, GL::FLOAT, false, 0, 0);
	ctx.gl.enable_vertex_attrib_array(item.a_position);
	ctx.gl.bind_buffer( GL::ARRAY_BUFFER, None);

	let u_tile_size =  ctx.gl.get_uniform_location(&item.program, "uTileSize").unwrap();
	ctx.gl.uniform2f(Some(&u_tile_size), 16., 16.);

	let u_resolution =  ctx.gl.get_uniform_location(&item.program, "uResolution").unwrap();
	ctx.gl.uniform2f(Some(&u_resolution), ctx.width as f32, ctx.height as f32);

	let u_map_size =  ctx.gl.get_uniform_location(&item.program, "uMapSize").unwrap();
	ctx.gl.uniform2f(Some(&u_map_size), 32., 32.);

	ctx.gl.active_texture(GL::TEXTURE0);
	ctx.gl.bind_texture(GL::TEXTURE_2D, Some(&item.texture));
	ctx.gl.uniform1i(Some(&item.u_image0), 0);

	ctx.gl.active_texture(GL::TEXTURE1);
	ctx.gl.bind_texture(GL::TEXTURE_2D, Some(&item.texture));
	ctx.gl.uniform1i(Some(&item.u_image1), 0);

	ctx.gl.draw_arrays(GL::TRIANGLES, 0, 6);

	ctx.gl.active_texture(GL::TEXTURE0);
	ctx.gl.bind_texture(GL::TEXTURE_2D, None);

	ctx.gl.active_texture(GL::TEXTURE1);
	ctx.gl.bind_texture(GL::TEXTURE_2D, None);
}
