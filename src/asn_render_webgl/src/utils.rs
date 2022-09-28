use wasm_bindgen::{ JsCast };
use web_sys::window;
pub use web_sys::WebGlRenderingContext as GL;

pub fn get_webgl_context () -> Result<GL, String> {

    let window: web_sys::Window = match window() {
        None => return Err(String::from("Window not found.")),
        Some(t) => t
    };

    let document: web_sys::Document = match window.document() {
        None => return Err(String::from("Document not found.")),
        Some(t) => t
    };

    let element: web_sys::Element = match document.get_element_by_id("canvasGL") {
        None => return Err(String::from("CanvasElement not found.")),
        Some(t) => t
    };

    let canvas: web_sys::HtmlCanvasElement = match element.dyn_into() {
        Err(_) => return Err(String::from("Element as HtmlCanvasElement error")),
        Ok(t) => t
    };

    let gl_object = match canvas.get_context("webgl") {
        Err(_) => return Err(String::from("HtmlCanvasElement as webgl error")),
        Ok(t) => match t {
            None => return Err(String::from("webgl context not found.")),
            Some(t) => t
        }
    };

    let gl_context:GL = match gl_object.dyn_into() {
        Err(_) => return Err(String::from("webgl to GL error")),
        Ok(t) => t
    };

    Ok(gl_context)
}
