use std::fmt::{Debug, Display};
use std::ops::{Add, Mul};

// from 0 to MAX
pub trait UnsignedNum:
    Debug + Add<Output = Self> + Mul<Output = Self> + Display + Copy + PartialOrd
{
    fn to_usize(&self) -> usize;
}
