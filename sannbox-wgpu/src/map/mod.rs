use asn_core::math::Size2D;

// layers: (TODO)
// Ground
// Walls
// items

pub struct AsnLayer {
    pub name: String,
    pub size: Size2D<u32>,
    pub bytes: Vec<u8>,
}

pub struct AsnMap {
    size: Size2D<u32>,
    layers: Vec<AsnLayer>,
}
