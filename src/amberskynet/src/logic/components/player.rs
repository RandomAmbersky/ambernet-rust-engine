use crate::logic::defines::PLAYER_SPRITE_ID;
use asn_core::Direction;
use specs::{Component, VecStorage};

#[derive(Default, Debug)]
pub struct Player {
    pub dir: Direction,
}

impl Component for Player {
    type Storage = VecStorage<Self>;
}

pub fn dir_to_sprite(dir: &Direction) -> u8 {
    match dir {
        Direction::Up => PLAYER_SPRITE_ID,
        Direction::Down => PLAYER_SPRITE_ID + 4,
        Direction::Left => PLAYER_SPRITE_ID + 2,
        Direction::Right => PLAYER_SPRITE_ID + 6,
    }
}
