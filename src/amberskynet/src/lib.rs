mod utils;

use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use amberskynet_logger_web::LoggerWeb;
use asn_render_webgl::RenderContext;

#[wasm_bindgen]
pub struct AmberSkyNetClient {
    logger: LoggerWeb,
    ctx: RenderContext
}

impl Default for AmberSkyNetClient {
    fn default() -> Self {
        let ctx = asn_render_webgl::init_context();
        Self {
            logger: LoggerWeb {},
            ctx,
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

    pub fn upload_tiles(&self, _data: Vec<u8>) -> Result<(), JsValue> {
        let mess = "engine upload_tiles".to_owned();
        self.logger.log(&mess);
        Ok(())
    }

    pub fn upload_map(&self, _data: Vec<u8>) -> Result<(), JsValue> {
        let mess = "engine upload_map".to_owned();
        self.logger.log(&mess);
        Ok(())
    }

    pub fn update(&self, _time: f32) -> Result<(), JsValue> {
        let _mess = format!("engine update: {}", _time);
        // self.logger.log(&_mess);
        Ok(())
    }

    pub fn resize(&self, _width: f32, _height: f32) -> Result<(), JsValue> {
        let mess = format!("engine resize: {} x {}", _width, _height);
        self.logger.log(&mess);
        asn_render_webgl::resize(&self.ctx, _width, _height);
        Ok(())
    }

    pub fn render(&self) -> Result<(), JsValue> {
        asn_render_webgl::draw(&self.ctx);
        Ok(())
    }
}
