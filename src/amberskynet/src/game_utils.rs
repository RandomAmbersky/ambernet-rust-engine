use rand::Rng;
use amberskynet_logger_web::LoggerWeb;
use asn_core::{Array2D, Point2D, Size2D};
use asn_images::decode_texture;
use asn_render_webgl::RenderContext;
use asn_tiled::load_xml_map;
use asn_view_2d::View2D;
use crate::cell_game::CellGame;

const TILE_SIZE: Size2D = Size2D {
	width: 16,
	height: 16
};

pub fn set_map(game: &mut CellGame, data: &[u8]) -> Result<(), String> {
	let decoded_map = load_xml_map(&data)?;
	// let mess = format!("parsed map is: {:?}", decoded_map);
	// LoggerWeb::log(&mess);
	game.set_map(decoded_map)?;
	Ok(())
}

pub fn set_tiles(ctx: &RenderContext, view: &mut View2D, image: &[u8]) -> Result<(), String>
{
	let tex = decode_texture(image)?;
	view.set_tiles(ctx, &TILE_SIZE, &tex)?;
	Ok(())
}

pub fn update(game: &mut CellGame, time: f32) -> Result<(), String> {
	game.update(time)?;

	let mut rng = rand::thread_rng();
	let pos = Point2D {
		x: rng.gen_range(0..game.map.width),
		y: rng.gen_range(0..game.map.height)
	};

	let rnd = rng.gen_range(0..100);

	if rnd > 90 {
		game.look_at(&pos)?;
	}

	Ok(())
}
