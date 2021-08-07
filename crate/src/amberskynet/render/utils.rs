use wasm_bindgen::JsValue;
use wasm_bindgen::JsCast;
use web_sys::window;
// use js_sys::WebAssembly;
// use web_sys::*;
pub use web_sys::WebGlRenderingContext as GL;

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
