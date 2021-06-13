extern crate wasm_bindgen;
extern crate web_sys;

mod amberskynet;

use wasm_bindgen::prelude::*;
use amberskynet::EngineWebGl;
use amberskynet::api::AmberNetApi;
use crate::amberskynet::api::{RenderApi, LoggerApi};
use crate::amberskynet::set_panic_hook;

#[wasm_bindgen]
pub struct AmberSkyNet {
    a: EngineWebGl
}

#[wasm_bindgen]
impl AmberSkyNet {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        set_panic_hook();
        let a = amberskynet::get_engine();
        Self { a }
    }
    pub fn update(&self, _time: f32) {
        let mess = format!("engine update: {}", _time);
        self.a.get_log().log(&mess);
    }
    pub fn resize(&self, _width: f32, _height: f32) {
        self.a.get_render().resize(_width, _height)
    }
    pub fn render(&self) {
        self.a.get_render().draw()
    }
}
