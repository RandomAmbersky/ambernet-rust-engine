mod utils;
mod texture_data;
mod render_data;
mod view_utils;

use amberskynet_logger_web::LoggerWeb;
use asn_render_webgl::{ RenderContext };

use web_sys::WebGlRenderingContext as GL;
use asn_core::{Array2D, Point2D, Size2D};

use render_data::RenderData;
use crate::view_utils::set_view_from;

pub struct View2D {
	view: Array2D,
	render_data: RenderData,
	is_need_update_texture_view: bool
}

pub fn new_item (ctx: &RenderContext) -> Result<View2D, String> {

	let render_data = RenderData::new(ctx)?;

	let view2d = View2D {
		render_data,
		view: Array2D::default(),
		is_need_update_texture_view: false
	};
	Ok(view2d)
}

pub fn set_cell (item: &mut View2D, x: u32, y: u32, cell: u32) -> Result<(), String> {
	let index = get_index(item, x, y)?;

	let cell_u8 = cell as u8;

	item.view.bytes[index] = cell_u8;

	// let cell_y = cell_u8 / 16;
	// let cell_x = cell_u8 - cell_y * 16;

	// let mess = format!("set_cell on {}, {}", cell_y, cell_x);
	// LoggerWeb::log(&mess);

	// let texture_index = get_texture_index(item, x, y)?;

	// item.texture_data.bytes[texture_index] = cell_x as u8;
	// item.texture_data.bytes[texture_index+1] = cell_y as u8;
	// item.texture_data.bytes[texture_index+2] = 255;
	// item.texture_data.bytes[texture_index+3] = 255;
	//
	Ok(())
}

pub fn get_index (item: &mut View2D, x: u32, y: u32) -> Result<usize, String> {
	let index = (item.view.width * y + x ) as usize;

	if index >= item.view.bytes.len() {
		let mess = format!("Invalid index {} on map [{},{}]", index, x, y);
		return Err(mess)
	};

	Ok(index)
}

impl View2D {
	pub fn set_view (&mut self, window: &Size2D, map: &Array2D) -> Result<(), String> {
		let pos: Point2D = Point2D {
			x: 0,
			y: 0
		};
		set_view_from(map, &pos, window, &mut self.view)?;
		self.is_need_update_texture_view = true;
		Ok(())
	}
	pub fn set_tiles(&self, ctx: &RenderContext, tile_size: &Size2D, tex: &Array2D) -> Result<(), String>
	{
		self.render_data.update_tiles(ctx, tex)?;
		self.render_data.set_tile_size(ctx, tile_size)?;
		Ok(())
	}
	pub fn draw(&mut self, ctx: &RenderContext) -> Result<(), String> {
		if self.is_need_update_texture_view {
			self.update_texture_view(ctx)?;
			self.is_need_update_texture_view = false;
		}
		self.render_data.draw(ctx);
		Ok(())
	}
	fn update_texture_view (&self, ctx: &RenderContext) -> Result<(), String> {
		let texture_data = texture_data::from_array2d(&self.view, 16);
		self.render_data.update_view(ctx, &texture_data)?;
		Ok(())
	}
	pub fn update(&mut self, _time: f32) -> Result<(), String> {
		self.is_need_update_texture_view = true;
		Ok(())
	}
}
