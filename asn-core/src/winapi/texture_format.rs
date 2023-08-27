#[derive(Default, Debug, Clone, Copy)]
pub enum AsnTextureFormat {
    #[default]
    Rgba8,
    Rgba16,
    Rgba32,
    Bgra8UnormSrgb,
}

impl AsnTextureFormat {
    pub fn bytes_per_pixel(&self) -> u32 {
        match self {
            AsnTextureFormat::Bgra8UnormSrgb => 4,
            AsnTextureFormat::Rgba8 => 4,
            AsnTextureFormat::Rgba16 => 4 * 2,
            AsnTextureFormat::Rgba32 => 4 * 4,
        }
    }
}
