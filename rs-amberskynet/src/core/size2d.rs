use crate::core::Pos2D;
use std::ops::{Add, Mul};

#[derive(Default, Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Size2D<T> {
    pub width: T,
    pub height: T,
}

impl<T> Size2D<T>
where
    T: Mul<T, Output = T> + Add<T, Output = T> + Copy,
{
    pub fn get_index_from_xy(self, x: T, y: T) -> T {
        self.width * y + x
    }
    pub fn get_index(self, pos: &Pos2D<T>) -> T {
        self.width * pos.y + pos.x
    }
}
