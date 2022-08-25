mod utils;
mod shaders;
mod buffers;
mod textures;

use web_sys::{WebGlBuffer, WebGlProgram, WebGlTexture, WebGlUniformLocation};
use asn_images::DecodedTexture;
use utils::GL as GL;

const ONE_BLUE_PIXEL: [u8; 4] = [0, 0, 255, 255];

pub struct RenderContext {
	pub gl: GL,
	pub width: i32,
	pub height: i32
}

pub fn resize(ctx: &mut RenderContext, width: i32, height: i32) {
	ctx.width = width;
	ctx.height= height;
	ctx.gl.enable(GL::BLEND);
	ctx.gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
	ctx.gl.clear_color(0., 0., 0., 1.0); //RGBA
	ctx.gl.clear_depth(1.0);
	ctx.gl.enable(GL::DEPTH_TEST);
	let margin_left = 10;
	let margin_right = 10;
	let margin_top = 10;
	let margin_bottom = 10;

	ctx.gl.viewport(margin_left, margin_bottom, width - margin_right - margin_left, height - margin_top - margin_bottom);
}

pub	fn draw(ctx: &RenderContext) {
	ctx.gl.clear_color(0.5, 0.5, 0.5, 1.0);
	ctx.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT );
}

pub fn init_context() -> RenderContext {
		let gl = utils::get_webgl_context().unwrap();
		RenderContext {
			gl,
			width: 0,
			height: 0
		}
}

pub fn link_program (ctx: &RenderContext, vert: &str, frag: &str) -> Result<WebGlProgram, String> {
	shaders::link_program(&ctx.gl, vert, frag)
}

pub fn load_buffer(ctx: &RenderContext, buf: &[f32]) -> WebGlBuffer {
	buffers::load_buffer(&ctx.gl, buf)
}

pub fn load_index_buffer(ctx: &RenderContext, buf: &[u16]) -> WebGlBuffer {
	buffers::load_index_buffer(&ctx.gl, buf)
}

pub fn update_texture(ctx: &RenderContext, texture: Option<&WebGlTexture>, tex: DecodedTexture, is_linear: bool) -> Result<(), String> {
	textures::update(&ctx.gl, texture, tex, is_linear)
}

pub fn upload_texture(ctx: &RenderContext, tex: DecodedTexture, is_linear: bool) -> Result<WebGlTexture, String> {
	textures::upload(&ctx.gl, tex, is_linear)
}

pub fn load_empty_texture(ctx: &RenderContext) -> Result<WebGlTexture, String> {
	// look at https://snoozetime.github.io/2019/12/19/webgl-texture.html
	let tex = DecodedTexture {
		width: 1,
		height: 1,
		bytes: ONE_BLUE_PIXEL.to_vec()
	};
	textures::upload(&ctx.gl, tex, false)
}
