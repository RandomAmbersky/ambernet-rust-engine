use asn_core::math::{Array3D, Pos2D, Size2D, Size3D, UnsignedNum};
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
    fn calc_start(&self, layer_index: usize) -> usize {
        layer_index * self.map.size.width.as_usize() * self.map.size.height.as_usize()
    }
}

#[allow(dead_code)]
impl AsnMap {
    pub fn get_size(&self) -> Size3D<MapDimension> {
        self.map.size
    }
    pub fn get_size_2d(&self) -> Size2D<MapDimension> {
        Size2D {
            width: self.map.size.width,
            height: self.map.size.height,
        }
    }
    pub fn get_cell(&self, layer_index: usize, pos: &Pos2D<MapDimension>) -> CellDimension {
        let index = self.calc_start(layer_index) + (pos.y * self.map.size.width + pos.x).as_usize();
        self.map.bytes[index]
    }
    pub fn set_cell(&mut self, layer_index: usize, pos: &Pos2D<MapDimension>, cell: u8) {
        let index = self.calc_start(layer_index) + (pos.y * self.map.size.width + pos.x).as_usize();
        self.map.bytes[index] = cell
    }
}
