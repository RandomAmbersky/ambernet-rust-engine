use std::fmt::{Debug, Display};
use std::ops::{Add, Mul};

pub trait AxeDimension:
    Debug + Add<Output = Self> + Mul<Output = Self> + Display + Copy + PartialOrd
{
    fn to_usize(&self) -> usize;
}
