use rs_core::{Array2D, Size2D};
use rs_gfx_core::PixelRGBA;

pub type SizeDimension = u32;
pub type ByteDimension = u8;

pub type Size2d = Size2D<SizeDimension>;
pub type BytesArray = Array2D<SizeDimension, ByteDimension>;
pub type Pixel = PixelRGBA<ByteDimension>;
