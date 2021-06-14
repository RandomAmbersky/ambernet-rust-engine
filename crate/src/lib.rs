extern crate wasm_bindgen;
extern crate web_sys;

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
        let mess = format!("engine update: {}", _time);
        self.a.get_log().log(&mess);
        Ok(())
    }
    pub fn resize(&self, _width: f32, _height: f32) -> Result<(), JsValue> {
        self.a.get_render().resize(_width, _height);
        Ok(())
    }
    pub fn render(&self) -> Result<(), JsValue> {
        self.a.get_render().draw();
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
        let prog = self.a.get_render().load_render_2d_program(vert, frag, &mesh_array);
        let prog_box = Box::new(prog);
        self.a.upload_render_program(prog_box);
        Ok(())
    }
}
