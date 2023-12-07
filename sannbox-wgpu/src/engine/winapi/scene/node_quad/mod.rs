mod resource;

use crate::engine::winapi::resources::ONE_BLUE_PIXEL;
use crate::engine::winapi::scene::node_quad::resource::{Vertex, INDICES, SHADER_SOURCE, VERTICES};
use crate::engine::winapi::utils::ToWgpuFormat;
use crate::engine::winapi::wgpu::texture::AsnTexture;
use crate::engine::winapi::wgpu::{AsnWgpuFrameContext, AsnWgpuWinApi};
use asn_core::errors::AsnRenderError;
use asn_core::winapi::scene::{TNodeBase, TNodeQuad};
use asn_core::winapi::{TAsnWinapi, TTexture};
use std::sync::Arc;
use wgpu::util::DeviceExt;
use wgpu::{BindGroup, RenderPipeline, ShaderModule, StoreOp, TextureFormat};

pub struct AsnWgpuNodeQuad {
    shader: ShaderModule,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    diffuse_bind_group: wgpu::BindGroup,
    render_pipeline: wgpu::RenderPipeline,
    // texture: Arc<AsnTexture>,
}

fn create_node_quad_set(
    gfx: &mut AsnWgpuWinApi,
    texture: &AsnTexture,
    texture_format: TextureFormat,
    shader: &ShaderModule,
) -> (RenderPipeline, BindGroup) {
    let texture_bind_group_layout =
        gfx.get_device()
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });
    let diffuse_bind_group = gfx
        .get_device()
        .create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        });
    let render_pipeline_layout =
        gfx.get_device()
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout],
                push_constant_ranges: &[],
            });
    let render_pipeline =
        gfx.get_device()
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[Vertex::desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: texture_format,
                        blend: Some(wgpu::BlendState {
                            color: wgpu::BlendComponent::REPLACE,
                            alpha: wgpu::BlendComponent::REPLACE,
                        }),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState {
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
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                // If the pipeline will be used with a multiview render pass, this
                // indicates how many array layers the attachments will have.
                multiview: None,
            });

    (render_pipeline, diffuse_bind_group)
}

impl AsnWgpuNodeQuad {
    pub fn new(gfx: &mut AsnWgpuWinApi) -> Self {
        let texture_format = gfx.get_config().texture_format.to_wgpu_format();
        // println!("texure format: {:?}", texture_format);

        let shader = gfx
            .get_device()
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Shader"),
                source: wgpu::ShaderSource::Wgsl(SHADER_SOURCE.into()),
            });

        let vertex_buffer =
            gfx.get_device()
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(VERTICES),
                    usage: wgpu::BufferUsages::VERTEX,
                });

        let index_buffer = gfx
            .get_device()
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(INDICES),
                usage: wgpu::BufferUsages::INDEX,
            });
        let num_indices = INDICES.len() as u32;

        let texture = AsnTexture::from_raw(
            gfx,
            &ONE_BLUE_PIXEL.bytes,
            &ONE_BLUE_PIXEL.size,
            ONE_BLUE_PIXEL.texture_format,
        )
        .unwrap();
        let arc_texture = Arc::new(texture);

        let (render_pipeline, diffuse_bind_group) =
            create_node_quad_set(gfx, &arc_texture, texture_format, &shader);

        Self {
            shader,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            diffuse_bind_group,
        }
    }
}

impl TNodeBase for AsnWgpuNodeQuad {
    type FrameContext = AsnWgpuFrameContext;
    type WinApi = AsnWgpuWinApi;
    type AsnTexture = AsnTexture;

    fn draw(&mut self, fcx: &mut Self::FrameContext) {
        let mut render_pass = fcx.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &fcx.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }
    fn update(&mut self, gfx: &mut Self::WinApi) {
        todo!()
    }
}

impl TNodeQuad for AsnWgpuNodeQuad {
    fn set_texture(
        &mut self,
        gfx: &mut Self::WinApi,
        texture: &Self::AsnTexture,
    ) -> Result<(), AsnRenderError> {
        // println!("AsnWgpuNodeQuad set_texture");

        let texture_format = gfx.get_config().texture_format.to_wgpu_format();
        let (render_pipeline, diffuse_bind_group) =
            create_node_quad_set(gfx, texture, texture_format, &self.shader);

        self.render_pipeline = render_pipeline;
        self.diffuse_bind_group = diffuse_bind_group;
        Ok(())
    }
}
