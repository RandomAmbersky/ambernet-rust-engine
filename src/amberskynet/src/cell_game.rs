use asn_core::Array2D;
use asn_core::{Point2D, Size2D};
use asn_render_webgl::RenderContext;
use asn_view_2d::View2D;
use crate::new_view_2d;

const WINDOW_SIZE: Size2D = Size2D {
	width: 10,
	height: 10
};

pub struct CellGame {
	pub view: View2D,
	pub map: Array2D,
	window_size: Size2D
}

impl CellGame {
	pub fn new(ctx: &RenderContext) -> Result<CellGame, String> {
		let view_2d = new_view_2d(&ctx)?;

		let game = CellGame {
			map: Default::default(),
			window_size: WINDOW_SIZE,
			view: view_2d
		};
		Ok(game)
	}

	pub fn set_map(&mut self, map: Array2D) -> Result<(), String> {
		self.map = map;
		self.view.set_view(&self.window_size, &self.map)?;
		Ok(())
	}

	pub fn update(&mut self, time: f32) -> Result<(), String> {
		self.view.update(time)?;
		Ok(())
	}

	pub fn look_at(&mut self, pos: &Point2D) -> Result<(), String> {
		self.view.look_at(pos, &self.window_size, &self.map)?;
		Ok(())
	}
}
