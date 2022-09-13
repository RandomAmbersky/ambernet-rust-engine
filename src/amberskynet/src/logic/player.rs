use specs::{Component, VecStorage};

pub struct Player {}

impl Component for Player {
	type Storage = VecStorage<Self>;
}
