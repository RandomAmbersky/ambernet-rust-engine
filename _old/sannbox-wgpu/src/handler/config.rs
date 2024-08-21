use asn_core::math::Size2D;

pub struct AsnGameConfig {
    pub view_size: Size2D<u32>,
    pub player_cell: u8,
}

pub fn get_config() -> AsnGameConfig {
    AsnGameConfig {
        view_size: Size2D {
            width: 15,
            height: 15,
        },
        player_cell: 2 * 16 + 14,
    }
}
