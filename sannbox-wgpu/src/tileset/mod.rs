use asn_core::math::Size2D;

pub struct AsnTileSet {
    pub tile_size: Size2D<u32>,
    pub tile_count: u32,
    pub columns: u32,
}

impl AsnTileSet {
    pub fn get_tile_size(&self) -> Size2D<u32> {
        self.tile_size
    }
}
