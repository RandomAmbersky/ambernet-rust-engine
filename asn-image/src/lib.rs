use image::GenericImageView;
use asn_core::math::{Array2D, Size2D};

pub fn load_texture(bytes: &[u8]) -> Array2D<u32, u8> {
    let img = image::load_from_memory(bytes).unwrap();
    let rgba = img.to_rgba8();
    let dimensions = img.dimensions();
    let size = Size2D::<u32> {
        width: dimensions.0,
        height: dimensions.1,
    };
    Array2D {
        size,
        bytes: rgba.to_vec(),
    }
}
