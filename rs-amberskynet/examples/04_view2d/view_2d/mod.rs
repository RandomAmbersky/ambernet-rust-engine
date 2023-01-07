mod mesh;
mod model_vertex;
mod view_screen;

use crate::view_2d::mesh::Mesh;
use crate::view_2d::view_screen::{Array2D, Size2D};
use model_vertex::ModelVertex;
use model_vertex::{INDICES, VERTICES};
use rs_amberskynet::gfx::{AsnTexture, BindGroupEntryBuilder, BindGroupLayoutBuilder};
use wgpu::{
    BindGroupLayout, CommandEncoder, Device, Queue, ShaderModule, TextureFormat, TextureView,
};

pub const SHADER_SOURCE: &str = include_str!("shader.wgsl");
const ONE_BLUE_PIXEL: [u8; 4] = [0, 0, 255, 255];

pub struct View2D {
    view: Array2D<u32, u8>,
    mesh: Mesh,
    bind_group: wgpu::BindGroup,
    render_pipeline: wgpu::RenderPipeline,
}

impl View2D {
    pub fn new(
        device: &Device,
        queue: &Queue,
        _texture: &AsnTexture,
        format: TextureFormat,
    ) -> Self {
        let mesh = Mesh::build(VERTICES, INDICES, device);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(SHADER_SOURCE.into()),
        });

        let group_layout_builder = BindGroupLayoutBuilder::new().texture().sampler();
        let group_layout_desc = wgpu::BindGroupLayoutDescriptor {
            entries: group_layout_builder.entries(),
            label: Some("texture_bind_group_layout"),
        };
        let diffuse_bind_group_layout = device.create_bind_group_layout(&group_layout_desc);
        let bind_group_layouts = &[&diffuse_bind_group_layout];

        let render_pipeline = get_render_pipeline(device, format, &shader, bind_group_layouts);

        let view = Array2D {
            size: Size2D {
                width: 1,
                height: 1,
            },
            bytes: ONE_BLUE_PIXEL.to_vec(),
        };

        let texture = AsnTexture::get_from_rgba(device, queue, None, &view.bytes).unwrap();

        let group_entry_builder = BindGroupEntryBuilder::default()
            .texture(&texture.view)
            .sampler(&texture.sampler);
        let group_desc = wgpu::BindGroupDescriptor {
            layout: &diffuse_bind_group_layout,
            entries: group_entry_builder.entries(),
            label: Some("diffuse_bind_group"),
        };
        let bind_group = device.create_bind_group(&group_desc);

        Self {
            view,
            mesh,
            bind_group,
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
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.mesh.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.mesh.num_indices, 0, 0..1);
    }
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
    bind_group_layouts: &[&BindGroupLayout],
) -> wgpu::RenderPipeline {
    let desc = wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts,
        push_constant_ranges: &[],
    };

    let render_pipeline_layout = device.create_pipeline_layout(&desc);

    let vertex_state = wgpu::VertexState {
        module: shader,
        entry_point: "vs_main",
        buffers: &[ModelVertex::desc()],
    };

    let target_state = wgpu::ColorTargetState {
        format,
        blend: Some(wgpu::BlendState {
            color: wgpu::BlendComponent::REPLACE,
            alpha: wgpu::BlendComponent::REPLACE,
        }),
        write_mask: wgpu::ColorWrites::ALL,
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
