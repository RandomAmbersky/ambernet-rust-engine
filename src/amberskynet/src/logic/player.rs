use specs::{Component, VecStorage};

#[derive(Default, Debug)]
pub struct Player {
}

impl Component for Player {
    type Storage = VecStorage<Self>;
}
