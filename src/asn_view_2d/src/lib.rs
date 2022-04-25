mod shaders;
mod buffer;

use web_sys::WebGlRenderingContext as GL;
use web_sys::{WebGlBuffer, WebGlProgram};
use asn_render_webgl::RenderContext;

pub struct View2D {
	program: WebGlProgram,
	buffer: WebGlBuffer,
	w: i32,
	h: i32
}

pub fn new_item (ctx: &RenderContext, w: i32, h: i32) -> View2D {
	let program = asn_render_webgl::link_program(ctx, shaders::VERT_SHADER, shaders::FRAG_SHADER).unwrap();
	let buffer = asn_render_webgl::load_buffer(ctx, &buffer::SIMPLE_BUF);
	View2D {
		w, h,
		program,
		buffer
	}
}

pub fn draw (ctx: &RenderContext, item: &View2D) {
	let gl = &ctx.gl;
	gl.use_program(Some(&item.program));
	gl.bind_buffer(GL::ARRAY_BUFFER, Some(&item.buffer));
}
