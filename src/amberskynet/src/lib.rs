mod utils;

use asn_utils_js::say_hello;

use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use amberskynet_logger_web::LoggerWeb;
use asn_render_webgl::RenderContext;

use color_quad::{new_item as new_color_quad, ColorQuad};
use textured_quad::{new_item as new_textured_quad, TexturedQuad};
use triangle::{new_item as new_triangle, Triangle};

#[wasm_bindgen]
pub struct AmberSkyNetClient {
    logger: LoggerWeb,
    ctx: RenderContext,
    triangle: Triangle,
    color_quad: ColorQuad,
    textured_quad: TexturedQuad
}

impl Default for AmberSkyNetClient {
    fn default() -> Self {
        let ctx = asn_render_webgl::init_context();
        let triangle = new_triangle(&ctx);
        let color_quad = new_color_quad(&ctx);
        let textured_quad = new_textured_quad(&ctx);
        Self {
            logger: LoggerWeb {},
            ctx,
            triangle,
            color_quad,
            textured_quad
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
        // say_hello();
        // self.logger.log(&_mess);
        Ok(())
    }

    pub fn resize(&mut self, width: i32, height: i32) -> Result<(), JsValue> {
        let mess = format!("engine resize: {} x {}", width, height);
        self.logger.log(&mess);
        asn_render_webgl::resize(&mut self.ctx, width, height);
        Ok(())
    }

    pub fn render(&self) -> Result<(), JsValue> {
        // asn_render_webgl::draw(&self.ctx);
        // triangle::draw(&self.ctx, &self.triangle);
        color_quad::draw(&self.ctx, &self.color_quad);
        // textured_quad::draw(&self.ctx, &self.textured_quad);
        Ok(())
    }
}

/*
fn make_test_item (ctx: &RenderContext) -> Test2D {
    let frag = "
        precision mediump float;

        uniform vec4 uColor;
        uniform float uOpacity;

        void main() {
            gl_FragColor = vec4(uColor.r, uColor.g, uColor.b, uColor.a * uOpacity);
        }";

    let vert = r#"
        attribute vec4 aPosition;
        uniform mat4 uTransform;

        void main() {
            gl_Position = uTransform * aPosition;
        }"#;

    let buf = [
        0., 1.,
        0., 0.,
        1., 1.,
        1., 1.,
        0., 0.,
        1., 0.
    ];

    new_test_2d(ctx, vert, frag, &buf)
}
fn make_view_2d (ctx: &RenderContext) -> View2D {
    new_view_2d(ctx, 10,10)
}
*/