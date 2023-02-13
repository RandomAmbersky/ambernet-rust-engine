use crate::axe_dimension::AxeDimension;
use crate::cell_type::CellType;
use crate::pos2d::Pos2D;
use crate::size2d::Size2D;
use std::slice::SliceIndex;

pub struct Array2D<S: AxeDimension, T: CellType> {
    pub size: Size2D<S>,
    pub bytes: Vec<T>,
}

impl<S: AxeDimension + SliceIndex<[T], Output = T>, T: CellType> Array2D<S, T> {
    pub fn check_in_array(&mut self, pos: &Pos2D<S>) -> bool {
        if self.size.width > pos.x {
            println!("{:?} {}", self.size.width, pos.x);
            return false;
        }
        if self.size.height > pos.y {
            return false;
        }
        true
        // println!("{}", self.size.width < pos.x);
        // (self.size.width > pos.x) && (self.size.height > pos.y)
    }

    pub fn get_point(&mut self, pos: &Pos2D<S>) -> Result<T, String> {
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
            return Err("Not in array".parse().unwrap());
        }
        let index = self.size.get_index(pos);
        self.bytes[index] = val;
        Ok(())
    }
}
