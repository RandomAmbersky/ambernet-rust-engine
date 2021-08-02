extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;


mod amberskynet;
use amberskynet::render;
use amberskynet::AmberNetEngine as A;

#[wasm_bindgen]
pub struct AmberApi {
    ctx: A
}

#[wasm_bindgen]
impl AmberApi {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // set_panic_hook();
        let ctx = amberskynet::get_engine();
        Self { ctx }
    }
    pub fn update(&self, _time: f32) -> Result<(), JsValue> {
        let mess = format!("engine update: {}", _time);
        amberskynet::log(&mess);
        Ok(())
    }
    pub fn resize(&self, _width: f32, _height: f32) -> Result<(), JsValue> {
        render::resize(&self.ctx.render_ctx, _width, _height);
        Ok(())
    }
    pub fn render(&self) -> Result<(), JsValue> {
        render::draw(&self.ctx.render_ctx);
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
        let prog = render::load_render_2d_program(&self.ctx.render_ctx, vert, frag, &mesh_array);
        let prog_box = Box::new(prog);
        render::upload_program(&mut self.ctx.render_ctx, prog_box);
        Ok(())
    }
}
