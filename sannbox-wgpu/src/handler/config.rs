use asn_core::math::Size2D;

pub struct AsnGameConfig {
    pub view_size: Size2D<u32>,
}

pub fn get_config() -> AsnGameConfig {
    AsnGameConfig {
        view_size: Size2D {
            width: 10,
            height: 10,
        },
    }
}
