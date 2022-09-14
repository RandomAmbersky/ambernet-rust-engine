use asn_core::{Delta2D};

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
	pub fn to_delta(&self) -> Delta2D {
		let mut res = Delta2D::default();
		match self {
				Direction::Down => res.y -=1,
				Direction::Up => res.y +=1,
				Direction::Left => res.x -=1,
				Direction::Right => res.x +=1,
		}
		return res;
	}
}
