mod utils;
mod shaders;
mod buffers;

use web_sys::{WebGlBuffer, WebGlProgram};
use utils::GL as GL;

pub struct RenderContext {
	pub gl: GL,
}

pub fn resize(ctx: &RenderContext, _width: f32, _height: f32) {
	ctx.gl.enable(GL::BLEND);
	ctx.gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
	ctx.gl.clear_color(1.0, 1.0, 1.0, 1.0); //RGBA
	ctx.gl.clear_depth(1.0);
}

pub	fn draw(ctx: &RenderContext) {
	ctx.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT );
}

pub fn init_context() -> RenderContext {
		let gl = utils::get_webgl_context().unwrap();
		RenderContext {
			gl
		}
}

pub fn link_program (ctx: &RenderContext, vert: &str, frag: &str) -> Result<WebGlProgram, String> {
	shaders::link_program(&ctx.gl, vert, frag)
}

pub fn load_buffer(ctx: &RenderContext, buf: &[f64]) -> WebGlBuffer {
	buffers::load_buffer(&ctx.gl, buf)
}
