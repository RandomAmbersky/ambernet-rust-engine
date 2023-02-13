use crate::cell_type::CellType;
use std::fmt::{Debug, Display};
use std::ops::{Add, Mul};
use std::slice::SliceIndex;

pub trait AxeDimension:
    Debug
    + Add<Output = Self>
    + Mul<Output = Self>
    + Display
    + Copy
    + PartialOrd
    + SliceIndex<Self, Output = Self>
{
}
