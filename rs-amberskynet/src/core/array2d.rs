use crate::core::size2d::Size2D;
use crate::core::Pos2D;
use std::ops::{Add, Index, Mul};
use std::slice::SliceIndex;

pub struct Array2D<S, T>
where
    S: Sized
        + Copy
        + Mul<S, Output = S>
        + Add<S, Output = S>
        + PartialOrd
        + SliceIndex<[T], Output = T>,
    T: Copy,
{
    pub size: Size2D<S>,
    pub bytes: Vec<T>,
}

impl<S, T> Array2D<S, T>
where
    S: Copy + Mul<S, Output = S> + Add<S, Output = S> + PartialOrd + SliceIndex<[T], Output = T>,
    T: Copy,
{
    pub fn check_in_array(&mut self, pos: &Pos2D<S>) -> bool {
        self.size.width < pos.x && self.size.height < pos.y
    }

    pub fn get_point(&mut self, pos: &Pos2D<S>) -> Result<T, String> {
        if !self.check_in_array(pos) {
            return Err("Not in array".parse().unwrap());
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
