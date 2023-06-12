pub enum AsnTextureFormat {
    #[allow(dead_code)]
    Rgb8,
    Rgb16,
    Rgb32,
    Rgba8,
    Rgba16,
    Rgba32,
}

impl AsnTextureFormat {
    pub fn bytes_per_pixel(&self) -> u8 {
        match self {
            AsnTextureFormat::Rgb8 => 3,
            AsnTextureFormat::Rgb16 => 3 * 2,
            AsnTextureFormat::Rgb32 => 3 * 4,
            AsnTextureFormat::Rgba8 => 4,
            AsnTextureFormat::Rgba16 => 4 * 2,
            AsnTextureFormat::Rgba32 => 4 * 4,
        }
    }
}
