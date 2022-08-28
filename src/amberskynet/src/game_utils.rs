use amberskynet_logger_web::LoggerWeb;
use asn_array_2d::Array2D;
use asn_render_webgl::RenderContext;
use asn_tiled::load_xml_map;
use asn_view_2d::View2D;
use crate::cell_game::CellGame;

pub fn set_map(game: &mut CellGame, data: &Vec<u8>) -> Result<(), String> {
	let decoded_map = load_xml_map(&data)?;

	let mess = format!("parsed map is: {:?}", decoded_map);
	LoggerWeb::log(&mess);

	let map = Array2D {
		width: decoded_map.width,
		height: decoded_map.height,
		bytes: decoded_map.map,
	};

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
	asn_view_2d::set_tile_size(ctx, view, tile_width, tile_height)
}
