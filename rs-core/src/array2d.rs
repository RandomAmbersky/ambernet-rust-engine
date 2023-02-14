use crate::axe_dimension::UnsignedNum;
use crate::cell_type::CellType;
use crate::pos2d::Pos2D;
use crate::size2d::Size2D;

pub struct Array2D<S: UnsignedNum, T: CellType> {
    pub size: Size2D<S>,
    pub bytes: Vec<T>,
}

impl<S: UnsignedNum, T: CellType> Array2D<S, T> {
    fn _check_not_in_array(&self, pos: &Pos2D<S>) -> Result<(), String> {
        if self.size.is_pos_into(pos) {
            return Ok(());
        }
        let resp = format!(
            "Not in array {:?} x {} [{}, {}]",
            self.size.width, self.size.height, pos.x, pos.y
        );
        Err(resp)
    }

    pub fn get_point(&self, pos: &Pos2D<S>) -> Result<T, String> {
        self._check_not_in_array(pos)?;
        let index = self.size.get_index(pos)?;
        let value = self.bytes[index];
        Ok(value)
    }
    pub fn set_point(&mut self, pos: &Pos2D<S>, val: T) -> Result<(), String> {
        self._check_not_in_array(pos)?;
        let index = self.size.get_index(pos)?;
        self.bytes[index] = val;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::array2d::Array2D;
    use crate::axe_dimension::UnsignedNum;
    use crate::cell_type::CellType;
    use crate::pos2d::Pos2D;
    use crate::size2d::Size2D;

    type Axe = u32;
    impl UnsignedNum for Axe {
        fn to_usize(&self) -> usize {
            usize::try_from(*self).expect("convert error")
        }
    }

    type Cell = u8;
    impl CellType for Cell {}

    #[test]
    fn check_in_array() {
        let axe_value: Axe = 10;
        let arr = Array2D {
            size: Size2D {
                width: axe_value,
                height: axe_value,
            },
            bytes: vec![0 as Cell; 100],
        };

        let input = Pos2D {
            x: 5 as Axe,
            y: 5 as Axe,
        };
        let result = arr._check_not_in_array(&input);
        assert!(result.is_ok());

        let input = Pos2D {
            x: 100 as Axe,
            y: 110 as Axe,
        };
        let result = arr._check_not_in_array(&input);
        assert!(result.is_err())
    }

    #[test]
    fn get_point() {
        let axe_value: Axe = 10;
        let arr = Array2D {
            size: Size2D {
                width: axe_value,
                height: axe_value,
            },
            bytes: vec![55 as Cell; 100],
        };
        let pos = Pos2D {
            x: 5 as Axe,
            y: 5 as Axe,
        };
        let result = arr.get_point(&pos).expect("error");
        assert_eq!(result, 55 as Cell);
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
            bytes: vec![55 as Cell; 100],
        };
        let pos = Pos2D {
            x: 5 as Axe,
            y: 5 as Axe,
        };
        let result = arr.set_point(&pos, 99 as Cell);
        assert!(result.is_ok());

        let cell = arr.bytes[(arr.size.width * pos.y + pos.x).to_usize()];
        assert_eq!(cell, 99);
    }
}
