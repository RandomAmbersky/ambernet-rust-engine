mod utils;
mod test_2d;
mod binary_font;
mod texture;

use image::{GenericImageView};
use web_sys::WebGlRenderingContext as GL;
pub use test_2d::Test2D;
pub use binary_font::BinaryFont;
pub use texture::Texture;
use crate::amberskynet::Logger;

pub struct RenderContext<'a> {
    gl: GL,
    logger: &'a Logger
}

pub fn get_render_ctx () -> GL {
    return utils::get_webgl_context().unwrap()
}

// pub fn get_render_ctx (logger: &Logger) -> RenderContext {
//     RenderContext {
//         gl:
//         logger
//     }
// }

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

pub fn upload_binary_font(_ctx: &RenderContext, _data: Vec<u8>) -> BinaryFont {
    panic!("Implement me!");
}

pub fn upload_texture(ctx: &RenderContext, bytes: &[u8]) -> Texture {
    let img = match image::load_from_memory(bytes) {
        Ok(t) => t,
        Err(why) => {
            panic!("image::load_from_memory error: {}", why)
        }
    };
    let decode_bytes = img.to_rgba8().into_raw();
    let tex = match ctx.gl.create_texture() {
        Some(t) => t,
        None => {
            panic!("create_texture error")
        }
    };
    ctx.gl.bind_texture(GL::TEXTURE_2D, Some(tex.as_ref()));
    ctx.gl.pixel_storei(GL::UNPACK_FLIP_Y_WEBGL, 1);

    ctx.gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
    ctx.gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);
    ctx.gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::LINEAR as i32);
    ctx.gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::LINEAR as i32);
    // ctx.gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::NEAREST as i32);
    // ctx.gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::NEAREST as i32);

    // target: u32,
    // level: i32,
    // internalformat: i32,
    // width: i32,
    // height: i32,
    // border: i32,
    // format: u32,
    // type_: u32,
    // pixels: Option<&[u8]>,

    let level = 0;
    let internal_format = GL::RGBA;
    let border = 0;
    let src_format = GL::RGBA;
    let src_type = GL::UNSIGNED_BYTE;

    let load_result = match ctx.gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        GL::TEXTURE_2D,
        level,
        internal_format as i32,
        img.width() as i32,
        img.height() as i32,
        border,
        src_format,
        src_type,
        Some(&decode_bytes),
    ) {
        Ok(t) => t,
        Err(why) => {
            let err_str = match why.as_string() {
               Some(t) => t,
               None => panic!("ctx.gl.tex_image_2d as_string error")
            };
            panic!("ctx.gl.tex_image_2d error: {}", &err_str);
        }
    };

    Texture {
        texture: tex,
        width: img.width(),
        height: img.height()
    }
}
