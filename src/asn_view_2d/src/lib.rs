mod utils;
mod texture_data;
mod render_data;

use amberskynet_logger_web::LoggerWeb;
use asn_render_webgl::{ RenderContext };

use web_sys::WebGlRenderingContext as GL;
use asn_core::{Array2D, Point2D, Size2D};

use render_data::RenderData;
use crate::texture_data::from_array2d;

pub struct View2D {
	screen: Array2D,
	render_data: RenderData,
	is_need_texture_update: bool
}

pub fn new_item(ctx: &RenderContext, window_size: &Size2D) -> Result<View2D, String> {
	let render_data = RenderData::new(ctx)?;

	let screen = Array2D {
		width: window_size.width,
		height: window_size.height,
		bytes: Default::default()
	};

	let view2d = View2D {
		render_data,
		is_need_texture_update: false,
		screen,
	};
	Ok(view2d)
}

impl View2D {
	pub fn set_tiles(&self, ctx: &RenderContext, tile_size: &Size2D, tex: &Array2D) -> Result<(), String>
	{
		self.render_data.update_tiles(ctx, tex, tile_size)?;
		Ok(())
	}

	pub fn look_at(&mut self, pos: &Point2D, map: &Array2D) -> Result<(), String> {

		if self.screen.is_zero() {
			return Err(String::from("window size is zero"))
		}

		let half_width = self.screen.width / 2;
		let half_height = self.screen.height / 2;

		let map_width_minus_width = map.width - self.screen.width;
		let map_height_minus_height = map.height - self.screen.height;

		let mut n_pos = *pos;

		if n_pos.x > half_width {
			n_pos.x -= half_width;
		} else {
			n_pos.x = 0;
		}

		if n_pos.y > half_height {
			n_pos.y -= half_height;
		} else {
			n_pos.y = 0;
		}

		if n_pos.y > map_height_minus_height {
			n_pos.y = map_height_minus_height;
		}

		if n_pos.x > map_width_minus_width {
			n_pos.x = map_width_minus_width;
		}

		// let mess = format!("n_pos: {:?}", n_pos);
		// LoggerWeb::log(&mess);

		self.screen.cut_from(&n_pos, map)?;
		self.is_need_texture_update = true;
		Ok(())
	}

	// ф-ция для перекодирования номера Entity в текущий номер спрайта анимации (на будущее)
	// обновление таймеров спрайтов анимации
	pub fn update(&mut self, _time: f32) -> Result<(), String> {
		// self.is_need_texture_update = true;
		Ok(())
	}

	pub fn draw(&mut self, ctx: &RenderContext) -> Result<(), String> {
		if self.is_need_texture_update {
			let texture = from_array2d(&self.screen, 16);
			self.render_data.update_view(ctx, &texture)?;
			self.is_need_texture_update = false;
		}
		self.render_data.draw(ctx);
		Ok(())
	}
}
