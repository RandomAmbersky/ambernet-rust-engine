mod utils;

use utils::GL as GL;

pub struct RenderContext {
	gl: GL,
}

pub fn resize(ctx: &RenderContext, _width: f32, _height: f32) {
	ctx.gl.enable(GL::BLEND);
	ctx.gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
	ctx.gl.clear_color(1.0, 0.0, 1.0, 1.0); //RGBA
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
