use asn_core::{Delta2D, Size2D};

pub const PLAYER_SPRITE_ID: u8 = 161;

pub const TILE_SIZE: Size2D = Size2D {
	width: 16,
	height: 16
};

pub const WINDOW_SIZE: Size2D = Size2D {
	width: 10,
	height: 10
};


#[derive(Debug)]
pub enum Direction {
	Up,
	Down,
	Left,
	Right
}

#[derive(Debug)]
pub enum Action {
	Move,
	Use
}

#[derive(Debug)]
pub enum Key {
	None,
	Up,
	Down,
	Left,
	Right,
	Fire
}

impl Direction {
	pub fn from_key(key: &Key) -> Result<Direction, String> {
		let result = match key {
			Key::Up=> Direction::Up,
			Key::Down => Direction::Down,
			Key::Left => Direction::Left,
			Key::Right=> Direction::Right,
		    _ => return Err(String::from("Invalid direction"))
		};
		return Ok(result)
	}

	pub fn as_delta(&self) -> Delta2D {
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
