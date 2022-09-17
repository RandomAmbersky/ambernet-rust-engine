use specs::{Component, VecStorage};

#[derive(Default, Debug)]
pub struct Background {
    pub cell: u8
}

impl Component for Background {
    type Storage = VecStorage<Self>;
}
