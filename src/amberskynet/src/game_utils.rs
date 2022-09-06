use rand::Rng;
use amberskynet_logger_web::LoggerWeb;
use asn_array_2d::Array2D;
use asn_core::Rect2D;
use asn_render_webgl::RenderContext;
use asn_tiled::load_xml_map;
use asn_view_2d::View2D;
use crate::cell_game::CellGame;

pub fn set_map(game: &mut CellGame, view: &mut View2D, data: &Vec<u8>) -> Result<(), String> {
	let decoded_map = load_xml_map(&data)?;

	let mess = format!("parsed map is: {:?}", decoded_map);
	LoggerWeb::log(&mess);

	let map = Array2D {
		width: decoded_map.width,
		height: decoded_map.height,
		bytes: decoded_map.map,
	};

	let rect = Rect2D {
		width: decoded_map.width,
		height: decoded_map.height
	};

	asn_view_2d::set_view(view, rect, &map.bytes);
	game.set_map(map)?;
	Ok(())
}

pub fn set_tiles(
	ctx: &RenderContext,
	view: &mut View2D,
	tile_width: u32,
	tile_height: u32,
	data: &Vec<u8>) -> Result<(), String> {
	asn_view_2d::set_tiles(ctx, view, data)?;
	asn_view_2d::set_tile_size(ctx, view, tile_width, tile_height)?;
	asn_view_2d::update_view(ctx, view);
	Ok(())
}

pub fn update(
	map: &Array2D,
	view: &mut View2D,
	_time: f32
) -> Result<(), String> {
	let mut rng = rand::thread_rng();
	if map.width == 0 || map.height == 0 {
		let mess = format!("map {} x {}", map.width, map.height);
		LoggerWeb::log(&mess);
		return Ok(());
	};
	let x = rng.gen_range(0..map.width);
	let y = rng.gen_range(0..map.height);

	let rnd = rng.gen_range(0..254);

	// let mut cell = map.get_cell(x, y)?;
	// cell += 1;
	// if cell > 255 {
	// 	cell = 0;
	// }

	match asn_view_2d::set_cell(view, x, y, rnd as u32) {
	    Ok(()) => {},
	    Err(e) => {
	        LoggerWeb::log(&e);
	    }
	};

	// LoggerWeb::log(&mess);
	Ok(())
}
