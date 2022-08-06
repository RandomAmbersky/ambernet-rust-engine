extern crate core;

mod utils;
mod map;

use amberskynet_logger_web::LoggerWeb;
use web_sys::{WebGlBuffer, WebGlProgram, WebGlTexture, WebGlUniformLocation};
use asn_render_webgl::{ RenderContext };
use web_sys::WebGlRenderingContext as GL;
use map::Map;

pub struct View2D {
	texture: WebGlTexture,
	map_texture: WebGlTexture,
	program: WebGlProgram,
	a_position: u32,
	u_transform: WebGlUniformLocation,
	transform_matrix: [f32;16],
	u_image0: WebGlUniformLocation,
	u_image1: WebGlUniformLocation,
	vertices_buf: WebGlBuffer,
	map: Map
}

pub fn new_item (
	ctx: &RenderContext, _w_cells: i32, _h_cells: i32
) -> View2D {

	let vertices_buf = asn_render_webgl::load_buffer(ctx, &utils::VERTICES);

	let program = asn_render_webgl::link_program(ctx, utils::VERTEX_SHADER, utils::FRAG_SHADER);

	let texture = asn_render_webgl::load_empty_texture(ctx);
	let map_texture = asn_render_webgl::load_empty_texture(ctx);

	let a_position = ctx.gl.get_attrib_location(&program, "aPosition") as u32;

	let u_transform =  ctx.gl.get_uniform_location(&program, "uTransform").unwrap();
	let mut transform_matrix = asn_math::mult_matrix_4(
		asn_math::IDENTITY_MATRIX,
		asn_math::translation_matrix(-0.5, -0.5, 0.)
	);

	let u_image0 =  ctx.gl.get_uniform_location(&program, "u_image0").unwrap();
	let u_image1 =  ctx.gl.get_uniform_location(&program, "u_image1").unwrap();

	View2D {
		program,
		a_position,
		texture,
		map_texture,
		u_transform,
		transform_matrix,
		u_image0,
		u_image1,
		vertices_buf,
		map: Map::default()
	}
}

pub fn set_tiles (ctx: &RenderContext, item: &View2D, buf: &[u8]) {
	asn_render_webgl::update_texture(ctx, Some(&item.texture), buf);
}

pub fn set_map (ctx: &RenderContext, item: &mut View2D, width: u32, height: u32, buf: &[u8]) {

	let mut map_texture: Vec<u8> = Vec::new();

	for cell in buf.into_iter() {
		let index = cell - 1;
		let g = index / 16;
		let r = index - g * 16;

		let index_check = g * 16 + r;

		let mess = format!("cell {:?} [{:?}, {:?}] {:?}", cell, g, r, index_check);
		LoggerWeb::log(&mess);
		map_texture.push(r);
		map_texture.push(g);
		map_texture.push(255);
		map_texture.push(255);
	}

	asn_render_webgl::update_raw_texture(ctx, Some(&item.map_texture), width as i32, height as i32, &map_texture);

	item.map.width = width as i32;
	item.map.height = height as i32;

	// move | new_map | item.map = new_map ;

	// const cell = layer.data[i] - 1
	// const g = Math.floor(cell / tile.width)
	// const r = cell - g * tile.width
	// const b = 255
	// const a = 255
	// layerArray.push(r)
	// layerArray.push(g)
	// layerArray.push(b)
	// layerArray.push(a)
}

pub fn draw(ctx: &RenderContext, item: &View2D) {
	ctx.gl.use_program(Some(&item.program));

	ctx.gl.uniform_matrix4fv_with_f32_array(Some(&item.u_transform), false, &item.transform_matrix);

	ctx.gl.bind_buffer( GL::ARRAY_BUFFER, Some(&item.vertices_buf));
	ctx.gl.vertex_attrib_pointer_with_i32(item.a_position, 2, GL::FLOAT, false, 0, 0);
	ctx.gl.enable_vertex_attrib_array(item.a_position);
	ctx.gl.bind_buffer( GL::ARRAY_BUFFER, None);

	let u_tile_size =  ctx.gl.get_uniform_location(&item.program, "uTileSize").unwrap();
	ctx.gl.uniform2f(Some(&u_tile_size), 16., 16.);

	let u_resolution =  ctx.gl.get_uniform_location(&item.program, "uResolution").unwrap();
	ctx.gl.uniform2f(Some(&u_resolution), ctx.width as f32, ctx.height as f32);

	let u_map_size =  ctx.gl.get_uniform_location(&item.program, "uMapSize").unwrap();
	ctx.gl.uniform2f(Some(&u_map_size), item.map.width as f32,  item.map.height as f32);

	ctx.gl.active_texture(GL::TEXTURE0);
	ctx.gl.bind_texture(GL::TEXTURE_2D, Some(&item.map_texture));
	ctx.gl.uniform1i(Some(&item.u_image0), 0);

	ctx.gl.active_texture(GL::TEXTURE1);
	ctx.gl.bind_texture(GL::TEXTURE_2D, Some(&item.texture));
	ctx.gl.uniform1i(Some(&item.u_image1), 1);

	ctx.gl.draw_arrays(GL::TRIANGLES, 0, 6);

	ctx.gl.active_texture(GL::TEXTURE0);
	ctx.gl.bind_texture(GL::TEXTURE_2D, None);

	ctx.gl.active_texture(GL::TEXTURE1);
	ctx.gl.bind_texture(GL::TEXTURE_2D, None);
}
