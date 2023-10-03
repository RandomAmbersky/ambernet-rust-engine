use asn_core::math::{Array3D, Size2D, Size3D, UnsignedNum};
use std::cell::Cell;
use std::ops::Range;

type CellDimension = u8;
type MapDimension = u32;

// layers: (TODO)
// Ground
// Walls
// items

pub struct AsnMap {
    map: Array3D<MapDimension, CellDimension>,
}

impl AsnMap {
    pub fn new(s: &Size3D<MapDimension>) -> Self {
        let map = Array3D::new(s);
        Self { map }
    }

    pub fn copy_layer(
        &mut self,
        layer_index: usize,
        src_size: &Size2D<MapDimension>,
        src: &[CellDimension],
    ) {
        let start = layer_index * self.map.size.width.as_usize() * self.map.size.height.as_usize();
        let end = src_size.get_size().as_usize();
        let range = Range { start, end };
        self.map.bytes[range].copy_from_slice(src);
    }
}
