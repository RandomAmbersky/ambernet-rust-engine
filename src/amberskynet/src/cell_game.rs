use asn_core::{Array2D, Point2D, Size2D};
use asn_render_webgl::RenderContext;
use asn_view_2d::{new_item as new_view_2d, View2D };

const WINDOW_SIZE: Size2D = Size2D {
	width: 10,
	height: 10
};

pub struct CellGame {
	pub map: Array2D,
	pub view: View2D
}

impl CellGame {
	pub fn new(ctx: &RenderContext) -> Result<CellGame, String> {
		let view_2d = new_view_2d(&ctx, &WINDOW_SIZE)?;

		let game = CellGame {
			map: Default::default(),
			view: view_2d,
		};
		Ok(game)
	}

	pub fn set_map(&mut self, map: Array2D) -> Result<(), String> {
		self.map = map;
		Ok(())
	}

	pub fn update(&mut self, time: f32) -> Result<(), String> {
		self.view.update(time)?;
		Ok(())
	}

	pub fn look_at(&mut self, pos: &Point2D) -> Result<(), String> {
		if self.map.is_zero() {
			return Err(String::from("map size is zero"))
		}
		self.view.look_at(pos, &self.map)?;
		Ok(())
	}
}
