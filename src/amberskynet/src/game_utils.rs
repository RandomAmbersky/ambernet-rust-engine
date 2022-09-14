use rand::Rng;
use amberskynet_logger_web::LoggerWeb;
use asn_core::{Array2D, Point2D, Size2D};
use asn_images::decode_texture;
use asn_render_webgl::RenderContext;
use asn_tiled::load_xml_map;
use asn_view_2d::render_tiles::RenderTiles;
use asn_view_2d::View2D;

use crate::Logic;
use crate::logic::defines::TILE_SIZE;

pub fn set_map(logic: &mut Logic, data: &[u8]) -> Result<(), String> {
	let decoded_map = load_xml_map(&data)?;
	// let mess = format!("parsed map is: {:?}", decoded_map);
	// LoggerWeb::log(&mess);
	logic.set_map(decoded_map);
	Ok(())
}

pub fn set_tiles(ctx: &RenderContext, render_tiles: &mut RenderTiles, image: &[u8]) -> Result<(), String>
{
	let tex = decode_texture(image)?;
	render_tiles.update_tiles(ctx, &tex, &TILE_SIZE)?;
	Ok(())
}

pub fn update(logic: &mut Logic, render_tiles: &mut RenderTiles, time: f32) -> Result<(), String> {
	
// 	game.update(time)?;
//
// 	let mut rng = rand::thread_rng();
// 	let pos = Point2D {
// 		x: rng.gen_range(0..game.map.size.width),
// 		y: rng.gen_range(0..game.map.size.height)
// 	};
//
// 	let rnd = rng.gen_range(0..100);
//
// 	if rnd > 90 {
// 		game.look_at(&pos)?;
// 	}
//
	Ok(())
}
