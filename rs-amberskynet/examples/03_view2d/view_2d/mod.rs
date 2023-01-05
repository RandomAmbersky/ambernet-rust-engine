mod model_vertex;

use crate::view_2d::model_vertex::{INDICES, VERTICES};
use model_vertex::ModelVertex;
use rs_amberskynet::gfx::{AsnTexture, BindGroupEntryBuilder, BindGroupLayoutBuilder, Vertex};
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

        let render_pipeline = get_render_pipeline(device, format, shader, &group_layout);

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

pub fn get_render_pipeline(
    device: &Device,
    format: TextureFormat,
    shader: &ShaderModule,
    texture_bind_group_layout: &BindGroupLayout,
) -> wgpu::RenderPipeline {
    let desc = wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[texture_bind_group_layout],
        push_constant_ranges: &[],
    };

    let render_pipeline_layout = device.create_pipeline_layout(&desc);

    let target_state = wgpu::ColorTargetState {
        format,
        blend: Some(wgpu::BlendState {
            color: wgpu::BlendComponent::REPLACE,
            alpha: wgpu::BlendComponent::REPLACE,
        }),
        write_mask: wgpu::ColorWrites::ALL,
    };

    let vertex_state = wgpu::VertexState {
        module: shader,
        entry_point: "vs_main",
        buffers: &[ModelVertex::desc()],
    };

    let fragment_state = wgpu::FragmentState {
        module: shader,
        entry_point: "fs_main",
        targets: &[Some(target_state)],
    };

    let primitive = wgpu::PrimitiveState {
        topology: wgpu::PrimitiveTopology::TriangleList,
        strip_index_format: None,
        front_face: wgpu::FrontFace::Ccw,
        cull_mode: Some(wgpu::Face::Back),
        // Setting this to anything other than Fill requires Features::POLYGON_MODE_LINE
        // or Features::POLYGON_MODE_POINT
        polygon_mode: wgpu::PolygonMode::Fill,
        // Requires Features::DEPTH_CLIP_CONTROL
        unclipped_depth: false,
        // Requires Features::CONSERVATIVE_RASTERIZATION
        conservative: false,
    };

    let multisample = wgpu::MultisampleState {
        count: 1,
        mask: !0,
        alpha_to_coverage_enabled: false,
    };

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: vertex_state,
        fragment: Some(fragment_state),
        primitive,
        depth_stencil: None,
        multisample,
        // If the pipeline will be used with a multiview render pass, this
        // indicates how many array layers the attachments will have.
        multiview: None,
    })
}
