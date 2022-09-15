use specs::World;
use amberskynet_logger_web::LoggerWeb;
use asn_core::{Array2D, Point2D, Size2D};
use asn_images::decode_texture;
use asn_render_webgl::RenderContext;
use asn_tiled::load_xml_map;
use asn_view_2d::View2D;
use crate::{logic, Logic};
use crate::logic::defines::TILE_SIZE;

pub fn set_map(l: &mut Logic, w: &mut World, data: &[u8]) -> Result<(), String> {
	let decoded_map = load_xml_map(&data)?;
	let mess = format!("parsed map is: {:?}", decoded_map);
	LoggerWeb::log(&mess);
	l.set_map(w, decoded_map);
	Ok(())
}

pub fn set_tiles(ctx: &RenderContext, view: &mut View2D, image: &[u8]) -> Result<(), String>
{
	let tex = decode_texture(image)?;
	view.set_tiles(ctx, &tex, &TILE_SIZE)?;
	Ok(())
}
