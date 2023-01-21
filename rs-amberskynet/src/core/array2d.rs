use crate::core::size2d::Size2D;
use crate::core::Pos2D;

pub struct Array2D<S, T> {
    pub size: Size2D<S>,
    pub bytes: Vec<T>,
}

impl<S, T> Array2D<S, T> {
    pub fn set_point(&mut self, pos: &Pos2D<S>) {}
}
