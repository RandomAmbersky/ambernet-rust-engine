mod utils;
mod texture_data;
pub mod render_tiles;

use amberskynet_logger_web::LoggerWeb;
use asn_render_webgl::{ RenderContext };

use web_sys::WebGlRenderingContext as GL;
use asn_core::{Array2D, Point2D, Size2D};

use render_tiles::RenderTiles;

pub struct View2D {
	screen: Array2D,
	render_tiles: RenderTiles,
	is_need_texture_update: bool
}

pub fn new_item(ctx: &RenderContext, window_size: &Size2D) -> Result<View2D, String> {
	let screen = Array2D {
		size: Size2D {
			width: window_size.width,
			height: window_size.height
		},
		bytes: Default::default()
	};

	let render_tiles = RenderTiles::new(ctx)?;

	let view2d = View2D {
		is_need_texture_update: false,
		render_tiles,
		screen,
	};
	Ok(view2d)
}

impl View2D {
	pub fn look_at(&mut self, pos: &Point2D, map: &Array2D) -> Result<(), String> {

		let n_pos = map.calc_screen_pos(pos, &self.screen.size)?;
		// let mess = format!("n_pos: {:?}", n_pos);
		// LoggerWeb::log(&mess);

		self.screen.cut_from(&n_pos, map)?;
		self.is_need_texture_update = true;
		Ok(())
	}

	// ф-ция для перекодирования номера Entity в текущий номер спрайта анимации (на будущее)
	// обновление таймеров спрайтов анимации
	pub fn update(&mut self, _time: f32) -> Result<(), String> {
		self.is_need_texture_update = true;
		Ok(())
	}

	pub fn draw(&mut self, ctx: &RenderContext) -> Result<(), String> {
		if self.is_need_texture_update {
			self.render_tiles.update_view(ctx, &self.screen)?;
			self.is_need_texture_update = false;
		}
		self.render_tiles.draw(ctx);
		Ok(())
	}
}
