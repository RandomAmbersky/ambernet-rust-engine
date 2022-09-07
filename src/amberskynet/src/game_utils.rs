use rand::Rng;
use amberskynet_logger_web::LoggerWeb;
use asn_array_2d::Array2D;
use asn_core::Size2D;
use asn_images::decode_texture;
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

	let rect = Size2D {
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
	tile_size: &Size2D,
	data: &Vec<u8>) -> Result<(), String>
{
	let tex = decode_texture(data)?;
	view.set_tiles(ctx, tile_size, &tex)?;
	Ok(())
}

pub fn update(
	map: &mut Array2D,
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

	// let rnd = rng.gen_range(0..16);

	let mut cell = map.get_cell(x, y)?;
	cell += 1;
	// if cell > 14 {
	// 	cell = 0;
	// }

	map.set_cell(x, y, cell)?;
	match asn_view_2d::set_cell(view, x, y, cell as u32) {
	    Ok(()) => {},
	    Err(e) => {
	        LoggerWeb::log(&e);
	    }
	};

	Ok(())
}
