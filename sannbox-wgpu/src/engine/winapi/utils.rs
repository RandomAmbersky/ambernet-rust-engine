use crate::engine::core::winapi::AsnTextureFormat;
use wgpu::TextureFormat;

pub trait ToWgpuFormat {
    fn to_wgpu_format(self) -> wgpu::TextureFormat;
    fn get_from(f: wgpu::TextureFormat) -> Self;
}

impl ToWgpuFormat for AsnTextureFormat {
    fn to_wgpu_format(self) -> wgpu::TextureFormat {
        match self {
            AsnTextureFormat::Rgba8 => wgpu::TextureFormat::Rgba8UnormSrgb,
            AsnTextureFormat::Rgba16 => wgpu::TextureFormat::Rgba16Unorm,
            AsnTextureFormat::Rgba32 => wgpu::TextureFormat::Rgba32Uint,
            AsnTextureFormat::Bgra8 => wgpu::TextureFormat::Bgra8UnormSrgb,
        }
    }
    fn get_from(f: wgpu::TextureFormat) -> Self {
        match f {
            TextureFormat::Rgba8UnormSrgb => AsnTextureFormat::Rgba8,
            TextureFormat::Rgba16Unorm => AsnTextureFormat::Rgba8,
            TextureFormat::Rgba32Uint => AsnTextureFormat::Rgba8,
            TextureFormat::Bgra8UnormSrgb => AsnTextureFormat::Bgra8,
            _ => {
                panic!("Can't convert {:?} ", f);
            }
        }
    }
}
