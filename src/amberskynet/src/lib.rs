mod utils;
mod game_utils;
mod logic;
mod key_utils;

use specs::World;
use asn_view_2d::{new_item as new_view_2d, View2D};

use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use amberskynet_logger_web::LoggerWeb;
use asn_render_webgl::RenderContext;

use color_quad::{new_item as new_color_quad, ColorQuad};
use textured_quad::{new_item as new_textured_quad, TexturedQuad};
use triangle::{new_item as new_triangle, Triangle};
use logic::Logic;
use logic::defines::{Action, Direction};
use crate::logic::defines::WINDOW_SIZE;

#[wasm_bindgen]
pub struct AmberSkyNetClient {
    // logger: LoggerWeb,
    ctx: RenderContext,
    #[allow(dead_code)]
    triangle: Triangle,
    #[allow(dead_code)]
    color_quad: ColorQuad,
    #[allow(dead_code)]
    textured_quad: TexturedQuad,
    #[allow(dead_code)]
    view: View2D,
    #[allow(dead_code)]
    logic: Logic,
    #[allow(dead_code)]
    world: World
}

impl Default for AmberSkyNetClient {
    fn default() -> Self {
        let ctx = asn_render_webgl::init_context();

        let triangle = match new_triangle(&ctx) {
            Ok(t) => t,
            Err(err) => {
                LoggerWeb::log(&err);
                panic!()
            }
        };

        let color_quad = match new_color_quad(&ctx) {
            Ok(t) => t,
            Err(err) => {
                LoggerWeb::log(&err);
                panic!()
            }
        };

        let textured_quad = match new_textured_quad(&ctx) {
            Ok(t) => t,
            Err(err) => {
                LoggerWeb::log(&err);
                panic!()
            }
        };

        let logic = Logic::default();
        let world = logic::create_world();

        let view = match asn_view_2d::new_item(&ctx, &WINDOW_SIZE) {
            Ok(t) => t,
            Err(err) => {
                LoggerWeb::log(&err);
                panic!()
            }
        };

        Self {
            ctx,
            triangle,
            color_quad,
            textured_quad,
            view,
            logic,
            world
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

    pub fn on_event(&mut self, evt: web_sys::Event) -> Result<(), JsValue> {

        // let mess = format!("on_event: {:?}", evt.type_());
        // LoggerWeb::log(&mess);

        if let Some(key_e) = wasm_bindgen::JsCast::dyn_ref::<web_sys::KeyboardEvent>(&evt) {
            self.on_keyboard_event(key_e)?
        } else {
            // let mess = format!("on_event: {:?}", evt);
            // LoggerWeb::log(&mess);
        }
        Ok(())
    }

    fn on_keyboard_event (&mut self, evt: &web_sys::KeyboardEvent) -> Result<(), JsValue> {
        let key = key_utils::match_key(evt);
        self.logic.process_key(&mut self.world, key);
        Ok(())
    }

    pub fn upload_tiles(&mut self, data: Vec<u8>) -> Result<(), JsValue> {
        let mess = "engine upload_tiles";
        LoggerWeb::log(mess);
        game_utils::set_tiles(&self.ctx, &mut self.view, &data)?;
        Ok(())
    }

    pub fn upload_map(&mut self, data: Vec<u8>) -> Result<(), JsValue> {
        let mess = "engine upload_map";
        LoggerWeb::log(mess);
        game_utils::set_map(&mut self.logic, &mut self.world, &data)?;
        Ok(())
    }

    pub fn update(&mut self, time: f32) -> Result<(), JsValue> {
        // let mess = format!("update times: {} ", time);
        // LoggerWeb::log(&mess);
        // game_utils::update(&mut self.logic, time)?;
        self.logic.update(&mut self.world, &mut self.view, time)?;
        self.view.update(time)?;
        Ok(())
    }

    pub fn resize(&mut self, width: i32, height: i32) -> Result<(), JsValue> {
        let mess = format!("engine resize: {} x {}", width, height);
        LoggerWeb::log(&mess);
        asn_render_webgl::resize(&mut self.ctx, width, height);
        Ok(())
    }

    pub fn render(&mut self) -> Result<(), JsValue> {
        asn_render_webgl::draw(&self.ctx);
        // triangle::draw(&self.ctx, &self.triangle);
        // textured_quad::draw(&self.ctx, &self.textured_quad);
        self.view.draw(&self.ctx)?;
        // color_quad::draw(&self.ctx, &self.
        // color_quad);
        Ok(())
    }
}
