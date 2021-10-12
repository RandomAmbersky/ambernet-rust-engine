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

pub fn resize(gl: &GL, _width: i32, _height: i32) {
    // gl.enable(GL::BLEND);
    // gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
    // gl.clear_color(0.0, 0.0, 0.0, 1.0); //RGBA
    // gl.clear_depth(1.0);
    gl.viewport(0, 0, _width, _height);
}

pub fn clear(gl: &GL) {
    gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn load_vertex_buffer(gl: &GL, buf: &[f32]) -> WebGlBuffer {
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
    gl.bind_buffer(GL::ARRAY_BUFFER, None);
    buffer
}

#[allow(dead_code)]
pub fn load_index_buffer(gl: &GL, indices: &[u16]) -> WebGlBuffer {
    let memory_buffer = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()
        .unwrap()
        .buffer();
    let indices_location: u32 = indices.as_ptr() as u32 / 2;
    let indices_array = js_sys::Uint16Array::new(&memory_buffer).subarray(
        indices_location,
        indices_location + indices.len() as u32
    );

    let indices_buffer = gl.create_buffer()
        .ok_or_else(||String::from("Failed to create buffer"))
        .unwrap();
    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&indices_buffer));
    gl.buffer_data_with_array_buffer_view(
        GL::ELEMENT_ARRAY_BUFFER,
        &indices_array,
        GL::STATIC_DRAW,
    );
    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, None);
    indices_buffer
}
