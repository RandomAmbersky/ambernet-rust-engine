use rs_amberskynet::gfx::AsnTexture;
use std::num::NonZeroU32;
use std::os::unix::raw::dev_t;
use wgpu::{Device, Label, Queue};

pub struct Size2D<T> {
    pub width: T,
    pub height: T,
}

pub struct Array2D<S, T> {
    pub size: Size2D<S>,
    pub bytes: Vec<T>,
}

pub fn update_texture(texture: &mut AsnTexture, queue: &Queue, array: &Array2D<u32, u8>) {
    let dimensions: (u32, u32) = (array.size.width, array.size.height);
    let size = wgpu::Extent3d {
        width: dimensions.0,
        height: dimensions.1,
        depth_or_array_layers: 1,
    };

    queue.write_texture(
        wgpu::ImageCopyTexture {
            aspect: wgpu::TextureAspect::All,
            texture: &texture.texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
        },
        &array.bytes,
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: NonZeroU32::new(4 * dimensions.0),
            rows_per_image: NonZeroU32::new(dimensions.1),
        },
        size,
    );
}

pub fn array_to_texture(device: &Device, queue: &Queue, array: &Array2D<u32, u8>) -> AsnTexture {
    let dimensions: (u32, u32) = (array.size.width, array.size.height);

    let size = wgpu::Extent3d {
        width: dimensions.0,
        height: dimensions.1,
        depth_or_array_layers: 1,
    };

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: None,
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
    });

    queue.write_texture(
        wgpu::ImageCopyTexture {
            aspect: wgpu::TextureAspect::All,
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
        },
        &array.bytes,
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: NonZeroU32::new(4 * dimensions.0),
            rows_per_image: NonZeroU32::new(dimensions.1),
        },
        size,
    );

    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Nearest,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    });

    AsnTexture {
        texture,
        view,
        sampler,
    }
}
