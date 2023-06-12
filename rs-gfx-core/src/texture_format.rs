#[derive(Debug, Clone, Copy)]
pub enum AsnTextureFormat {
    #[allow(dead_code)]
    Rgba8,
    Rgba16,
    Rgba32,
}

impl AsnTextureFormat {
    pub fn bytes_per_pixel(&self) -> u32 {
        match self {
            AsnTextureFormat::Rgba8 => 4,
            AsnTextureFormat::Rgba16 => 4 * 2,
            AsnTextureFormat::Rgba32 => 4 * 4,
        }
    }
}
