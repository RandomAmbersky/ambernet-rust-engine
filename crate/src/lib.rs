mod mycore;

extern crate wasm_bindgen;
extern crate web_sys;
extern crate console_error_panic_hook;

use std::sync::Arc;
use wasm_bindgen::prelude::*;

use mycore::LogLevel;
use mycore::{Logger, Render};

fn set_panic_hook() {
#[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

const DEFAULT_LOG_LEVEL: LogLevel = LogLevel::Info;

#[wasm_bindgen]
pub struct AmberApi {
    logger: Arc<Logger>,
    render: Arc<Render>
}

#[wasm_bindgen]
impl AmberApi {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        set_panic_hook();
        let logger = mycore::new_logger(DEFAULT_LOG_LEVEL);
        let render = mycore::new_render(&logger);
        Self {
            logger,
            render
        }
    }
    pub fn update(&self, time: f32) -> Result<(), JsValue> {
        let mess = format!("update: {}", time);
        // self.logger.info(&mess);
        Ok(())
    }
    pub fn resize(&mut self, width: i32, height: i32) -> Result<(), JsValue> {
        let mess = format!("resize {} x {} ", width, height);
        self.logger.info(&mess);
        Ok(())
    }
    pub fn render(&self) -> Result<(), JsValue> {
        let mess = format!("render...");
        // self.logger.info(&mess);
        self.render.draw();
        Ok(())
    }
}
