mod utils;
mod test_2d;

use web_sys::WebGlRenderingContext as GL;
pub use test_2d::Test2D;

pub struct RenderContext {
    gl: GL,
}

pub fn get_render_ctx () -> RenderContext {
    RenderContext {
        gl: utils::get_webgl_context().unwrap()
    }
}

pub fn resize(ctx: &RenderContext, _width: i32, _height: i32) {
    ctx.gl.enable(GL::BLEND);
    ctx.gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
    ctx.gl.clear_color(0.0, 0.0, 0.0, 1.0); //RGBA
    ctx.gl.clear_depth(1.0);
    ctx.gl.viewport(0, 0, _width, _height);
}

pub fn clear(ctx: &RenderContext) {
    ctx.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT );
    // if ctx.curr_program_id.is_nil() {
    //     return;
    // }
    // let prog = ctx.programs.get(&ctx.curr_program_id).unwrap();
    // let mess = format!("draw program: {}", &ctx.curr_program_id);
    // log(&mess);
    // prog.render(&ctx.gl);
}

pub fn load_2d_program(ctx: &RenderContext, vert: &str, frag: &str, mesh: &[f64]) -> Test2D {
    let program = utils::link_program(&ctx.gl, vert, frag).unwrap();
    let buf = utils::load_buffer(&ctx.gl, mesh);
    Test2D {
        u_color: ctx.gl.get_uniform_location(&program, "uColor").unwrap(),
        u_opacity: ctx.gl.get_uniform_location(&program, "uOpacity").unwrap(),
        u_transform: ctx.gl.get_uniform_location(&program, "uTransform").unwrap(),
        buffer: buf,
        program,
    }
}
