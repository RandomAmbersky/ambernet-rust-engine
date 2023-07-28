use crate::engine::core::errors::AsnRenderError;
use crate::engine::core::winapi::AsnTextureFormat;
use crate::engine::winapi::resources::ONE_BLUE_PIXEL;
use crate::engine::winapi::utils::ToWgpuFormat;
use crate::engine::winapi::wgpu::defines::BytesArray;
use crate::engine::winapi::wgpu::AsnWgpuWinApi;
use image::{DynamicImage, GenericImageView};

pub struct AsnTexture {
    pub texture_format: AsnTextureFormat,
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl AsnTexture {
    pub fn new(gfx: &AsnWgpuWinApi) -> Self {
        let base_texture_data = ONE_BLUE_PIXEL;

        let texture_format = base_texture_data.texture_format;
        let bpp = texture_format.bytes_per_pixel();

        let dimensions: (u32, u32) = (base_texture_data.size.width, base_texture_data.size.height);
        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture = gfx.device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: texture_format.to_wgpu_format(),
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        gfx.queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &base_texture_data.bytes,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = gfx.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Self {
            texture_format,
            texture,
            view,
            sampler,
        }
    }
    pub fn update_from_raw_image(
        &mut self,
        gfx: &AsnWgpuWinApi,
        bytes: &[u8],
        f: AsnTextureFormat,
    ) -> Result<(), AsnRenderError> {
        let img = image::load_from_memory(bytes);
        match img {
            Err(_) => Err(AsnRenderError::CustomError("image import faut".into())),
            Ok(t) => Self::update_from_image(self, gfx, &t, f),
        }
    }
    fn update_from_image(
        &mut self,
        gfx: &AsnWgpuWinApi,
        img: &DynamicImage,
        f: AsnTextureFormat,
    ) -> Result<(), AsnRenderError> {
        let texture_format = f.to_wgpu_format();

        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        // let texture = gfx.device.create_texture(&wgpu::TextureDescriptor {
        //     label: None,
        //     size,
        //     mip_level_count: 1,
        //     sample_count: 1,
        //     dimension: wgpu::TextureDimension::D2,
        //     format: texture_format,
        //     usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        //     view_formats: &[],
        // });

        gfx.queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            size,
        );

        // let view = selftexture.create_view(&wgpu::TextureViewDescriptor::default());
        // let sampler = gfx.device.create_sampler(&wgpu::SamplerDescriptor {
        //     address_mode_u: wgpu::AddressMode::ClampToEdge,
        //     address_mode_v: wgpu::AddressMode::ClampToEdge,
        //     address_mode_w: wgpu::AddressMode::ClampToEdge,
        //     mag_filter: wgpu::FilterMode::Nearest,
        //     min_filter: wgpu::FilterMode::Nearest,
        //     mipmap_filter: wgpu::FilterMode::Nearest,
        //     ..Default::default()
        // });
        //
        // Ok(Self {
        //     texture_format: f,
        //     texture,
        //     view,
        //     sampler,
        // })
        Ok(())
    }

    fn update_from_array(
        &mut self,
        gfx: &AsnWgpuWinApi,
        array: &BytesArray,
    ) -> Result<(), AsnRenderError> {
        let dimensions: (u32, u32) = (array.size.width, array.size.height);
        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        gfx.queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &array.bytes,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            size,
        );
        Ok(())
    }
}
