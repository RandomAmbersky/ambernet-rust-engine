use wasm_bindgen::JsValue;
use wasm_bindgen::JsCast;
use web_sys::window;
use web_sys::*;
pub use web_sys::WebGlRenderingContext as GL;
use js_sys::WebAssembly;

pub fn get_webgl_context () -> Result<GL, JsValue> {

    let window: web_sys::Window = match window() {
        None => return Err(JsValue::from("Window not found.")),
        Some(t) => t
    };

    let document: web_sys::Document = match window.document() {
        None => return Err(JsValue::from("Document not found.")),
        Some(t) => t
    };

    let element: web_sys::Element = match document.get_element_by_id("canvasGL") {
        None => return Err(JsValue::from("CanvasElement not found.")),
        Some(t) => t
    };

    let canvas: web_sys::HtmlCanvasElement = match element.dyn_into() {
        Err(e) => return Err(JsValue::from(e)),
        Ok(t) => t
    };

    let gl_object = match canvas.get_context("webgl") {
        Err(e) => return Err(JsValue::from(e)),
        Ok(t) => match t {
            None => return Err(JsValue::from("webgl context not found.")),
            Some(t) => t
        }
    };

    let gl_context:GL = match gl_object.dyn_into() {
        Err(e) => return Err(JsValue::from(e)),
        Ok(t) => t
    };

    Ok(gl_context)
}

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

pub fn compile_shader (
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

pub fn load_buffer(gl: &GL, buf: &[f64]) -> WebGlBuffer {
    let memory_buffer = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()
        .unwrap()
        .buffer();
    let vertices_location = buf.as_ptr() as u32 / 4;
    let vert_array = js_sys::Float32Array::new(&memory_buffer).subarray(
        vertices_location,
        vertices_location + buf.len() as u32,
    );

    let buffer = gl.create_buffer()
        .ok_or_else(||String::from("Failed to create buffer"))
        .unwrap();

    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
    gl.buffer_data_with_array_buffer_view(
        GL::ARRAY_BUFFER,
        &vert_array,
        GL::STATIC_DRAW,
    );
    buffer
}

pub fn translation_matrix(tx: f32, ty: f32, tz: f32) -> [f32; 16] {
    let mut return_var = [0.; 16];

    return_var[0] = 1.;
    return_var[5] = 1.;
    return_var[10] = 1.;
    return_var[15] = 1.;

    return_var[12] = tx;
    return_var[13] = ty;
    return_var[14] = tz;

    return_var
}

pub fn scaling_matrix(sx: f32, sy: f32, sz: f32) -> [f32; 16] {
    let mut return_var = [0.; 16];

    return_var[0] = sx;
    return_var[5] = sy;
    return_var[10] = sz;
    return_var[15] = 1.;

    return_var
}

pub fn mult_matrix_4(a: [f32; 16], b: [f32; 16]) -> [f32; 16] {
    let mut return_var = [0.; 16];

    return_var[0] = a[0] * b[0] + a[1] * b[4] + a[2] * b[8] + a[3] * b[12];
    return_var[1] = a[0] * b[1] + a[1] * b[5] + a[2] * b[9] + a[3] * b[13];
    return_var[2] = a[0] * b[2] + a[1] * b[6] + a[2] * b[10] + a[3] * b[14];
    return_var[3] = a[0] * b[3] + a[1] * b[7] + a[2] * b[11] + a[3] * b[15];

    return_var[4] = a[4] * b[0] + a[5] * b[4] + a[6] * b[8] + a[7] * b[12];
    return_var[5] = a[4] * b[1] + a[5] * b[5] + a[6] * b[9] + a[7] * b[13];
    return_var[6] = a[4] * b[2] + a[5] * b[6] + a[6] * b[10] + a[7] * b[14];
    return_var[7] = a[4] * b[3] + a[5] * b[7] + a[6] * b[11] + a[7] * b[15];

    return_var[8] = a[8] * b[0] + a[9] * b[4] + a[10] * b[8] + a[11] * b[12];
    return_var[9] = a[8] * b[1] + a[9] * b[5] + a[10] * b[9] + a[11] * b[13];
    return_var[10] = a[8] * b[2] + a[9] * b[6] + a[10] * b[10] + a[11] * b[14];
    return_var[11] = a[8] * b[3] + a[9] * b[7] + a[10] * b[11] + a[11] * b[15];

    return_var[12] = a[12] * b[0] + a[13] * b[4] + a[14] * b[8] + a[15] * b[12];
    return_var[13] = a[12] * b[1] + a[13] * b[5] + a[14] * b[9] + a[15] * b[13];
    return_var[14] = a[12] * b[2] + a[13] * b[6] + a[14] * b[10] + a[15] * b[14];
    return_var[15] = a[12] * b[3] + a[13] * b[7] + a[14] * b[11] + a[15] * b[15];

    return_var
}
