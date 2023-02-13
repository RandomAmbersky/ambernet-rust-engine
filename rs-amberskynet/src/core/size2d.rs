use crate::core::axe_dimension::AxeDimension;
use crate::core::Pos2D;

pub struct Size2D<T: AxeDimension> {
    pub width: T,
    pub height: T,
}

impl<T: AxeDimension> Size2D<T> {
    pub fn get_index_from_xy(self, x: T, y: T) -> T {
        self.width * y + x
    }
    pub fn get_index(self, pos: &Pos2D<T>) -> T {
        self.width * pos.y + pos.x
    }
}
