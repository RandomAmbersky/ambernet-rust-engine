mod utils;
mod shaders;
mod buffers;
mod textures;

use amberskynet_logger_web::LoggerWeb;
use web_sys::{WebGlBuffer, WebGlProgram, WebGlTexture, WebGlUniformLocation};
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
	ctx.gl.viewport(0, 0, width, height);
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

pub fn load_texture(ctx: &RenderContext, buf: &[u8]) -> WebGlTexture {
	textures::upload_texture(&ctx.gl, buf)
}

pub fn update_texture(ctx: &RenderContext, texture: Option<&WebGlTexture>, buf: &[u8]) {
	textures::update_texture(&ctx.gl, texture, buf)
}

pub fn update_raw_texture(ctx: &RenderContext, texture: Option<&WebGlTexture>, width: i32, height: i32, buf: &[u8]) {
	textures::update_raw_texture(&ctx.gl, texture, buf, width, height);
}

pub fn upload_raw_texture(ctx: &RenderContext, width: i32, height: i32, buf: &[u8]) -> WebGlTexture {
	textures::upload_raw_texture(&ctx.gl, &buf.to_vec(), width, height)
}

pub fn load_empty_texture(ctx: &RenderContext) -> WebGlTexture {
	// look at https://snoozetime.github.io/2019/12/19/webgl-texture.html
	textures::upload_raw_texture(&ctx.gl, &ONE_BLUE_PIXEL.to_vec(), 1, 1)
}
