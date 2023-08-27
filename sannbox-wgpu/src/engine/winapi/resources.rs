use asn_core::math::Size2D;
use asn_core::winapi::AsnTextureFormat;

pub struct DefaultTextureData {
    pub bytes: [u8; 4],
    pub size: Size2D<u32>,
    pub texture_format: AsnTextureFormat,
}

pub const ONE_BLUE_PIXEL: DefaultTextureData = DefaultTextureData {
    bytes: [0, 0, 255, 255],
    size: Size2D {
        width: 1,
        height: 1,
    },
    texture_format: AsnTextureFormat::Rgba8,
};
