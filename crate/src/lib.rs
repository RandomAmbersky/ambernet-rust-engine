use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;

mod mycore;
mod render;
mod color_quad;

use mycore::{Logger};
use mycore::LogLevel;
use render::Render;
use color_quad::ColorQuad;

extern crate wasm_bindgen;
extern crate web_sys;

const DEFAULT_LOG_LEVEL: LogLevel = LogLevel::Info;

#[wasm_bindgen]
pub struct AmberApi {
    logger: Arc<Mutex<Logger>>,
    render: Arc<Mutex<Render>>,
    elem: ColorQuad
}

#[wasm_bindgen]
impl AmberApi {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        mycore::set_panic_hook();
        let logger = mycore::new_logger(DEFAULT_LOG_LEVEL);
        let render = render::new_render(&logger);
        let elem = color_quad::new(&render);
        Self {
            logger,
            render,
            elem
        }
    }
    pub fn update(&self, time: f32) -> Result<(), JsValue> {
        let mess = format!("update: {}", time);
        self.logger.lock().unwrap().trace(&mess);
        Ok(())
    }
    pub fn resize(&mut self, width: i32, height: i32) -> Result<(), JsValue> {
        let mess = format!("resize {} x {} ", width, height);
        self.logger.lock().unwrap().trace(&mess);
        self.render.lock().unwrap().resize(width, height);
        Ok(())
    }
    pub fn render(&self) -> Result<(), JsValue> {
        // self.render.lock().unwrap().draw();
        let render = &*self.render.lock().unwrap();
        // render.draw();
        self.elem.draw(&render);
        let mess = format!("render...");
        self.logger.lock().unwrap().trace(&mess);
        Ok(())
    }
}
