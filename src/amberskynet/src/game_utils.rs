use specs::World;
use amberskynet_logger_web::LoggerWeb;
use asn_images::decode_texture;
use asn_render_webgl::RenderContext;
use asn_tiled::{ load_tmx, load_tsx };
use asn_view_2d::View2D;
use crate::{Logic};
use crate::logic::defines::TILE_SIZE;


pub fn set_map(l: &mut Logic, w: &mut World, data: &[u8]) -> Result<(), String> {
	let decoded_map = load_tmx(&data)?;
	let mess = format!("parsed map is: {:?}", decoded_map);
	LoggerWeb::log(&mess);
	l.set_map(w, decoded_map)?;
	Ok(())
}

pub fn set_tiles(ctx: &RenderContext, view: &mut View2D, image: &[u8]) -> Result<(), String>
{
	let tex = decode_texture(image)?;
	view.set_tiles(ctx, &tex, &TILE_SIZE)?;
	Ok(())
}

pub fn set_cell_types (_l: &mut Logic, data: &[u8]) -> Result<(), String> {
	let tileset = load_tsx(&data)?;
	let mess = format!("tileset is: {:?}", tileset);
	LoggerWeb::log(&mess);
	Ok(())
}
