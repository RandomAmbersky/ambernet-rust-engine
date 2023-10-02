use asn_core::math::{Array3D, Size3D, UnsignedNum};

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
}
