use asn_map;
use asn_map::Map;
use asn_view_2d::View2D;

#[derive(Default, Debug)]
pub struct CellGame {
	map: Map
}

impl CellGame {
	pub fn set_map(&mut self, map: Map) -> Result<(), String>{
		self.map = map;
		Ok(())
	}
}

// impl Default for CellGame {
//     fn default() -> Self {
//         CellGame{}
//     }
// }
