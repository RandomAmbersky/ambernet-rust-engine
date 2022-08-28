mod utils;
mod cell_game;
mod game_utils;

use asn_view_2d::{new_item as new_view_2d, View2D, set_tiles};
use asn_tiled::load_xml_map;

use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use amberskynet_logger_web::LoggerWeb;
use asn_render_webgl::RenderContext;

use color_quad::{new_item as new_color_quad, ColorQuad};
use textured_quad::{new_item as new_textured_quad, TexturedQuad};
use triangle::{new_item as new_triangle, Triangle};
use crate::cell_game::CellGame;

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
    cell_game: CellGame,
    view_2d: View2D
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

        let view_2d = match new_view_2d(&ctx) {
            Ok(t) => t,
            Err(err) => {
                LoggerWeb::log(&err);
                panic!()
            }
        };

        let cell_game = CellGame::default();

        Self {
            // logger: LoggerWeb {},
            ctx,
            triangle,
            color_quad,
            textured_quad,
            cell_game,
            view_2d
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

    pub fn upload_tiles(&mut self, data: Vec<u8>) -> Result<(), JsValue> {
        let mess = "engine upload_tiles";
        LoggerWeb::log(mess);
        game_utils::set_tiles(&self.ctx, &mut self.view_2d, 16, 16, &data)?;
        Ok(())
    }

    pub fn upload_map(&mut self, data: Vec<u8>) -> Result<(), JsValue> {
        let mess = "engine upload_map";
        LoggerWeb::log(mess);

        game_utils::set_map(&mut self.cell_game, &data)?;
        // asn_view_2d::set_tile_size(&self.ctx, &mut self.view_2d, map.tile_width as u32, map.tile_height as u32);
        Ok(())
    }

    pub fn update(&mut self, _time: f32) -> Result<(), JsValue> {
        let rnd: u8 = rand::random::<u8>();
        // match asn_view_2d::set_cell(&mut self.view_2d, 10, 10, rnd as u32) {
        //     Ok(()) => {},
        //     Err(e) => {
        //         LoggerWeb::log(&e);
        //     }
        // };

        let mess = format!("engine update: {} {}", _time, rnd);
        // say_hello();
        LoggerWeb::log(&mess);
        Ok(())
    }

    pub fn resize(&mut self, width: i32, height: i32) -> Result<(), JsValue> {
        let mess = format!("engine resize: {} x {}", width, height);
        LoggerWeb::log(&mess);
        asn_render_webgl::resize(&mut self.ctx, width, height);
        Ok(())
    }

    pub fn render(&self) -> Result<(), JsValue> {
        asn_render_webgl::draw(&self.ctx);
        // triangle::draw(&self.ctx, &self.triangle);
        // textured_quad::draw(&self.ctx, &self.textured_quad);
        // asn_view_2d::draw(&self.ctx, &self.view_2d);
        // color_quad::draw(&self.ctx, &self.color_quad);
        Ok(())
    }
}
