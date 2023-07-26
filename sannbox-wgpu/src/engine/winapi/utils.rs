use crate::engine::core::winapi::AsnTextureFormat;

pub trait ToWgpuFormat {
    fn to_wgpu_format(self) -> wgpu::TextureFormat;
}

impl ToWgpuFormat for AsnTextureFormat {
    fn to_wgpu_format(self) -> wgpu::TextureFormat {
        match self {
            AsnTextureFormat::Rgba8 => wgpu::TextureFormat::Rgba8UnormSrgb,
            AsnTextureFormat::Rgba16 => wgpu::TextureFormat::Rgba16Unorm,
            AsnTextureFormat::Rgba32 => wgpu::TextureFormat::Rgba32Uint,
        }
    }
}
