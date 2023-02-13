use crate::axe_dimension::AxeDimension;
use crate::pos2d::Pos2D;

pub struct Size2D<T: AxeDimension> {
    pub width: T,
    pub height: T,
}

impl<T: AxeDimension> Size2D<T> {
    pub fn get_index(&self, pos: &Pos2D<T>) -> usize {
        (self.width * pos.y + pos.x).to_usize()
    }
    pub fn check_into(&self, pos: &Pos2D<T>) -> bool {
        if self.width < pos.x {
            return false;
        }
        if self.height < pos.y {
            return false;
        }
        true
    }
}
