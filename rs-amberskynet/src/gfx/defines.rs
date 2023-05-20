use crate::core_gfx::pixel::PixelRGBA;
use rs_core::{Array2D, Size2D};

pub type MyAxeDimension = usize;
pub type MyByteDimension = u8;

pub type Size2d = Size2D<MyAxeDimension>;

pub type BytesArray = Array2D<MyAxeDimension, MyByteDimension>;

pub type Pixel = PixelRGBA<MyByteDimension>;
