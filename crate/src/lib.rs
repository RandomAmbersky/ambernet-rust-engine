extern crate wasm_bindgen;
extern crate web_sys;
extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;

mod amberskynet;

// use glyph_brush;
use amberskynet::render;
use amberskynet::AmberNetEngine;
use amberskynet::render::RenderContext;
use amberskynet::render::Test2D;
use amberskynet::render::BinaryFont;
use amberskynet::render::Texture;

struct Screen {
    w: i32,
    h: i32
}

fn set_panic_hook() {
#[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct AmberApi {
    ctx: AmberNetEngine,
    render_ctx: RenderContext,
    scr: Screen,
    prog: Option<Test2D>,
    font: Option<BinaryFont>,
    texture: Option<Texture>
}

#[wasm_bindgen]
impl AmberApi {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        set_panic_hook();
        let scr = Screen { w: 0, h: 0};
        let ctx = amberskynet::get_engine();
        let render_ctx = render::get_render_ctx();
        Self {
            ctx,
            render_ctx,
            scr,
            prog: None,
            font: None,
            texture: None
        }
    }
    pub fn update(&self, _time: f32) -> Result<(), JsValue> {
        // let mess = format!("engine update: {}", _time);
        // amberskynet::log(&mess);
        Ok(())
    }
    pub fn resize(&mut self, _width: i32, _height: i32) -> Result<(), JsValue> {
        self.scr.h = _height;
        self.scr.w = _width;
        render::resize(&self.render_ctx, self.scr.w, self.scr.h);
        Ok(())
    }
    pub fn render(&self) -> Result<(), JsValue> {
        render::clear(&self.render_ctx);
        self.prog.as_ref().unwrap().render(&self.render_ctx);
        Ok(())
    }
    pub fn upload_render_program(&mut self, vert: &str, frag: &str) -> Result<(), JsValue> {
        let mess1 = format!("vert: {}", vert);
        amberskynet::log(&mess1);

        let mess2 = format!("frag: {}", frag);
        amberskynet::log(&mess2);

        let mesh_array = [
            -1.0, 1.0,
            1.0, -1.0,
            -1.0, -1.0,
            -1.0, 1.0,
            1.0, -1.0,
            1.0, 1.0];
        self.prog = Some(render::load_2d_program(&self.render_ctx, vert, frag, &mesh_array));
        Ok(())
    }
    pub fn upload_font(&mut self, _data: Vec<u8>) -> Result<(), JsValue> {
        panic!("Implement me!");
        // let mess = format!("upload_font: {} bytes", data.len());
        // amberskynet::log(&mess);
        // let _font = glyph_brush::ab_glyph::FontArc::try_from_vec(data).unwrap();
        // Ok(())
    }
    pub fn upload_binary_font(&mut self, data: Vec<u8>) -> Result<(), JsValue> {
        self.font = Some(render::upload_binary_font(&self.render_ctx, data));
        Ok(())
    }
    pub fn upload_texture_raw(&mut self, data: Vec<u8>) -> Result<(), JsValue> {
        self.texture = Some(render::upload_texture(&self.render_ctx, &*data));
        Ok(())
    }
}
