use asn_core::Array2D;
use asn_core::{Point2D, Size2D};
use asn_render_webgl::RenderContext;
use asn_view_2d::View2D;
use crate::new_view_2d;

const WINDOW_SIZE: Size2D = Size2D {
	width: 32,
	height: 32
};

pub struct CellGame {
	pub view: View2D,
	pub map: Array2D,
	window_size: Size2D,
	pos: Point2D
}

impl CellGame {
	pub fn new(ctx: &RenderContext) -> Result<CellGame, String> {
		let view_2d = new_view_2d(&ctx)?;

		let game = CellGame {
			map: Default::default(),
			pos: Default::default(),
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
		self.view.set_view(&self.window_size, &self.map)?;
		self.view.update(time)?;
		Ok(())
	}
}
