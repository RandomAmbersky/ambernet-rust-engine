use asn_core::math::{Size3D, UnsignedNum};

type CellDimension = u8;
type MapDimension = u32;

// layers: (TODO)
// Ground
// Walls
// items

pub struct AsnMap {
    size: Size3D<MapDimension>,
    layers: Vec<CellDimension>,
}

impl AsnMap {
    pub fn new(s: &Size3D<MapDimension>) -> Self {
        let map_size: usize = (s.width * s.height * s.depth) as usize;
        Self {
            size: *s,
            layers: vec![0; map_size],
        }
    }
}
