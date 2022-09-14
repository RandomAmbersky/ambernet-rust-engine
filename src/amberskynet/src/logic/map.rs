use specs::{Component, VecStorage};

pub struct Map {
    pub width: u32,
    pub height: u32,
    pub bytes: Vec<u8>
}

impl Component for Map {
    type Storage = VecStorage<Self>;
}
