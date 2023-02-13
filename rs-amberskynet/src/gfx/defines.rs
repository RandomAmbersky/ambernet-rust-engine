use crate::core::{Array2D, Size2D};
use crate::core_gfx::pixel::PixelRGBA;

pub type AxeDimension = usize;
pub type ByteDimension = u8;

pub type Size2d = Size2D<AxeDimension>;

pub type BytesArray = Array2D<AxeDimension, ByteDimension>;

pub type Pixel = PixelRGBA<ByteDimension>;
