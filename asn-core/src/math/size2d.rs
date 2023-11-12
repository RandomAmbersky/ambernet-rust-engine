use super::{Pos2D, UnsignedNum};

#[derive(Default, Debug, Copy, Clone)]
pub struct Size2D<T>
where
    T: UnsignedNum,
{
    pub width: T,
    pub height: T,
}

impl<T> Size2D<T>
where
    T: UnsignedNum,
{
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

    // Посмотреть на позицию pos через окно размером window
    pub fn look_at_window(&self, pos: &Pos2D<T>, window: Size2D<T>) -> Pos2D<T> {
        if self.width < window.width {
            panic!("Window {:?} too large for size {:?}", window, self)
        }

        if self.height < window.height {
            panic!("Window {:?} too large for size {:?}", window, self)
        }

        let mut look_at_pos = *pos;

        let half_width: T = window.height >> T::ONE;
        let half_height: T = window.height >> T::ONE;

        let map_width_minus_width = self.width - window.width;
        let map_height_minus_height = self.height - window.height;

        if look_at_pos.x > half_width {
            look_at_pos.x = look_at_pos.x - half_width - T::from_usize(10);
        } else {
            look_at_pos.x = T::ZERO;
        }

        if look_at_pos.y > half_height {
            look_at_pos.y = look_at_pos.y - half_height;
        } else {
            look_at_pos.y = T::ZERO;
        }

        if look_at_pos.y > map_height_minus_height {
            look_at_pos.y = map_height_minus_height;
        }

        if look_at_pos.x > map_width_minus_width {
            look_at_pos.x = map_width_minus_width;
        }

        look_at_pos
    }
}

#[cfg(test)]
mod tests {
    use crate::math::Pos2D;

    use super::Size2D;

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
