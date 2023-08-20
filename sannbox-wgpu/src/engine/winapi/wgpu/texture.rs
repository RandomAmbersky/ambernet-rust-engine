use crate::engine::core::errors::AsnRenderError;
use crate::engine::core::winapi::{AsnTextureFormat, TTexture};
use crate::engine::winapi::defines::BytesArray;
use crate::engine::winapi::utils::ToWgpuFormat;
use crate::engine::winapi::wgpu::AsnWgpuWinApi;
use image::{DynamicImage, GenericImageView};
use wgpu::{Sampler, Texture, TextureFormat, TextureView};
use crate::engine::core::math::Size2D;

pub struct AsnTexture {
    pub texture_format: AsnTextureFormat,
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Drop for AsnTexture {
    fn drop(&mut self) {
        println!("Drop AsnTexture");
        // self.texture.destroy();
    }
}

fn create_texture(
    device: &wgpu::Device,
    size: wgpu::Extent3d,
    texture_format: TextureFormat
) -> wgpu::Texture {
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: None,
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: texture_format,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    texture
}
fn write_texure (queue: &wgpu::Queue, texture: &wgpu::Texture, bytes: &[u8], size: &wgpu::Extent3d) {
    queue.write_texture(
        wgpu::ImageCopyTexture {
            aspect: wgpu::TextureAspect::All,
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
        },
        &bytes,
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(4 * size.width),
            rows_per_image: Some(size.height),
        },
        *size,
    );
}
fn create_view (texture: &Texture) -> wgpu::TextureView {
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    view
}
fn create_sampler (device: &wgpu::Device,) -> wgpu::Sampler {
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Nearest,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    });
    sampler
}

fn create_texture_set(
    gfx: &AsnWgpuWinApi,
    bytes: &[u8],
    size: wgpu::Extent3d,
    texture_format: TextureFormat,
) -> (Texture, TextureView, Sampler) {
    let texture = create_texture(&gfx.device, size, texture_format);
    write_texure(&gfx.queue, &texture, bytes, &size);
    let view = create_view(&texture);
    let sampler = create_sampler(&gfx.device);
    (texture, view, sampler)
}

impl TTexture for AsnTexture {
    type WinApi = AsnWgpuWinApi;
    type AsnTexture = AsnTexture;

    fn from_memory(gfx: &Self::WinApi, bytes: &[u8], f: AsnTextureFormat) -> Result<Self, AsnRenderError> {
        let img = image::load_from_memory(bytes);
        match img {
            Err(_) => Err(AsnRenderError::CustomError("image import faut".into())),
            Ok(t) => Self::from_image(gfx, &t, f),
        }
    }

    fn from_raw(gfx: &Self::WinApi, bytes: &[u8], size: Size2D<u32>, f: AsnTextureFormat) -> Result<Self::AsnTexture, AsnRenderError> {
        let texture_size = wgpu::Extent3d {
            width: size.width,
            height: size.height,
            depth_or_array_layers: 1,
        };
        let (texture, view, sampler) = create_texture_set(
            gfx,
            &bytes,
            texture_size,
            f.to_wgpu_format(),
        );
        Ok(Self {
            texture_format: f,
            texture,
            view,
            sampler,
        })
    }

    fn update_from_raw(&mut self, gfx: &Self::WinApi, bytes: &[u8], size: &Size2D<u32>) -> Result<(), AsnRenderError> {
        let texture_size = wgpu::Extent3d {
            width: size.width,
            height: size.height,
            depth_or_array_layers: 1,
        };

        let (texture, view, sampler) = create_texture_set(
            gfx,
            &bytes,
            texture_size,
            self.texture_format.to_wgpu_format(),
        );

        self.texture = texture;
        self.view = view;
        self.sampler = sampler;
        Ok(())
    }
}

impl AsnTexture {
    fn from_image(
        gfx: &AsnWgpuWinApi,
        img: &DynamicImage,
        f: AsnTextureFormat,
    ) -> Result<Self, AsnRenderError> {
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let (texture, view, sampler) = create_texture_set(gfx, &rgba, size, f.to_wgpu_format());
        Ok(Self {
            texture_format: f,
            texture,
            view,
            sampler,
        })
    }

    // pub fn update_from_array(
    //     &mut self,
    //     gfx: &AsnWgpuWinApi,
    //     array: &BytesArray,
    // ) -> Result<(), AsnRenderError> {
    //     let dimensions: (u32, u32) = (array.size.width, array.size.height);
    //     let size = wgpu::Extent3d {
    //         width: dimensions.0,
    //         height: dimensions.1,
    //         depth_or_array_layers: 1,
    //     };
    //
    //     let f = self.texture_format;
    //
    //     if self.texture.size() != size {
    //         let (texture, view, sampler) =
    //             create_texture_set(gfx, &array.bytes, size, f.to_wgpu_format());
    //         self.texture = texture;
    //         self.view = view;
    //         self.sampler = sampler;
    //     } else {
    //         gfx.queue.write_texture(
    //             wgpu::ImageCopyTexture {
    //                 aspect: wgpu::TextureAspect::All,
    //                 texture: &self.texture,
    //                 mip_level: 0,
    //                 origin: wgpu::Origin3d::ZERO,
    //             },
    //             &array.bytes,
    //             wgpu::ImageDataLayout {
    //                 offset: 0,
    //                 bytes_per_row: Some(4 * dimensions.0),
    //                 rows_per_image: Some(dimensions.1),
    //             },
    //             size,
    //         );
    //     }
    //     Ok(())
    // }
}
