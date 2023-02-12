use crate::core::pos2d::Pos2D;
use std::ops::{Add, Mul};

pub struct Size2D<T> {
    pub width: T,
    pub height: T,
}

impl<T> Size2D<T>
where
    T: Mul<T, Output = T> + Add<T, Output = T> + Copy,
{
    pub fn get_index_from_xy(&self, x: T, y: T) -> T {
        self.width * y + x
    }
    pub fn get_index(&self, pos: &Pos2D<T>) -> T {
        self.width * pos.y + pos.x
    }
}

#[cfg(test)]
mod tests {
    use crate::core::size2d::Size2D;

    #[test]
    fn it_works() {
        let value: u32 = 10;
        let pos = Size2D {
            width: value,
            height: value,
        };
        assert_eq!(pos.width, value);
        assert_eq!(pos.height, value);
    }
}
