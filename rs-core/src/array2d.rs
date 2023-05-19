use crate::{CellType, Pos2D, Size2D, UnsignedNum};

#[allow(dead_code)]
pub struct Array2D<S: UnsignedNum, T: CellType> {
    pub size: Size2D<S>,
    pub bytes: Vec<T>,
}

impl<S: UnsignedNum, T: CellType> Array2D<S, T> {
    #[allow(dead_code)]
    fn new(width: S, height: S) -> Self {
        Self {
            size: Size2D { width, height },
            bytes: vec![T::ZERO; (width * height).to_usize()],
        }
    }

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

    #[allow(dead_code)]
    pub fn get_point(&self, pos: &Pos2D<S>) -> Result<T, String> {
        self._check_not_in_array(pos)?;
        let index = self.size.get_index(pos)?;
        let value = self.bytes[index];
        Ok(value)
    }

    #[allow(dead_code)]
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
    use crate::cell_type::CellType;
    use crate::pos2d::Pos2D;
    use crate::unsigned_num::UnsignedNum;

    type Axe = u32;
    impl UnsignedNum for Axe {
        fn to_usize(&self) -> usize {
            usize::try_from(*self).expect("convert error")
        }
    }

    type Cell = u8;
    impl CellType for Cell {
        const ZERO: Self = 0 as Cell;
    }

    #[test]
    fn it_working() {
        let axe_value: Axe = 10;
        let arr: Array2D<Axe, Cell> = Array2D::new(axe_value, axe_value);
        assert_eq!(arr.size.width, axe_value);
        assert_eq!(arr.size.height, axe_value);
        assert_eq!(arr.bytes.len(), (axe_value * axe_value) as usize);
    }

    #[test]
    fn check_in_array() {
        let axe_value: Axe = 10;
        let arr: Array2D<Axe, Cell> = Array2D::new(axe_value, axe_value);

        let input = Pos2D {
            x: 5 as Axe,
            y: 5 as Axe,
        };
        let result = arr._check_not_in_array(&input);
        assert!(result.is_ok());

        let input = Pos2D {
            x: 110 as Axe,
            y: 110 as Axe,
        };
        let result = arr._check_not_in_array(&input);
        assert!(result.is_err())
    }

    #[test]
    fn get_point() {
        let axe_value: Axe = 10;
        let mut arr: Array2D<Axe, Cell> = Array2D::new(axe_value, axe_value);

        arr.bytes[(10 * 5 + 5).to_usize()] = 55;

        let pos = Pos2D {
            x: 5 as Axe,
            y: 5 as Axe,
        };

        let result = arr.get_point(&pos).expect("test error");
        assert_eq!(result, 55 as Cell);
    }

    #[test]
    fn set_point() {
        let axe_value: Axe = 10;
        let mut arr: Array2D<Axe, Cell> = Array2D::new(axe_value, axe_value);
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
