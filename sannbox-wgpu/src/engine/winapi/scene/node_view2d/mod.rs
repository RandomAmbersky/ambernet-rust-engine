mod resource;

use crate::engine::core::errors::AsnRenderError;
use crate::engine::core::math::{Pos2D, Size2D};
use crate::engine::core::winapi::scene::{TNodeBase, TNodeQuad, TNodeView2d};
use crate::engine::core::winapi::TAsnWinapi;
use crate::engine::core::winapi::{AsnTextureFormat, Mesh};
use crate::engine::winapi::defines;
use crate::engine::winapi::defines::{BytesArray, CellSize, SizeDimension};
use crate::engine::winapi::scene::node_view2d::resource::{
    Vertex, INDICES, SHADER_SOURCE, VERTICES,
};
use crate::engine::winapi::utils::ToWgpuFormat;
use crate::engine::winapi::wgpu::bind_groups::{BindGroupEntryBuilder, BindGroupLayoutBuilder};
use crate::engine::winapi::wgpu::texture::AsnTexture;
use crate::engine::winapi::wgpu::{AsnWgpuFrameContext, AsnWgpuWinApi};
use wgpu::{BindGroup, BindGroupLayout, Device, RenderPipeline, ShaderModule, TextureFormat};

pub struct AsnWgpuNodeView2d {
    tile_texture: AsnTexture,
    texture: AsnTexture,
    view: BytesArray,
    mesh: Mesh,
    bind_group: wgpu::BindGroup,
    render_pipeline: wgpu::RenderPipeline,
    is_need_update: bool,
    // rng: SmallRng,
    shader: ShaderModule,
}

fn create_node_view2d_set(
    gfx: &mut AsnWgpuWinApi,
    texture: &AsnTexture,
    tile_texture: &AsnTexture,
    texture_format: TextureFormat,
    shader: &ShaderModule,
) -> (RenderPipeline, BindGroup) {
    let group_layout_builder = BindGroupLayoutBuilder::new()
        .texture()
        .sampler()
        .texture()
        .sampler();
    let group_layout_desc = wgpu::BindGroupLayoutDescriptor {
        entries: group_layout_builder.entries(),
        label: Some("texture_bind_group_layout"),
    };
    let diffuse_bind_group_layout = gfx
        .get_device()
        .create_bind_group_layout(&group_layout_desc);
    let bind_group_layouts = &[&diffuse_bind_group_layout];
    let render_pipeline = get_render_pipeline(
        &gfx.get_device(),
        texture_format,
        &shader,
        bind_group_layouts,
    );

    let group_entry_builder = BindGroupEntryBuilder::default()
        .texture(&texture.view)
        .sampler(&texture.sampler)
        .texture(&tile_texture.view)
        .sampler(&tile_texture.sampler);

    let group_desc = wgpu::BindGroupDescriptor {
        layout: &diffuse_bind_group_layout,
        entries: group_entry_builder.entries(),
        label: Some("diffuse_bind_group"),
    };
    let bind_group = gfx.get_device().create_bind_group(&group_desc);

    (render_pipeline, bind_group)
}

impl AsnWgpuNodeView2d {
    pub fn new(gfx: &mut AsnWgpuWinApi) -> Self {
        let mesh = Mesh::build(bytemuck::cast_slice(VERTICES), INDICES, gfx.get_device());

        let shader = gfx
            .get_device()
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Shader"),
                source: wgpu::ShaderSource::Wgsl(SHADER_SOURCE.into()),
            });

        let texture_format = gfx.get_config().texture_format.to_wgpu_format();
        println!("texure format: {:?}", texture_format);

        let texture = AsnTexture::new(gfx);
        let tile_texture = AsnTexture::new(gfx);

        let (render_pipeline, bind_group) =
            create_node_view2d_set(gfx, &texture, &tile_texture, texture_format, &shader);
        let view_size_w: u32 = 1;
        let view_size_h: u32 = 1;

        let view = BytesArray {
            size: Size2D {
                width: view_size_w as SizeDimension,
                height: view_size_h as SizeDimension,
            },
            bytes: vec![0; (view_size_w * view_size_h * 4) as usize],
        };

        Self {
            tile_texture,
            shader,
            texture,
            render_pipeline,
            view,
            mesh,
            bind_group,
            is_need_update: false,
        }
    }
    fn draw_me(&mut self, fcx: &mut AsnWgpuFrameContext) {
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
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.mesh.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.mesh.num_indices, 0, 0..1);
    }
    fn update_me(&mut self, gfx: &mut AsnWgpuWinApi) {
        self.is_need_update = false;
        self.texture
            .update_from_array(gfx, &self.view)
            .expect("TODO: panic message");
    }
}

impl TNodeBase for AsnWgpuNodeView2d {
    type FrameContext = AsnWgpuFrameContext;
    type WinApi = AsnWgpuWinApi;
    type AsnTexture = AsnTexture;
    fn draw(&mut self, fcx: &mut Self::FrameContext) {
        self.draw_me(fcx);
    }

    fn update(&mut self, gfx: &mut Self::WinApi) {
        if !self.is_need_update {
            return;
        }
        self.update_me(gfx)
    }
}

impl TNodeView2d for AsnWgpuNodeView2d {
    type CellType = defines::CellSize;
    type SizeDimension = defines::SizeDimension;

    fn set_tile_texture(
        &mut self,
        gfx: &mut Self::WinApi,
        bytes: &[u8],
        f: AsnTextureFormat,
    ) -> Result<(), AsnRenderError> {
        println!("AsnWgpuNodeView2d set_texture");
        let tile_texture = AsnTexture::from_raw_image(gfx, bytes, f)?;

        let texture_format = gfx.get_config().texture_format.to_wgpu_format();
        let (render_pipeline, bind_group) = create_node_view2d_set(
            gfx,
            &self.texture,
            &tile_texture,
            texture_format,
            &self.shader,
        );

        self.tile_texture = tile_texture;
        self.render_pipeline = render_pipeline;
        self.bind_group = bind_group;
        Ok(())
    }

    fn set_view_size(&mut self, size: Size2D<Self::SizeDimension>) -> Result<(), AsnRenderError> {
        let view = BytesArray {
            size,
            bytes: vec![0; (size.width * size.height * 4) as usize],
        };
        self.view = view;
        self.is_need_update = true;
        Ok(())
    }

    fn set_cell(
        &mut self,
        pos: &Pos2D<Self::SizeDimension>,
        c: CellSize,
    ) -> Result<(), AsnRenderError> {
        self.view.set_point(pos, c).unwrap();
        self.is_need_update = true;
        Ok(())
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
        buffers: &[Vertex::desc()],
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
