use asn_array_2d;
use asn_array_2d::Array2D;
use asn_view_2d::View2D;

#[derive(Default, Debug)]
pub struct CellGame {
	pub map: Array2D
}

impl CellGame {
	pub fn set_map(&mut self, map: Array2D) -> Result<(), String>{
		self.map = map;
		Ok(())
	}
}

// impl Default for CellGame {
//     fn default() -> Self {
//         CellGame{}
//     }
// }
