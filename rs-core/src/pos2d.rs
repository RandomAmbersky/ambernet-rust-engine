use crate::core::axe_dimension::AxeDimension;

pub struct Pos2D<T: AxeDimension> {
    pub x: T,
    pub y: T,
}
