use crate::axe_dimension::AxeDimension;
use crate::pos2d::Pos2D;

pub struct Size2D<T: AxeDimension> {
    pub width: T,
    pub height: T,
}

impl<T: AxeDimension> Size2D<T> {
    pub fn get_index_from_xy(&self, x: T, y: T) -> T {
        self.width * y + x
    }
    pub fn get_index(&self, pos: &Pos2D<T>) -> usize {
        (self.width * pos.y + pos.x).to_usize()
    }
}
