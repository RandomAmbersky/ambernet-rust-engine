use crate::{Pos2D, UnsignedNum};

#[derive(Default, Debug, Copy, Clone)]
pub struct Size2D<T: UnsignedNum> {
    pub width: T,
    pub height: T,
}

impl<T: UnsignedNum> Size2D<T> {
    pub fn get_size(&self) -> T {
        self.width * self.height
    }
    pub fn get_index(&self, pos: &Pos2D<T>) -> Result<usize, String> {
        if !self.is_pos_into(pos) {
            return Err(String::from("Not in size"));
        };
        let result = (self.width * pos.y + pos.x).as_usize();
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

#[cfg(test)]
mod tests {
    use crate::*;

    type MySize2D = Size2D<u32>;

    #[test]
    fn get_index() {
        let my = MySize2D {
            width: 10,
            height: 20,
        };

        let pos = Pos2D { x: 50, y: 60 };
        let actual = my.get_index(&pos);
        assert!(actual.is_err());

        let pos = Pos2D { x: 5, y: 6 };
        let actual = my.get_index(&pos).unwrap();
        assert_eq!(actual, 6 * 10 + 5);
    }
}
