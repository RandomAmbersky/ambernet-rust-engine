use rs_core::{Array2D, CellType, Size2D, UnsignedNum};
use rs_core_gfx::PixelRGBA;

// pub struct MyAxeDimension(usize);
// pub struct MyByteDimension(u8);

// impl UnsignedNum for MyAxeDimension {
// 	fn as_usize(&self) -> usize {
// 		todo!()
// 	}
// };

// impl UnsignedNum for MyAxeDimension {
// 	fn as_usize(&self) -> usize {
// 		usize::try_from(*self).expect("convert error")
// 	}
// }

pub type Size2d = Size2D<u32>;

pub type BytesArray = Array2D<u32, u8>;

pub type Pixel = PixelRGBA<u8>;
