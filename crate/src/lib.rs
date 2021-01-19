extern crate wasm_bindgen;
extern crate web_sys;

mod utils;
mod amberskynet;
mod ambernet;

use wasm_bindgen::prelude::*;

use utils::set_panic_hook;

use amberskynet::get_engine;
use amberskynet::EngineWebGl;
use crate::amberskynet::api::{
    AmberNetApi,
    LoggerApi,
    RenderApi
};

use crate::ambernet::AmberNet;
use crate::ambernet::system::logger::SystemLog;
use crate::ambernet::system::render::SystemRender;

#[wasm_bindgen]
pub struct AmberSkyNet {
    engine: EngineWebGl,
    a: AmberNet
}

#[wasm_bindgen]
impl AmberSkyNet {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        set_panic_hook();
        let log = Box::new(SystemLog::new());
        let r = Box::new(SystemRender::new());
        let mut a = AmberNet::new();
        a
            .add_system_box(log)
            .add_system_box(r);
        Self {
            engine: get_engine(),
            a
        }
    }

    pub fn update(&self, _time: f32) {
        self.a.update(_time);
        let mess = format!("update {}", _time);
        self.engine.get_log().log(&mess);
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
