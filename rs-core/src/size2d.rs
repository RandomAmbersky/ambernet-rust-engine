use crate::axe_dimension::UnsignedDimension;
use crate::pos2d::Pos2D;

pub struct Size2D<T: UnsignedDimension> {
    pub width: T,
    pub height: T,
}

impl<T: UnsignedDimension> Size2D<T> {
    pub fn get_index(&self, pos: &Pos2D<T>) -> Result<usize, String> {
        if !self.is_pos_into(pos) {
            return Err(String::from("Not in size"));
        };
        let result = (self.width * pos.y + pos.x).to_usize();
        Ok(result)
    }
    pub fn is_pos_into(&self, pos: &Pos2D<T>) -> bool {
        if self.width < pos.x {
            return false;
        }
        if self.height < pos.y {
            return false;
        }
        true
    }
}
