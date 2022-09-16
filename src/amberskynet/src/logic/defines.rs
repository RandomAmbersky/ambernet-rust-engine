use asn_core::{Size2D};

pub const PLAYER_SPRITE_ID: u8 = 161;

pub const TILE_SIZE: Size2D = Size2D {
	width: 16,
	height: 16
};

pub const WINDOW_SIZE: Size2D = Size2D {
	width: 30,
	height: 30
};


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
