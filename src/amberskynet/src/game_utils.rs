use rand::Rng;
use amberskynet_logger_web::LoggerWeb;
use asn_core::{Array2D, Size2D};
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

pub fn set_tiles( ctx: &RenderContext, view: &mut View2D, image: &[u8]) -> Result<(), String>
{
	let tex = decode_texture(image)?;
	view.set_tiles(ctx, &TILE_SIZE, &tex)?;
	Ok(())
}

pub fn update(game: &mut CellGame, time: f32) -> Result<(), String> {
	game.update(time)?;

	for _ in 0..1000 {
		some(&mut game.map)?;
	}

	Ok(())
}

fn some (map: &mut Array2D) -> Result<(), String> {
	let mut rng = rand::thread_rng();

	if map.width == 0 || map.height == 0 {
		let mess = format!("map {} x {}", map.width, map.height);
		LoggerWeb::log(&mess);
		return Ok(());
	};
	let x = rng.gen_range(0..map.width);
	let y = rng.gen_range(0..map.height);

	// let rnd = rng.gen_range(0..16);

	let mut cell = map.get_cell(x, y)?;
	cell += 1;
	if cell > 192 {
		cell = 0;
	}

	map.set_cell(x, y, cell)?;
	Ok(())
}
