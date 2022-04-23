pub use web_sys::WebGlRenderingContext as GL;
use web_sys::{WebGlProgram, WebGlShader};

pub fn link_program (
    gl: &GL,
    vert: &str,
    frag: &str
) -> Result<WebGlProgram, String> {
    let prog = gl
        .create_program()
        .ok_or_else(||String::from("Error create program"))?;
    let vert_shader = compile_shader(
        &gl,
        GL::VERTEX_SHADER,
        vert)
        .unwrap();
    let frag_shader = compile_shader(
        &gl,
        GL::FRAGMENT_SHADER,
        frag)
        .unwrap();
    gl.attach_shader(&prog, &vert_shader);
    gl.attach_shader(&prog, &frag_shader);
    gl.link_program(&prog);

    gl.delete_shader(Some(&vert_shader));
    gl.delete_shader(Some(&frag_shader));

    if gl.get_program_parameter(&prog, GL::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(prog)
    } else {
        Err(
            gl.get_program_info_log(&prog)
                .unwrap_or_else(||String::from("Unable to get program info log"))
        )
    }
}

fn compile_shader (
    gl: &GL,
    shader_type: u32,
    src: &str
) -> Result<WebGlShader, String> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(||String::from("Error create shader"))?;
    gl.shader_source(&shader, src);
    gl.compile_shader(&shader);

    if gl.get_shader_parameter(&shader, GL::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false) {
        Ok(shader)
    } else {
        Err(
            gl.get_shader_info_log(&shader)
                .unwrap_or_else(||String::from("Unable to get shader info log"))
        )
    }
}
