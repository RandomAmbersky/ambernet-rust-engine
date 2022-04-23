mod utils;

use std::borrow::Borrow;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use amberskynet_logger_web::LoggerWeb;
use asn_render_webgl::RenderContext;
use asn_test_2d::{new_item, Test2D};

#[wasm_bindgen]
pub struct AmberSkyNetClient {
    logger: LoggerWeb,
    ctx: RenderContext,
    item: Test2D
}

fn make_test_item (ctx: &RenderContext) -> Test2D {
    let frag = "
        precision mediump float;

        uniform vec4 uColor;
        uniform float uOpacity;

        void main() {
            gl_FragColor = vec4(uColor.r, uColor.g, uColor.b, uColor.a * uOpacity);
        }";

    let vert = "
        attribute vec4 aPosition;
        uniform mat4 uTransform;

        void main() {
            gl_Position = uTransform * aPosition;
        }";

    let buf = [
        -1.0, 1.0,
        1.0, -1.0,
        -1.0, -1.0,
        -1.0, 1.0,
        1.0, -1.0,
        1.0, 1.0];

    new_item(ctx, vert, frag, &buf)
}


impl Default for AmberSkyNetClient {
    fn default() -> Self {
        let ctx = asn_render_webgl::init_context();
        let item = make_test_item(&ctx);
        Self {
            logger: LoggerWeb {},
            ctx,
            item
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
        asn_test_2d::render(&self.ctx, &self.item);
        Ok(())
    }
}
