use asn_core::Point2D;

pub enum Direction {
	Up,
	Down,
	Left,
	Right
}

pub enum Action {
	Move,
	Use
}

impl Direction {
	pub fn do_move(&self, pos: &mut Point2D ) -> Result<(), String>{
		match self {
				Direction::Down => pos.y -=1,
				Direction::Up => pos.y +=1,
				Direction::Left => pos.x -=1,
				Direction::Right => pos.x +=1,
		}
		Ok(())
	}
}
