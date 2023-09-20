use super::{Pos3D, UnsignedNum};

#[derive(Default, Debug, Copy, Clone)]
pub struct Size3D<T: UnsignedNum> {
    pub width: T,
    pub height: T,
    pub depth: T,
}

impl<T: UnsignedNum> Size3D<T> {
    pub fn get_size(&self) -> T {
        self.width * self.height * self.depth
    }
    pub fn get_index(&self, pos: &Pos3D<T>) -> Result<usize, String> {
        if !self.is_pos_into(pos) {
            return Err(String::from("Not in size"));
        };
        let result = (self.width * self.height * pos.z + self.width * pos.y + pos.x).as_usize();
        Ok(result)
    }
    pub fn is_pos_into(&self, pos: &Pos3D<T>) -> bool {
        if self.width < pos.x {
            return false;
        }
        if self.height < pos.y {
            return false;
        }
        if self.depth < pos.z {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Size3D;
    use crate::math::Pos3D;

    type MySize3D = Size3D<u32>;

    #[test]
    fn get_index() {
        let my = MySize3D {
            width: 10,
            height: 20,
            depth: 30,
        };

        let pos = Pos3D {
            x: 50,
            y: 60,
            z: 70,
        };
        let actual = my.get_index(&pos);
        assert!(actual.is_err());

        let pos = Pos3D { x: 5, y: 6, z: 7 };
        let actual = my.get_index(&pos).unwrap();
        assert_eq!(actual, 6 * 10 + 5);
    }
}
