use crate::core::size2d::Size2D;

pub struct Array2D<S, T> {
    pub size: Size2D<S>,
    pub bytes: Vec<T>,
}
