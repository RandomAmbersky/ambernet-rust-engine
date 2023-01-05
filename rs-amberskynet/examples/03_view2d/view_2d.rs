use crate::gfx_config::get_render_pipeline;
use crate::resource::{INDICES, VERTICES};
use rs_amberskynet::gfx::{AsnTexture, BindGroupEntryBuilder, BindGroupLayoutBuilder};
use wgpu::util::DeviceExt;
use wgpu::{BindGroupLayout, Device, ShaderModule, TextureFormat};

pub struct View2D {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
    pub diffuse_bind_group: wgpu::BindGroup,
    pub render_pipeline: wgpu::RenderPipeline,
}

impl View2D {
    pub fn new(
        device: &Device,
        texture: &AsnTexture,
        format: TextureFormat,
        shader: &ShaderModule,
    ) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
        let num_indices = INDICES.len() as u32;

        let group_layout = get_bind_group_layout(device);
        let diffuse_bind_group = get_bind_group(device, texture, &group_layout);

        let render_pipeline = get_render_pipeline(&device, format, &shader, &group_layout);

        Self {
            vertex_buffer,
            index_buffer,
            num_indices,
            diffuse_bind_group,
            render_pipeline,
        }
    }
    pub fn draw(&self) {}
}

fn get_bind_group_layout(device: &Device) -> BindGroupLayout {
    let entries = BindGroupLayoutBuilder::new().texture().sampler();
    let desc = wgpu::BindGroupLayoutDescriptor {
        entries: entries.entries(),
        label: Some("texture_bind_group_layout"),
    };
    device.create_bind_group_layout(&desc)
}

fn get_bind_group(
    device: &Device,
    texture: &AsnTexture,
    layout: &BindGroupLayout,
) -> wgpu::BindGroup {
    let entries = BindGroupEntryBuilder::new()
        .texture(&texture.view)
        .sampler(&texture.sampler);

    let desc = wgpu::BindGroupDescriptor {
        layout,
        entries: entries.entries(),
        label: Some("diffuse_bind_group"),
    };
    device.create_bind_group(&desc)
}
