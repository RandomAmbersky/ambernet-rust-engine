use crate::core::Pos2D;
use std::ops::{Add, Mul};

pub struct Size2D<T> {
    pub width: T,
    pub height: T,
}

impl<T> Size2D<T>
where
    T: Mul<T, Output = T> + Add<T, Output = T>,
{
    fn get_index_from_xy(self, x: T, y: T) -> T {
        self.width * y + x
    }
    fn get_index(self, pos: Pos2D<T>) -> T {
        self.width * pos.y + pos.x
    }
}
