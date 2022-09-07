mod utils;
mod texture_data;

use amberskynet_logger_web::LoggerWeb;
use web_sys::{WebGlBuffer, WebGlProgram, WebGlTexture, WebGlUniformLocation};
use asn_render_webgl::{ RenderContext };
use asn_images::{decode_texture};

use web_sys::WebGlRenderingContext as GL;
use asn_array_2d::Array2D;
use asn_core::Rect2D;

struct RenderData {

}

pub struct View2D {
	render_data: RenderData,
	texture: WebGlTexture,
	map_texture: WebGlTexture,
	program: WebGlProgram,
	a_position: u32,
	u_transform: WebGlUniformLocation,
	transform_matrix: [f32;16],
	u_image0: WebGlUniformLocation,
	u_image1: WebGlUniformLocation,
	vertices_buf: WebGlBuffer,
	view: Array2D,
	texture_data: Array2D,
	u_map_size: WebGlUniformLocation,
	u_sheet_size: WebGlUniformLocation,
	u_tile_size: WebGlUniformLocation,
}

pub fn new_item (ctx: &RenderContext) -> Result<View2D, String> {

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

	let u_tile_size = match ctx.gl.get_uniform_location(&program, "utileSize") {
		None => {
			return Err(String::from("uSheetSize not found"))
		},
		Some(t) => t
	};

	let render_data = RenderData {};

	let view2d = View2D {
		render_data,
		program,
		a_position,
		texture,
		map_texture,
		u_transform,
		transform_matrix,
		u_image0,
		u_image1,
		vertices_buf,
		view: Array2D::default(),
		texture_data: Array2D::default(),
		u_map_size,
		u_sheet_size,
		u_tile_size
	};
	Ok(view2d)
}

pub fn set_tiles (ctx: &RenderContext, item: &View2D, buf: &[u8]) -> Result<(), String>{
	let tex = decode_texture(buf)?;

	asn_render_webgl::update_texture(ctx, &item.texture, &tex, false)?;

	ctx.gl.use_program(Some(&item.program));
	ctx.gl.uniform2f(Some(&item.u_sheet_size), tex.width as f32, tex.height as f32);
	ctx.gl.use_program(None);

	Ok(())
}

pub fn set_tile_size (ctx: &RenderContext, item: &mut View2D, width: u32, height: u32) -> Result<(), String> {
	ctx.gl.use_program(Some(&item.program));
	ctx.gl.uniform2f(Some(&item.u_tile_size), width as f32, height as f32);
	ctx.gl.use_program(None);
	Ok(())
}

pub fn set_view (item: &mut View2D, window: Rect2D, map: &[u8]) {
	item.view.width = window.width;
	item.view.height = window.height;
	let mut bytes: Vec<u8> = Vec::new();

	for cell in map.iter() {
		bytes.push(*cell);
	}

	item.view.bytes = bytes;

	let texture_data = texture_data::from_array2d(&item.view, 16);

	item.texture_data = texture_data;

	let mess = format!("Map set on {}, {}", item.view.width, item.view.height);
	LoggerWeb::log(&mess);
}

pub fn update_view (ctx: &RenderContext, item: &mut View2D) {
	asn_render_webgl::update_texture(ctx, &item.map_texture, &item.texture_data, false).expect("update_texture panic!");
	ctx.gl.use_program(Some(&item.program));
	ctx.gl.uniform2f(Some(&item.u_map_size), item.texture_data.width as f32, item.texture_data.height as f32);
	ctx.gl.use_program(None);
}

pub fn draw(ctx: &RenderContext, item: &View2D) {
	ctx.gl.use_program(Some(&item.program));

	ctx.gl.uniform_matrix4fv_with_f32_array(Some(&item.u_transform), false, &item.transform_matrix);

	ctx.gl.bind_buffer( GL::ARRAY_BUFFER, Some(&item.vertices_buf));
	ctx.gl.vertex_attrib_pointer_with_i32(item.a_position, 2, GL::FLOAT, false, 0, 0);
	ctx.gl.enable_vertex_attrib_array(item.a_position);
	ctx.gl.bind_buffer( GL::ARRAY_BUFFER, None);

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

pub fn set_cell (item: &mut View2D, x: u32, y: u32, cell: u32) -> Result<(), String> {
	let index = get_index(item, x, y)?;

	let cell_u8 = cell as u8;

	item.view.bytes[index] = cell_u8;

	let cell_y = cell_u8 / 16;
	let cell_x = cell_u8 - cell_y * 16;

	// let mess = format!("set_cell on {}, {}", cell_y, cell_x);
	// LoggerWeb::log(&mess);

	let texture_index = get_texture_index(item, x, y)?;

	item.texture_data.bytes[texture_index] = cell_x as u8;
	item.texture_data.bytes[texture_index+1] = cell_y as u8;
	item.texture_data.bytes[texture_index+2] = 255;
	item.texture_data.bytes[texture_index+3] = 255;

	Ok(())
}

pub fn get_index (item: &mut View2D, x: u32, y: u32) -> Result<usize, String> {
	let index = (item.view.width * y + x ) as usize;

	if index >= item.view.bytes.len() {
		let mess = format!("Invalid index {} on map [{},{}]", index, x, y);
		return Err(mess)
	};

	Ok(index)
}

pub fn get_texture_index (item: &mut View2D, x: u32, y: u32) -> Result<usize, String> {
	let index = (item.texture_data.width * y * 4 + x * 4) as usize;

	if index >= item.texture_data.bytes.len() {
		let mess = format!("Invalid index {} on map [{},{}]", index, x, y);
		return Err(mess)
	};

	Ok(index)
}
