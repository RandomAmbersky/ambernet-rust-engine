use crate::engine::core::math::{Array2D, Pos2D, Size2D};
use crate::engine::core::winapi::PixelRGBA;

pub type SizeDimension = u32;
pub type ByteDimension = u8;

pub type CellSize = u8;

pub type Position = Pos2D<SizeDimension>;
pub type Size2d = Size2D<SizeDimension>;
pub type BytesArray = Array2D<SizeDimension, ByteDimension>;
pub type Pixel = PixelRGBA<ByteDimension>;
