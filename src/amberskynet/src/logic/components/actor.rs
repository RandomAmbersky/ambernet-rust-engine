use specs::{Component, VecStorage};

pub struct Actor {}

impl Component for Actor {
	type Storage = VecStorage<Self>;
}
