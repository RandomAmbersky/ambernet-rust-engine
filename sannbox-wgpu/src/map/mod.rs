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
        if src_size.width != self.map.size.width {
            panic!(
                "Dimension width error! {:?} != {:?}",
                src_size.width, self.map.size.width
            )
        }
        if src_size.height != self.map.size.height {
            panic!(
                "Dimension height error! {:?} != {:?}",
                src_size.height, self.map.size.height
            )
        }
        let start = layer_index * self.map.size.width.as_usize() * self.map.size.height.as_usize();
        let end = start + src_size.get_size().as_usize();
        let range = Range { start, end };
        self.map.bytes[range].copy_from_slice(src);
    }
    pub fn get_row_ptr(&self, layer_index: usize, row_index: usize) -> &[CellDimension] {
        if self.map.size.height.as_usize() < row_index {
            panic!(
                "Dimension row_index error! {:?} < {:?}",
                self.map.size.height, row_index
            )
        }
        let start = self.calc_start(layer_index);
        let end = start + self.map.size.width.as_usize();
        let range = Range { start, end };
        &self.map.bytes[range]
    }
    fn calc_start(&self, layer_index: usize) -> usize {
        layer_index * self.map.size.width.as_usize() * self.map.size.height.as_usize()
    }
}

impl AsnMap {
    pub fn get_size(&self) -> Size3D<MapDimension> {
        self.map.size
    }
}
