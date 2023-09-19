use asn_core::math::Size2D;

type CellDimension = u8;
type MapDimension = u32;

// layers: (TODO)
// Ground
// Walls
// items

pub struct AsnLayer {
    pub name: String,
    pub size: Size2D<MapDimension>,
    pub bytes: Vec<CellDimension>,
}

pub struct AsnMap {
    size: Size2D<MapDimension>,
    layers: Vec<AsnLayer>,
}

impl AsnMap {
    pub fn new(s: &Size2D<MapDimension>) -> Self {
        Self {
            size: *s,
            layers: vec![],
        }
    }
}
