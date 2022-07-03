mod utils;
mod shaders;
mod buffers;
mod textures;

use web_sys::{WebGlBuffer, WebGlProgram, WebGlTexture};
use utils::GL as GL;

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

pub fn link_program (ctx: &RenderContext, vert: &str, frag: &str) -> WebGlProgram {
	match shaders::link_program(&ctx.gl, vert, frag) {
		Ok(t) => t,
		Err(why) => {
			panic!("link_program error: {}", why)
		},
	}
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
