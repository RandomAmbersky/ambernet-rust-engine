use specs::{Component, VecStorage};
use asn_core::Point2D;

#[derive(Default, Debug)]
pub struct Position {
    pub pos: Point2D
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}
