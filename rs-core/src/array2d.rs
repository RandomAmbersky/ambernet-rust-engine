use crate::axe_dimension::UnsignedDimension;
use crate::cell_type::CellType;
use crate::pos2d::Pos2D;
use crate::size2d::Size2D;

pub struct Array2D<S: UnsignedDimension, T: CellType> {
    pub size: Size2D<S>,
    pub bytes: Vec<T>,
}

impl<S: UnsignedDimension, T: CellType> Array2D<S, T> {
    pub fn check_in_array(&self, pos: &Pos2D<S>) -> bool {
        if self.size.width < pos.x {
            println!("{} < {}", self.size.width, pos.x);
            return false;
        }
        if self.size.height < pos.y {
            println!("{} < {}", self.size.height, pos.y);
            return false;
        }
        true
    }
    pub fn get_point(&self, pos: &Pos2D<S>) -> Result<T, String> {
        if !self.check_in_array(pos) {
            let err_msg = format!(
                "Not in array {:?} x {} [{}, {}]",
                self.size.width, self.size.height, pos.x, pos.y
            );
            return Err(err_msg);
        }
        let index = self.size.get_index(pos);
        let value = self.bytes[index];
        Ok(value)
    }
    pub fn set_point(&mut self, pos: &Pos2D<S>, val: T) -> Result<(), String> {
        if !self.check_in_array(pos) {
            let err_msg = format!(
                "Not in array {:?} x {} [{}, {}]",
                self.size.width, self.size.height, pos.x, pos.y
            );
            return Err(err_msg);
        }
        let index = self.size.get_index(pos);
        self.bytes[index] = val;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::array2d::Array2D;
    use crate::axe_dimension::UnsignedDimension;
    use crate::cell_type::CellType;
    use crate::pos2d::Pos2D;
    use crate::size2d::Size2D;

    type Axe = u32;
    impl UnsignedDimension for Axe {
        fn to_usize(&self) -> usize {
            usize::try_from(*self).expect("convert error")
        }
    }

    type Byte = u8;
    impl CellType for Byte {}

    #[test]
    fn check_in_array() {
        let axe_value: Axe = 10;
        let arr = Array2D {
            size: Size2D {
                width: axe_value,
                height: axe_value,
            },
            bytes: vec![0 as Byte; 100],
        };

        let input = Pos2D {
            x: 5 as Axe,
            y: 5 as Axe,
        };
        let result = arr.check_in_array(&input);
        assert!(result);

        let input = Pos2D {
            x: 100 as Axe,
            y: 110 as Axe,
        };
        let result = arr.check_in_array(&input);
        assert!(!result)
    }

    #[test]
    fn get_point() {
        let axe_value: Axe = 10;
        let arr = Array2D {
            size: Size2D {
                width: axe_value,
                height: axe_value,
            },
            bytes: vec![55 as Byte; 100],
        };
        let pos = Pos2D {
            x: 5 as Axe,
            y: 5 as Axe,
        };
        let result = arr.get_point(&pos).expect("error");
        assert_eq!(result, 55 as Byte);
    }

    #[test]
    fn set_point() {
        let axe_value: Axe = 10;
        let size = Size2D {
            width: axe_value,
            height: axe_value,
        };
        let mut arr = Array2D {
            size,
            bytes: vec![55 as Byte; 100],
        };
        let pos = Pos2D {
            x: 5 as Axe,
            y: 5 as Axe,
        };
        let result = arr.set_point(&pos, 99 as Byte);
        assert!(result.is_ok());

        let cell = arr.bytes[(arr.size.width * pos.y + pos.x).to_usize()];
        assert_eq!(cell, 99);
    }
}
