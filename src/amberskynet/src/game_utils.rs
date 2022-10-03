use specs::World;
use amberskynet_logger_web::LoggerWeb;
use asn_core::Array2D;
use asn_images::decode_texture;
use asn_render_webgl::RenderContext;
use asn_tiled::{ load_tmx };
use asn_view_2d::View2D;
use crate::{Logic};
use crate::logic::defines::TILE_SIZE;
// use tiled::parse;
pub fn set_map(l: &mut Logic, w: &mut World, data: &[u8]) -> Result<(), String> {
	let decoded_map = load_tmx(&data)?;

	// let res = match parse(data) {
	// 	Ok(t) => t,
	// 	Err(err) => {
	// 		return Err(err.to_string())
	// 	},
	// };
	// let mess = format!("parsed map is: {:?}", res);
	// LoggerWeb::log(&mess);

	let mess = format!("parsed map is: {:?}", decoded_map);
	LoggerWeb::log(&mess);
	let last_layer_idx = &decoded_map.layers.len() - 1 ;
	let one_layer = &decoded_map.layers[0];
	let mess = format!("one_layer is: {:?}", one_layer.name);
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
