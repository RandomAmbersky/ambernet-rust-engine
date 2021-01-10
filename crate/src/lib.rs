extern crate wasm_bindgen;
extern crate web_sys;

mod utils;
mod amberskynet;

use wasm_bindgen::prelude::*;

use utils::set_panic_hook;

use amberskynet::get_engine;
use amberskynet::EngineWebGl;
use crate::amberskynet::api::{
    AmberNetApi,
    LoggerApi,
    RenderApi
};

#[wasm_bindgen]
struct AmberSkyNet {
    engine: EngineWebGl
}

#[wasm_bindgen]
impl AmberSkyNet {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        set_panic_hook();
        Self {
            engine: get_engine()
        }
    }

    pub fn update(&self, _time: f32) {
        // let mess = format!("update {}", _time);
        // self.engine.get_log().log(&mess)
    }

    pub fn resize(&self, _width: f32, _height: f32) {
        self.engine.get_render().resize(_width, _height);
        let mess = format!("resize {} x {}", _width, _height);
        self.engine.get_log().log(&mess);
    }

    pub fn render(&self) {
        self.engine.get_render().draw();
        // self.engine.get_log().log("render");
    }
}
