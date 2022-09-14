use specs::{Component, VecStorage};
use asn_core::Array2D;

pub struct Render {
    screen: Array2D,
    is_need_texture_update: bool
}

impl Component for Render {
    type Storage = VecStorage<Self>;
}
