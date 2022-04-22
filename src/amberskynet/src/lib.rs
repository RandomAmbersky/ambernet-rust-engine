mod utils;

use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use amberskynet_logger_web::LoggerWeb;

#[wasm_bindgen]
pub struct AmberSkyNetClient {
    logger: LoggerWeb
}

impl Default for AmberSkyNetClient {
    fn default() -> Self {
        Self {
            logger: LoggerWeb {}
        }
    }
}

#[wasm_bindgen]
impl AmberSkyNetClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        utils::set_panic_hook();
        AmberSkyNetClient::default()
    }
    pub fn update(&self, _time: f32) -> Result<(), JsValue>{
        let _mess = format!("engine update: {}", _time);
        // self.logger.log(&_mess);
        Ok(())
    }
    pub fn resize(&mut self, _width: f32, _height: f32) -> Result<(), JsValue> {
        let _mess = format!("engine resize: {} x {}", _width, _height);
        self.logger.log(&_mess);
        Ok(())
    }

    pub fn render(&mut self) -> Result<(), JsValue> {
        let _mess = "engine render".to_string();
        // self.logger.log(&_mess);
        Ok(())
    }
}
