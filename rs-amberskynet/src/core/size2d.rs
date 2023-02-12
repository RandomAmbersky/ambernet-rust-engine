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
    use crate::core::Pos2D;

    #[test]
    fn it_works() {
        let value: u32 = 10;
        let my_size = Size2D {
            width: value,
            height: value,
        };
        assert_eq!(my_size.width, value);
        assert_eq!(my_size.height, value);
    }

    #[test]
    fn get_index_from_xy() {
        let value: u32 = 10;
        let my_size = Size2D {
            width: value,
            height: value,
        };

        let index_value: u32 = 5;
        let index = my_size.get_index_from_xy(index_value, index_value);
        assert_eq!(index, 55);
    }

    #[test]
    fn get_index() {
        let value: u32 = 10;
        let my_size = Size2D {
            width: value,
            height: value,
        };

        let index_value: u32 = 5;
        let pos = Pos2D {
            x: index_value,
            y: index_value,
        };
        let index = my_size.get_index(&pos);
        assert_eq!(index, 55);
    }
}
