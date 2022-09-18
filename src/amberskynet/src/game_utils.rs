use std::any::Any;
use specs::World;
use amberskynet_logger_web::LoggerWeb;
use asn_core::Array2D;
use asn_images::decode_texture;
use asn_render_webgl::RenderContext;
use asn_tiled::{ load_tmx, load_tsx };
use asn_view_2d::View2D;
use crate::{Logic};
use crate::logic::defines::TILE_SIZE;

pub fn set_map(l: &mut Logic, w: &mut World, data: &[u8]) -> Result<(), String> {
	let decoded_map = load_tmx(&data)?;
	// let mess = format!("parsed map is: {:?}", decoded_map);
	// LoggerWeb::log(&mess);
	// let last_layer_idx = &decoded_map.layers.len() - 1 ;
	let one_layer = &decoded_map.layers[0];
	// let mess = format!("one_layer is: {:?}", one_layer.name);
	let map = Array2D {
		size: decoded_map.size,
		bytes: one_layer.bytes.to_vec()
	};
	l.set_map(w, map)?;
	Ok(())
}

pub fn set_tiles(ctx: &RenderContext, view: &mut View2D, image: &[u8]) -> Result<(), String>
{
	let tex = decode_texture(image)?;
	view.set_tiles(ctx, &tex, &TILE_SIZE)?;
	Ok(())
}

pub fn set_cell_types (l: &mut Logic, data: &[u8]) -> Result<(), String> {
	let tileset = load_tsx(&data)?;
	for tile in tileset.tiles {
		l.set_tile_type(tile.id, &tile.class)?;
	}
	Ok(())
}
