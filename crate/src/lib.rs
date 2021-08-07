extern crate wasm_bindgen;
extern crate web_sys;

// use glyph_brush::{FontId, GlyphBrush};

mod amberskynet;

use wasm_bindgen::prelude::*;
use amberskynet::EngineWebGl;
use amberskynet::api::AmberNetApi;
use crate::amberskynet::api::{RenderApi, LoggerApi};
use crate::amberskynet::set_panic_hook;

#[wasm_bindgen]
pub struct AmberSkyNetClient {
    a: EngineWebGl
}

#[wasm_bindgen]
impl AmberSkyNetClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        set_panic_hook();
        let a = amberskynet::get_engine();
        Self { a }
    }
    pub fn update(&self, _time: f32) -> Result<(), JsValue>{
        let _mess = format!("engine update: {}", _time);
        // self.a.get_log().log(&mess);
        Ok(())
    }
    pub fn resize(&mut self, _width: f32, _height: f32) -> Result<(), JsValue> {
        self.a.render().resize(_width, _height);
        Ok(())
    }
    pub fn render(&mut self) -> Result<(), JsValue> {
        &self.a.render().draw();
        Ok(())
    }
    pub fn upload_render_program(&mut self, vert: &str, frag: &str) -> Result<(), JsValue> {
        let mesh_array = [
            -1.0, 1.0,
            1.0, -1.0,
            -1.0, -1.0,
            -1.0, 1.0,
            1.0, -1.0,
            1.0, 1.0];
        let gl = &mut self.a.get_render().get_gl();
        let prog = self.a.render().get_shaders().load_render_2d_program(gl, vert, frag, &mesh_array);
        let prog_box = Box::new(prog);
        self.a.render().get_shaders().upload_program(prog_box);
        Ok(())
    }
    pub fn upload_font(&mut self, data: Vec<u8>) -> Result<(), JsValue> {
        let mess = format!("upload_font: {} bytes", data.len());
        self.a.get_log().log(&mess);
        let _font = glyph_brush::ab_glyph::FontArc::try_from_vec(data).unwrap();
        // let font_id = GlyphBrush::add_font(font);
        // let font_id = glyph_brush.borrow_mut():add_font(font);
        // ab_glyph::FontArc::try_from_vec(data).unwrap();
        // let mess = format!("upload font: {}", font);
        // self.a.get_log().log(&mess);
        Ok(())
    }
}
