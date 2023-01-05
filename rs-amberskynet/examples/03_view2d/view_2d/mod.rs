mod model_vertex;

use model_vertex::ModelVertex;
use model_vertex::{INDICES, VERTICES};
use rs_amberskynet::gfx::{AsnTexture, BindGroupEntryBuilder, BindGroupLayoutBuilder};
use wgpu::util::DeviceExt;
use wgpu::{BindGroupLayout, CommandEncoder, Device, ShaderModule, TextureFormat, TextureView};

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
    pub fn draw(&self, encoder: &mut CommandEncoder, view: &TextureView) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(get_color_attachment(view))],
            depth_stencil_attachment: None,
        });
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }
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

fn get_color_attachment(view: &TextureView) -> wgpu::RenderPassColorAttachment {
    let clear_color = wgpu::Color {
        r: 0.1,
        g: 0.2,
        b: 0.3,
        a: 1.0,
    };
    wgpu::RenderPassColorAttachment {
        view,
        resolve_target: None,
        ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(clear_color),
            store: true,
        },
    }
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
