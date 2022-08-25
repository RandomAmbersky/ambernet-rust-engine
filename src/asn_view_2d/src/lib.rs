mod utils;
mod map;

use amberskynet_logger_web::LoggerWeb;
use web_sys::{WebGlBuffer, WebGlProgram, WebGlTexture, WebGlUniformLocation};
use asn_render_webgl::{ RenderContext };
use asn_images::{decode_texture, DecodedTexture};

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
	map: Map,
	u_map_size: WebGlUniformLocation,
	u_sheet_size: WebGlUniformLocation,
}

pub fn new_item (
	ctx: &RenderContext, _w_cells: i32, _h_cells: i32
) -> Result<View2D, String> {

	let vertices_buf = asn_render_webgl::load_buffer(ctx, &utils::VERTICES);

	let program = match asn_render_webgl::link_program(ctx, utils::VERTEX_SHADER, utils::FRAG_SHADER) {
		Ok(t) => t,
		Err(str) => {
			return Err(str)
		}
	};

	let texture = asn_render_webgl::load_empty_texture(ctx)?;
	let map_texture = asn_render_webgl::load_empty_texture(ctx)?;

	let a_position = ctx.gl.get_attrib_location(&program, "aPosition") as u32;

	let u_transform =  ctx.gl.get_uniform_location(&program, "uTransform").unwrap();

	let scale_matrix = asn_math::scaling_matrix(
		2.,
		2.,
		1.
	);
	let trans_matrix = asn_math::translation_matrix(
		-1.,
		-1.,
		0.
	);

	let transform_matrix = asn_math::mult_matrix_4(scale_matrix, trans_matrix);

	let u_image0 = match ctx.gl.get_uniform_location(&program, "uTileMap") {
		Some(t) => t,
		None => {
			return Err(String::from("uTileMap not found"))
		}
	};

	let u_image1 =  match ctx.gl.get_uniform_location(&program, "uTileSheet") {
		Some(t) => t,
		None => {
			return Err(String::from("uTileSheet not found"))
		}
	};

	let u_map_size = match ctx.gl.get_uniform_location(&program, "uMapSize") {
		None => {
			return Err(String::from("uMapSize not found"))
		},
		Some(t) => t
	};

	let u_sheet_size = match ctx.gl.get_uniform_location(&program, "uSheetSize") {
		None => {
			return Err(String::from("uSheetSize not found"))
		},
		Some(t) => t
	};

	let view2d = View2D {
		program,
		a_position,
		texture,
		map_texture,
		u_transform,
		transform_matrix,
		u_image0,
		u_image1,
		vertices_buf,
		map: Map::default(),
		u_map_size,
		u_sheet_size
	};
	Ok(view2d)
}

pub fn set_tiles (ctx: &RenderContext, item: &View2D, buf: &[u8]) {
	let tex = decode_texture(buf).expect("decode texture panic!");

	asn_render_webgl::update_texture(ctx, &item.texture, &tex, false).expect("update_texture panic!");

	ctx.gl.use_program(Some(&item.program));
	ctx.gl.uniform2f(Some(&item.u_sheet_size), tex.width as f32, tex.height as f32);
	ctx.gl.use_program(None);
}

pub fn set_map (ctx: &RenderContext, item: &mut View2D, width: u32, height: u32, buf: &[u8]) {
	let mut map_texture: Vec<u8> = Vec::new();

	for cell in buf.iter() {
		let index = cell - 1;
		let y = index / 16;
		let x = index - y * 16;

		let index_check = y * 16 + x + 1;

		// assert_eq!(index, index_check);

		let mess = format!("cell {:?} [{:?}, {:?}] {:?}", cell, y, x, index_check);
		LoggerWeb::log(&mess);
		map_texture.push(x);
		map_texture.push(y);
		map_texture.push(255);
		map_texture.push(255);
	}

	let tex: DecodedTexture = DecodedTexture {
		width: width as i32,
		height: height as i32,
		bytes: map_texture
	};

	asn_render_webgl::update_texture(ctx, &item.map_texture, &tex, false).expect("update_texture panic!");

	ctx.gl.use_program(Some(&item.program));
	ctx.gl.uniform2f(Some(&item.u_map_size), width as f32, height as f32);
	ctx.gl.use_program(None);

	item.map.width = width as i32;
	item.map.height = height as i32;

	// move | new_map | item.map = new_map ;
}

pub fn draw(ctx: &RenderContext, item: &View2D) {
	ctx.gl.use_program(Some(&item.program));

	ctx.gl.uniform_matrix4fv_with_f32_array(Some(&item.u_transform), false, &item.transform_matrix);

	ctx.gl.bind_buffer( GL::ARRAY_BUFFER, Some(&item.vertices_buf));
	ctx.gl.vertex_attrib_pointer_with_i32(item.a_position, 2, GL::FLOAT, false, 0, 0);
	ctx.gl.enable_vertex_attrib_array(item.a_position);
	ctx.gl.bind_buffer( GL::ARRAY_BUFFER, None);

	// let u_tile_size =  ctx.gl.get_uniform_location(&item.program, "uTileSize").unwrap();
	// ctx.gl.uniform2f(Some(&u_tile_size), 16., 16.);
	//
	// let u_resolution =  ctx.gl.get_uniform_location(&item.program, "uResolution").unwrap();
	// ctx.gl.uniform2f(Some(&u_resolution), ctx.width as f32, ctx.height as f32);

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
