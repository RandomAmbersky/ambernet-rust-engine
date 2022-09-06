use asn_array_2d;
use asn_array_2d::Array2D;
use asn_core::Point2D;

#[derive(Default, Debug)]
pub struct CellGame {
	pub map: Array2D,
	pub pos: Point2D
}

impl CellGame {
	pub fn set_map(&mut self, map: Array2D) -> Result<(), String>{
		self.map = map;
		Ok(())
	}
}
