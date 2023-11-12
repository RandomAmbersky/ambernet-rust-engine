use crate::math::cell_type::CellType;
use crate::math::{Pos3D, Size3D, UnsignedNum};
use std::mem;

#[allow(dead_code)]
pub struct Array3D<S: UnsignedNum, T: CellType> {
    pub size: Size3D<S>,
    pub bytes: Vec<T>,
}

impl<S: UnsignedNum, T: CellType> Array3D<S, T> {
    #[allow(dead_code)]
    pub fn new(s: &Size3D<S>) -> Self {
        let cell_size = mem::size_of::<T>();
        Self {
            size: *s,
            bytes: vec![T::ZERO; (s.get_size()).as_usize() * cell_size],
        }
    }

    fn check_not_in_array(&self, pos: &Pos3D<S>) -> Result<(), String> {
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
    pub fn get_size(&self) -> Result<Size3D<S>, String> {
        let size = self.size;
        Ok(size)
    }

    #[allow(dead_code)]
    pub fn get_point(&self, pos: &Pos3D<S>) -> Result<T, String> {
        self.check_not_in_array(pos)?;
        let index = self.size.get_index(pos)?;
        let value = self.bytes[index];
        Ok(value)
    }

    #[allow(dead_code)]
    pub fn set_point(&mut self, pos: &Pos3D<S>, val: T) -> Result<(), String> {
        self.check_not_in_array(pos)?;
        let index = self.size.get_index(pos)?;
        self.bytes[index] = val;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::math::{Array3D, Pos3D, Size3D};

    type Axe = u32;

    type Cell = u8;

    #[test]
    fn it_working() {
        let axe_value: Axe = 10;
        let arr: Array3D<Axe, Cell> = Array3D::new(&Size3D {
            width: axe_value,
            height: axe_value,
            depth: axe_value,
        });
        assert_eq!(arr.size.width, axe_value);
        assert_eq!(arr.size.height, axe_value);
        assert_eq!(
            arr.bytes.len(),
            (axe_value * axe_value * axe_value) as usize
        );
    }

    #[test]
    fn check_in_array() {
        let axe_value: Axe = 10;
        let arr: Array3D<Axe, Cell> = Array3D::new(&Size3D {
            width: axe_value,
            height: axe_value,
            depth: axe_value,
        });

        let input = Pos3D {
            x: 5 as Axe,
            y: 5 as Axe,
            z: 5 as Axe,
        };
        let result = arr.check_not_in_array(&input);
        assert!(result.is_ok());

        let input = Pos3D {
            x: 110 as Axe,
            y: 110 as Axe,
            z: 110 as Axe,
        };
        let result = arr.check_not_in_array(&input);
        assert!(result.is_err())
    }

    #[test]
    fn get_point() {
        let axe_value: Axe = 10_u32;
        let mut arr: Array3D<Axe, Cell> = Array3D::new(&Size3D {
            width: axe_value,
            height: axe_value,
            depth: axe_value,
        });

        let index = axe_value * axe_value * 5 + axe_value * 5 + 5;
        arr.bytes[index as usize] = 55;

        let pos = Pos3D {
            x: 5 as Axe,
            y: 5 as Axe,
            z: 5 as Axe,
        };

        let result = arr.get_point(&pos).expect("test error");
        assert_eq!(result, 55 as Cell);
    }

    #[test]
    fn set_point() {
        let axe_value: Axe = 10;
        let mut arr: Array3D<Axe, Cell> = Array3D::new(&Size3D {
            width: axe_value,
            height: axe_value,
            depth: axe_value,
        });
        let pos = Pos3D {
            x: 5 as Axe,
            y: 5 as Axe,
            z: 5 as Axe,
        };
        let result = arr.set_point(&pos, 99 as Cell);
        assert!(result.is_ok());

        // let cell = arr.bytes[(arr.size.width * pos.y + pos.x).as_usize()];
        // assert_eq!(cell, 99);
    }
}
